use crate::{
    BlockyAnimation, InterpolationType, Keyframe, NodeAnimation, Quatf, Vec2f, Vec3f,
    BLOCKYANIM_FPS,
};

/// Values sampled from a node animation at a point in time.
///
/// Each field is optional because a node may animate only some channels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SampledNodeAnimation {
    pub position: Option<Vec3f>,
    pub orientation: Option<Quatf>,
    pub shape_stretch: Option<Vec3f>,
    pub shape_visible: Option<bool>,
    pub shape_uv_offset: Option<Vec2f>,
}

impl BlockyAnimation {
    /// Samples one named node at `seconds`.
    ///
    /// This method is duration-aware. When `holdLastKeyframe == false`, tracks are sampled as
    /// loops and missing frame-0 keyframes interpolate across the animation boundary from the
    /// final keyframe back to the first one. This matches how looping DCC/game animations are
    /// usually expected to behave and avoids a visible snap near frame 0.
    pub fn sample_node_seconds(&self, node_name: &str, seconds: f32) -> Option<SampledNodeAnimation> {
        let frames = Self::seconds_to_frames(seconds);
        self.sample_node_frames(node_name, frames)
    }

    /// Samples one named node at `.blockyanim` frame time.
    ///
    /// `.blockyanim` stores time in frames at 60 FPS.
    pub fn sample_node_frames(&self, node_name: &str, frames: f32) -> Option<SampledNodeAnimation> {
        let node = self.node_animation(node_name)?;
        let duration = self.duration.max(0.0);
        let looping = !self.hold_last_keyframe && duration > 0.0;

        let frames = if duration <= 0.0 {
            0.0
        } else if self.hold_last_keyframe {
            frames.max(0.0).min(duration)
        } else {
            frames.rem_euclid(duration)
        };

        Some(node.sample_frames_with_duration(frames, duration, looping))
    }
}

impl NodeAnimation {
    /// Samples without duration information.
    ///
    /// Kept for simple/offline uses. For runtime playback of a full `.blockyanim`, prefer
    /// `BlockyAnimation::sample_node_seconds` or `BlockyAnimation::sample_node_frames`, because
    /// those methods know whether the animation loops and can interpolate across the loop seam.
    pub fn sample_seconds(&self, seconds: f32) -> SampledNodeAnimation {
        let frames = seconds * BLOCKYANIM_FPS;
        self.sample_frames(frames)
    }

    /// Samples without loop-boundary interpolation.
    pub fn sample_frames(&self, frames: f32) -> SampledNodeAnimation {
        self.sample_frames_with_duration(frames, 0.0, false)
    }

    /// Samples with explicit duration/looping information.
    pub fn sample_seconds_with_duration(
        &self,
        seconds: f32,
        duration_seconds: f32,
        looping: bool,
    ) -> SampledNodeAnimation {
        self.sample_frames_with_duration(
            seconds * BLOCKYANIM_FPS,
            duration_seconds * BLOCKYANIM_FPS,
            looping,
        )
    }

    /// Samples with explicit duration/looping information, in `.blockyanim` frame units.
    pub fn sample_frames_with_duration(
        &self,
        frames: f32,
        duration_frames: f32,
        looping: bool,
    ) -> SampledNodeAnimation {
        SampledNodeAnimation {
            position: sample_vec3_track(&self.position, frames, duration_frames, looping),
            orientation: sample_quat_track(&self.orientation, frames, duration_frames, looping),
            shape_stretch: sample_vec3_track(&self.shape_stretch, frames, duration_frames, looping),
            shape_visible: sample_bool_track(&self.shape_visible, frames, duration_frames, looping),
            shape_uv_offset: sample_vec2_track(&self.shape_uv_offset, frames, duration_frames, looping),
        }
    }
}

fn interpolation_t(interpolation: Option<InterpolationType>, t: f32) -> f32 {
    match interpolation.unwrap_or(InterpolationType::Linear) {
        // The exporter marks these tracks as smooth. This cheap smoothstep is stable and avoids
        // pulling in a full curve implementation. If you need exact DCC playback, replace this
        // with the curve used by your editor/exporter.
        InterpolationType::Smooth => t * t * (3.0 - 2.0 * t),
        InterpolationType::Linear | InterpolationType::Unknown => t,
    }
}

