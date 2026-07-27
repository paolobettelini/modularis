use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrassInteractionSource {
    /// Center of the interaction capsule in world space.
    pub position: Vec3,
    /// Capsule axis. It does not need to be normalized by callers.
    pub axis: Vec3,
    /// Half the length of the capsule's center segment.
    pub half_length: f32,
    pub radius: f32,
    pub strength: f32,
}

impl GrassInteractionSource {
    pub fn is_valid(self) -> bool {
        self.position.is_finite()
            && self.axis.is_finite()
            && self.axis.length_squared() > f32::EPSILON
            && self.half_length.is_finite()
            && self.half_length >= 0.0
            && self.radius.is_finite()
            && self.strength.is_finite()
            && self.radius > 0.0
            && self.strength >= 0.0
    }
}

/// Shared presentation field. Feature mods own named sources; renderers decide
/// how many sources they can process efficiently.
#[derive(Resource, Debug, Clone, Default)]
pub struct ClientGrassInteractionField {
    sources: BTreeMap<String, GrassInteractionSource>,
}

impl ClientGrassInteractionField {
    pub fn set(&mut self, owner: impl Into<String>, source: GrassInteractionSource) {
        if source.is_valid() {
            self.sources.insert(owner.into(), source);
        }
    }

    pub fn remove(&mut self, owner: &str) {
        self.sources.remove(owner);
    }

    pub fn sources(&self) -> impl Iterator<Item = (&str, GrassInteractionSource)> + '_ {
        self.sources
            .iter()
            .map(|(owner, source)| (owner.as_str(), *source))
    }

    pub fn sources_nearest_to(
        &self,
        position: Vec3,
        limit: usize,
    ) -> Vec<(&str, GrassInteractionSource)> {
        let mut sources = self.sources().collect::<Vec<_>>();
        sources.sort_by(|(left_owner, left), (right_owner, right)| {
            left.position
                .distance_squared(position)
                .total_cmp(&right.position.distance_squared(position))
                .then_with(|| left_owner.cmp(right_owner))
        });
        sources.truncate(limit);
        sources
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrassInteractionCollectSet;

pub trait ClientGrassInteractionApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(position: Vec3) -> GrassInteractionSource {
        GrassInteractionSource {
            position,
            axis: Vec3::Y,
            half_length: 0.9,
            radius: 0.5,
            strength: 1.0,
        }
    }

    #[test]
    fn rejects_invalid_capsules_and_selects_nearest_sources() {
        let mut field = ClientGrassInteractionField::default();
        field.set("far", source(Vec3::splat(10.0)));
        field.set("near", source(Vec3::X));
        field.set(
            "invalid",
            GrassInteractionSource {
                axis: Vec3::ZERO,
                ..source(Vec3::ZERO)
            },
        );

        let selected = field.sources_nearest_to(Vec3::ZERO, 1);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].0, "near");
        assert_eq!(field.sources().count(), 2);
    }
}
