use serde::{Deserialize, Serialize};

/// A JSON `{x, y}` vector.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vec2f {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Vec2f {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        Self {
            x: self.x + (rhs.x - self.x) * t,
            y: self.y + (rhs.y - self.y) * t,
        }
    }
}

/// A JSON `{x, y, z}` vector.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vec3f {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

impl Vec3f {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        Self {
            x: self.x + (rhs.x - self.x) * t,
            y: self.y + (rhs.y - self.y) * t,
            z: self.z + (rhs.z - self.z) * t,
        }
    }
}

/// A JSON quaternion `{x, y, z, w}`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quatf {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
    #[serde(default = "default_quat_w")]
    pub w: f32,
}

fn default_quat_w() -> f32 {
    1.0
}

impl Default for Quatf {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quatf {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn normalized(self) -> Self {
        let len_sq = self.length_squared();
        if len_sq <= f32::EPSILON {
            return Self::IDENTITY;
        }
        let inv_len = len_sq.sqrt().recip();
        Self {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
            w: self.w * inv_len,
        }
    }

    pub fn negated(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        Self {
            x: self.x + (rhs.x - self.x) * t,
            y: self.y + (rhs.y - self.y) * t,
            z: self.z + (rhs.z - self.z) * t,
            w: self.w + (rhs.w - self.w) * t,
        }
        .normalized()
    }

    /// Spherical interpolation. This is suitable for `.blockyanim` orientation tracks.
    pub fn slerp(self, mut rhs: Self, t: f32) -> Self {
        let lhs = self.normalized();
        rhs = rhs.normalized();

        let mut cos_theta = lhs.dot(rhs);
        if cos_theta < 0.0 {
            rhs = rhs.negated();
            cos_theta = -cos_theta;
        }

        // Fall back to normalized lerp for very close rotations.
        if cos_theta > 0.9995 {
            return lhs.lerp(rhs, t);
        }

        let theta = cos_theta.clamp(-1.0, 1.0).acos();
        let sin_theta = theta.sin();
        if sin_theta.abs() <= f32::EPSILON {
            return lhs;
        }

        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        Self {
            x: lhs.x * a + rhs.x * b,
            y: lhs.y * a + rhs.y * b,
            z: lhs.z * a + rhs.z * b,
            w: lhs.w * a + rhs.w * b,
        }
        .normalized()
    }
}

#[cfg(feature = "glam")]
impl From<Vec2f> for glam::Vec2 {
    fn from(v: Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

#[cfg(feature = "glam")]
impl From<glam::Vec2> for Vec2f {
    fn from(v: glam::Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

#[cfg(feature = "glam")]
impl From<Vec3f> for glam::Vec3 {
    fn from(v: Vec3f) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "glam")]
impl From<glam::Vec3> for Vec3f {
    fn from(v: glam::Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "glam")]
impl From<Quatf> for glam::Quat {
    fn from(q: Quatf) -> Self {
        Self::from_xyzw(q.x, q.y, q.z, q.w).normalize_or_identity()
    }
}

#[cfg(feature = "glam")]
impl From<glam::Quat> for Quatf {
    fn from(q: glam::Quat) -> Self {
        Self::new(q.x, q.y, q.z, q.w)
    }
}