fn surrounding<T>(
    track: &[Keyframe<T>],
    frames: f32,
    duration_frames: f32,
    looping: bool,
) -> Option<(&Keyframe<T>, Option<&Keyframe<T>>, f32)> {
    if track.is_empty() {
        return None;
    }

    if track.len() == 1 {
        return Some((&track[0], None, 0.0));
    }

    let mut before: Option<&Keyframe<T>> = None;
    let mut after: Option<&Keyframe<T>> = None;
    let mut first: Option<&Keyframe<T>> = None;
    let mut last: Option<&Keyframe<T>> = None;

    for keyframe in track {
        match first {
            Some(current) if keyframe.time >= current.time => {}
            _ => first = Some(keyframe),
        }

        match last {
            Some(current) if keyframe.time <= current.time => {}
            _ => last = Some(keyframe),
        }

        if keyframe.time <= frames {
            match before {
                Some(prev) if keyframe.time <= prev.time => {}
                _ => before = Some(keyframe),
            }
        } else {
            match after {
                Some(next) if keyframe.time >= next.time => {}
                _ => after = Some(keyframe),
            }
        }
    }

    match (before, after) {
        (Some(a), Some(b)) => Some((a, Some(b), frames)),
        (Some(a), None) if looping && duration_frames > 0.0 => {
            let b = first?;
            Some((a, Some(b), frames))
        }
        (None, Some(b)) if looping && duration_frames > 0.0 => {
            let a = last?;
            Some((a, Some(b), frames + duration_frames))
        }
        (Some(a), None) => Some((a, None, frames)),
        (None, Some(b)) => Some((b, None, frames)),
        (None, None) => None,
    }
}

fn span_and_t<T>(a: &Keyframe<T>, b: &Keyframe<T>, frames: f32, duration_frames: f32) -> f32 {
    let a_time = a.time;
    let mut b_time = b.time;
    if duration_frames > 0.0 && b_time <= a_time {
        b_time += duration_frames;
    }

    let span = b_time - a_time;
    if span.abs() <= f32::EPSILON {
        1.0
    } else {
        ((frames - a_time) / span).clamp(0.0, 1.0)
    }
}

fn sample_vec3_track(
    track: &[Keyframe<Vec3f>],
    frames: f32,
    duration_frames: f32,
    looping: bool,
) -> Option<Vec3f> {
    let (a, b, frames) = surrounding(track, frames, duration_frames, looping)?;
    match b {
        None => Some(a.delta),
        Some(b) => {
            let t = span_and_t(a, b, frames, duration_frames);
            let t = interpolation_t(a.interpolation_type, t);
            Some(a.delta.lerp(b.delta, t))
        }
    }
}

fn sample_vec2_track(
    track: &[Keyframe<Vec2f>],
    frames: f32,
    duration_frames: f32,
    looping: bool,
) -> Option<Vec2f> {
    let (a, b, frames) = surrounding(track, frames, duration_frames, looping)?;
    match b {
        None => Some(a.delta),
        Some(b) => {
            let t = span_and_t(a, b, frames, duration_frames);
            let t = interpolation_t(a.interpolation_type, t);
            Some(a.delta.lerp(b.delta, t))
        }
    }
}

fn sample_quat_track(
    track: &[Keyframe<Quatf>],
    frames: f32,
    duration_frames: f32,
    looping: bool,
) -> Option<Quatf> {
    let (a, b, frames) = surrounding(track, frames, duration_frames, looping)?;
    match b {
        None => Some(a.delta.normalized()),
        Some(b) => {
            let t = span_and_t(a, b, frames, duration_frames);
            let t = interpolation_t(a.interpolation_type, t);
            Some(a.delta.slerp(b.delta, t))
        }
    }
}

fn sample_bool_track(
    track: &[Keyframe<bool>],
    frames: f32,
    duration_frames: f32,
    looping: bool,
) -> Option<bool> {
    if track.is_empty() {
        return None;
    }

    let mut before: Option<&Keyframe<bool>> = None;
    let mut first: Option<&Keyframe<bool>> = None;
    let mut last: Option<&Keyframe<bool>> = None;

    for keyframe in track {
        match first {
            Some(current) if keyframe.time >= current.time => {}
            _ => first = Some(keyframe),
        }
        match last {
            Some(current) if keyframe.time <= current.time => {}
            _ => last = Some(keyframe),
        }
        if keyframe.time <= frames {
            match before {
                Some(prev) if keyframe.time <= prev.time => {}
                _ => before = Some(keyframe),
            }
        }
    }

    if let Some(k) = before {
        return Some(k.delta);
    }

    if looping && duration_frames > 0.0 {
        return last.map(|k| k.delta);
    }

    first.map(|k| k.delta)
}
