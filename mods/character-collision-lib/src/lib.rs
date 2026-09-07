//! Engine-independent kinematic capsule against oriented boxes.
use bevy::prelude::*;
use collision_api::*;

fn endpoints(q: CharacterQuery) -> (Vec3, Vec3, f32) {
    let r = q.radius.min(q.height * 0.5);
    (
        q.position + q.up * r,
        q.position + q.up * (q.height - r),
        r,
    )
}

fn bounds(q: CharacterQuery, extra: f32) -> Aabb {
    let (a, b, r) = endpoints(q);
    let padding = Vec3::splat(r + extra);
    Aabb {
        min: a
            .min(b)
            .min(a + q.displacement)
            .min(b + q.displacement)
            - padding,
        max: a
            .max(b)
            .max(a + q.displacement)
            .max(b + q.displacement)
            + padding,
    }
}

/// Exact closest point on a segment to an AABB: squared distance is piecewise quadratic.
fn segment_box(a: Vec3, b: Vec3, h: Vec3) -> (Vec3, Vec3) {
    let d = b - a;
    let mut breaks = vec![0.0, 1.0];

    for axis in 0..3 {
        if d[axis].abs() > 1e-12 {
            for side in [-h[axis], h[axis]] {
                let t = (side - a[axis]) / d[axis];
                if t > 0.0 && t < 1.0 {
                    breaks.push(t);
                }
            }
        }
    }

    breaks.sort_by(f32::total_cmp);

    let mut best = (a, a.clamp(-h, h));
    let mut distance = f32::INFINITY;

    for interval in breaks.windows(2) {
        let mid = (interval[0] + interval[1]) * 0.5;
        let (mut numerator, mut denominator) = (0.0, 0.0);

        for axis in 0..3 {
            let v = a[axis] + d[axis] * mid;
            let side = if v < -h[axis] {
                -h[axis]
            } else if v > h[axis] {
                h[axis]
            } else {
                continue;
            };
            numerator += d[axis] * (a[axis] - side);
            denominator += d[axis] * d[axis];
        }

        let t = if denominator > 0.0 {
            (-numerator / denominator).clamp(interval[0], interval[1])
        } else {
            mid
        };

        let point = a + d * t;
        let on_box = point.clamp(-h, h);
        let sq = point.distance_squared(on_box);

        if sq < distance {
            distance = sq;
            best = (point, on_box);
        }
    }

    best
}

#[derive(Debug, Clone, Copy)]
struct ContactDetail {
    separation: f32,
    contact: CharacterContact,
    axis_point_local: Vec3,
    box_point_local: Vec3,
}

fn contact_detail(q: CharacterQuery, b: CollisionBox) -> ContactDetail {
    let (a, z, r) = endpoints(q);
    let inv = b.rotation.conjugate();
    let a = inv * (a - b.center);
    let z = inv * (z - b.center);
    let (p, c) = segment_box(a, z, b.half_extents);
    let delta = p - c;
    let length = delta.length();

    let (separation, normal, point) = if length > 1e-8 {
        (length - r, delta / length, c)
    } else {
        // Capsule axis intersects the box: use the smallest full separating translation.
        let mut depth = f32::INFINITY;
        let mut normal = Vec3::X;

        for axis in 0..3 {
            let positive = b.half_extents[axis] - a[axis].min(z[axis]) + r;
            let negative = b.half_extents[axis] + a[axis].max(z[axis]) + r;

            if positive < depth {
                depth = positive;
                normal = Vec3::ZERO;
                normal[axis] = 1.0;
            }
            if negative < depth {
                depth = negative;
                normal = Vec3::ZERO;
                normal[axis] = -1.0;
            }
        }

        (-depth, normal, c)
    };

    ContactDetail {
        separation,
        contact: CharacterContact {
            normal: b.rotation * normal,
            point: b.center + b.rotation * point,
            surface: b.surface,
        },
        axis_point_local: p,
        box_point_local: c,
    }
}

fn contact(q: CharacterQuery, b: CollisionBox) -> (f32, CharacterContact) {
    let detail = contact_detail(q, b);
    (detail.separation, detail.contact)
}

/// Returns the actual box face supporting the character.
///
/// Capsule collision normals around a convex box edge are diagonal because the
/// bottom hemisphere is touching an edge/corner. Those normals are useful for
/// collision response, but they must not turn the edge into a walkable floor:
/// doing so makes ground snapping roll the character down the rounded capsule.
///
/// A ground/support contact is therefore accepted only when the closest point
/// lies on the interior of exactly one box face. Edges and corners still take
/// part in normal collision resolution through `contact`/`sweep`.
fn walkable_face(
    q: CharacterQuery,
    b: CollisionBox,
    detail: ContactDetail,
) -> Option<CharacterContact> {
    let p = detail.axis_point_local;
    let h = b.half_extents;

    let mut face_axis = None;
    let mut face_sign = 0.0;

    for axis in 0..3 {
        let sign = if p[axis] > h[axis] {
            1.0
        } else if p[axis] < -h[axis] {
            -1.0
        } else {
            continue;
        };

        // More than one outside axis means the closest box feature is an edge
        // or corner, not a face interior.
        if face_axis.is_some() {
            return None;
        }

        face_axis = Some(axis);
        face_sign = sign;
    }

    let axis = face_axis?;

    let mut local_normal = Vec3::ZERO;
    local_normal[axis] = face_sign;
    let normal = (b.rotation * local_normal).normalize_or_zero();

    if normal.dot(q.up) < q.slope_cosine {
        return None;
    }

    Some(CharacterContact {
        normal,
        point: b.center + b.rotation * detail.box_point_local,
        surface: b.surface,
    })
}

fn sweep(q: CharacterQuery, boxes: &[CollisionBox]) -> Option<(f32, CharacterContact)> {
    let length = q.displacement.length();
    if length < 1e-10 {
        return None;
    }

    let mut best = None;
    let mut best_time = 1.0;

    for b in boxes {
        let mut t = 0.0;

        for _ in 0..64 {
            let (gap, hit) = contact(
                CharacterQuery {
                    position: q.position + q.displacement * t,
                    ..q
                },
                *b,
            );

            if gap <= q.skin() * 1.1 {
                if q.displacement.dot(hit.normal) < -1e-8 && t <= best_time {
                    best_time = t;
                    best = Some((t, hit));
                }
                break;
            }

            // Distance is 1-Lipschitz under translation, so this cannot skip a thin shape.
            t += (gap - q.skin()) / length;

            if t > best_time || t > 1.0 {
                break;
            }
        }
    }

    best
}

/// Sweeps only against real walkable box faces.
///
/// This is deliberately stricter than `sweep`: rounded capsule contact with a
/// convex edge/corner is collision, but never support.
fn support_sweep(q: CharacterQuery, boxes: &[CollisionBox]) -> Option<(f32, CharacterContact)> {
    let length = q.displacement.length();
    if length < 1e-10 {
        return None;
    }

    let mut best = None;
    let mut best_time = 1.0;

    for b in boxes {
        let mut t = 0.0;

        for _ in 0..64 {
            let at_t = CharacterQuery {
                position: q.position + q.displacement * t,
                ..q
            };
            let detail = contact_detail(at_t, *b);

            if detail.separation <= q.skin() * 1.1 {
                if let Some(face) = walkable_face(at_t, *b, detail) {
                    if q.displacement.dot(face.normal) < -1e-8 && t <= best_time {
                        best_time = t;
                        best = Some((t, face));
                    }
                }
                break;
            }

            t += (detail.separation - q.skin()) / length;

            if t > best_time || t > 1.0 {
                break;
            }
        }
    }

    best
}

fn slide(mut q: CharacterQuery, boxes: &[CollisionBox]) -> CharacterResult {
    let mut contacts = Vec::new();

    for _ in 0..8 {
        let deepest = boxes
            .iter()
            .map(|b| contact(q, *b))
            .filter(|(d, _)| *d < 0.0)
            .min_by(|a, b| a.0.total_cmp(&b.0));

        let Some((gap, hit)) = deepest else {
            break;
        };

        q.position += hit.normal * (-gap + q.skin());
        contacts.push(hit);
    }

    for _ in 0..8 {
        let Some((t, hit)) = sweep(q, boxes) else {
            q.position += q.displacement;
            break;
        };

        q.position += q.displacement * t;
        q.displacement *= 1.0 - t;
        contacts.push(hit);

        let slope = hit.normal.dot(q.up);
        if slope > 0.0 && slope < q.slope_cosine {
            // Treat a non-walkable slope as a wall in the gravity plane. Removing
            // upward motion AFTER projecting onto the real surface points back into
            // that surface, causing repeated zero-time hits until the solver stalls.
            let wall = (hit.normal - q.up * slope).normalize_or_zero();
            contacts.push(CharacterContact {
                normal: wall,
                ..hit
            });
        }

        // Clip against every accumulated plane, not independent global axis components.
        for _ in 0..3 {
            for c in &contacts {
                let into = q.displacement.dot(c.normal);
                if into < 0.0 {
                    q.displacement -= c.normal * into;
                }
            }
        }

        if q.displacement.length_squared() < q.skin() * q.skin() {
            break;
        }
    }

    CharacterResult {
        position: q.position,
        contacts,
        support: None,
    }
}

pub fn support(
    q: CharacterQuery,
    geometry: &impl CharacterGeometry,
) -> Option<CharacterContact> {
    let probe = CharacterQuery {
        displacement: -q.up * q.ground_probe,
        ..q
    };
    let boxes = geometry.query(bounds(probe, q.skin()));
    support_sweep(probe, &boxes).map(|(_, hit)| hit)
}

pub fn resolve(q: CharacterQuery, geometry: &impl CharacterGeometry) -> CharacterResult {
    if !q.position.is_finite()
        || !q.displacement.is_finite()
        || q.up.length_squared() < 0.9
        || q.radius <= 0.0
        || q.height <= 0.0
    {
        return CharacterResult {
            position: q.position,
            contacts: Vec::new(),
            support: None,
        };
    }

    let mut area = bounds(q, q.skin());
    let raised = bounds(
        CharacterQuery {
            position: q.position + q.up * q.step_height,
            ..q
        },
        q.skin(),
    );
    let lowered = bounds(
        CharacterQuery {
            position: q.position - q.up * q.ground_probe,
            ..q
        },
        q.skin(),
    );

    area.min = area.min.min(raised.min).min(lowered.min);
    area.max = area.max.max(raised.max).max(lowered.max);

    let boxes = geometry
        .query(area)
        .into_iter()
        .filter(|b| q.ignore_surface != Some(b.surface))
        .collect::<Vec<_>>();

    let mut result = slide(q, &boxes);
    let planar = q.displacement - q.up * q.displacement.dot(q.up);

    if q.was_grounded
        && q.step_height > 0.0
        && q.displacement.dot(q.up) <= q.skin()
        && (result.position - q.position).dot(planar)
            < planar.length_squared() - q.skin() * q.skin()
    {
        let raised = CharacterQuery {
            displacement: q.up * q.step_height,
            ..q
        };

        if sweep(raised, &boxes).is_none() {
            let across = slide(
                CharacterQuery {
                    position: q.position + q.up * q.step_height,
                    displacement: planar,
                    ..q
                },
                &boxes,
            );

            let down = CharacterQuery {
                position: across.position,
                displacement: -q.up * (q.step_height + q.ground_probe),
                ..q
            };

            if let Some((t, hit)) = support_sweep(down, &boxes) {
                let landed = down.position + down.displacement * t;

                if (landed - q.position).dot(planar)
                    > (result.position - q.position).dot(planar) + q.skin() * q.skin()
                {
                    result = CharacterResult {
                        position: landed,
                        contacts: across.contacts,
                        support: Some(hit),
                    };
                }
            }
        }
    }

    // Uphill walking has positive gravity-up velocity but remains tangent to
    // its support. A jump separates along the actual support normal instead.
    let leaving_support = support_sweep(CharacterQuery {
        displacement: -q.up * q.ground_probe, ..q
    }, &boxes).is_some_and(|(_, hit)| q.displacement.dot(hit.normal) > q.skin());
    if !leaving_support && (q.was_grounded || q.displacement.dot(q.up) <= q.skin()) {
        let probe = CharacterQuery {
            position: result.position,
            displacement: -q.up * q.ground_probe,
            ..q
        };

        if let Some((t, hit)) = support_sweep(probe, &boxes) {
            result.support = Some(hit);

            if q.was_grounded {
                result.position += probe.displacement * t;
            }
        }
    }

    if let Some(hit) = result.support {
        result.contacts.push(hit);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scene(Vec<CollisionBox>);

    impl CharacterGeometry for Scene {
        fn query(&self, _: Aabb) -> Vec<CollisionBox> {
            self.0.clone()
        }
    }

    fn box_at(center: Vec3, half_extents: Vec3) -> CollisionBox {
        CollisionBox {
            center,
            half_extents,
            rotation: Quat::IDENTITY,
            surface: 0,
        }
    }

    #[test]
    fn tilted_steep_contact_preserves_sideways_motion_with_rotated_gravity() {
        let gravity_rotation = Quat::from_rotation_x(0.8);
        let rotation = gravity_rotation * Quat::from_rotation_z(1.1);
        let up = gravity_rotation * Vec3::Y;
        let normal = rotation * Vec3::Y;
        let obstacle = CollisionBox {
            center: Vec3::ZERO,
            rotation,
            half_extents: Vec3::new(5.0, 0.2, 5.0),
            surface: 9,
        };
        let scene = Scene(vec![obstacle]);

        let position = normal * (0.2 + 0.15 - 0.0001) - up * 0.15;
        let sideways = gravity_rotation * Vec3::Z;
        let motion = sideways * 0.2 - up * 0.1;
        let q = CharacterQuery::new(position, motion, up, 0.15, 0.9);
        let result = resolve(q, &scene);

        assert!((result.position - position).dot(sideways) > 0.19);
        assert!(contact(CharacterQuery { position: result.position, ..q }, obstacle).0 >= -q.skin());
    }

    #[test]
    fn fast_motion_does_not_skip_thin_model_elements() {
        let scene = Scene(vec![box_at(
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.01, 2.0, 2.0),
        )]);
        let result = resolve(
            CharacterQuery::new(Vec3::ZERO, Vec3::X * 10.0, Vec3::Y, 0.1, 0.4),
            &scene,
        );

        assert!(result.position.x < 0.9);
        assert!(!result.contacts.is_empty());
    }

    #[test]
    fn scaled_character_passes_under_low_model() {
        let scene = Scene(vec![
            box_at(Vec3::new(0.0, -0.5, 0.0), Vec3::new(5.0, 0.5, 2.0)),
            box_at(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.5, 2.0)),
        ]);
        let small = resolve(
            CharacterQuery::new(
                Vec3::new(-1.0, 0.001, 0.0),
                Vec3::X * 2.0,
                Vec3::Y,
                0.06,
                0.36,
            ),
            &scene,
        );
        let large = resolve(
            CharacterQuery::new(
                Vec3::new(-1.0, 0.001, 0.0),
                Vec3::X * 2.0,
                Vec3::Y,
                0.3,
                1.8,
            ),
            &scene,
        );

        assert!(small.position.x > 0.9);
        assert!(large.position.x < 0.0);
    }

    #[test]
    fn supported_character_steps_up_without_global_axis_resolution() {
        for rot in [Quat::IDENTITY, Quat::from_rotation_x(1.1)] {
            let mut floor =
                box_at(Vec3::new(0.0, -0.5, 0.0), Vec3::new(5.0, 0.5, 2.0));
            let mut step =
                box_at(Vec3::new(0.5, 0.2, 0.0), Vec3::new(0.5, 0.2, 2.0));

            for b in [&mut floor, &mut step] {
                b.center = rot * b.center;
                b.rotation = rot;
            }

            let scene = Scene(vec![floor, step]);
            let mut q = CharacterQuery::new(
                rot * Vec3::new(-0.7, 0.001, 0.0),
                rot * Vec3::X * 1.3,
                rot * Vec3::Y,
                0.3,
                1.8,
            );
            q.was_grounded = true;

            let result = resolve(q, &scene);

            assert!(result.position.dot(rot * Vec3::X) > 0.4);
            assert!(result.position.dot(rot * Vec3::Y) > 0.39);
        }
    }

    #[test]
    fn steep_surface_is_not_walkable() {
        let rotation = Quat::from_rotation_z(1.1);
        let scene = Scene(vec![CollisionBox {
            center: Vec3::ZERO,
            rotation,
            half_extents: Vec3::new(4.0, 0.2, 4.0),
            surface: 7,
        }]);
        let result = resolve(
            CharacterQuery::new(
                Vec3::Y * 3.0,
                Vec3::NEG_Y * 4.0,
                Vec3::Y,
                0.15,
                0.9,
            ),
            &scene,
        );

        assert!(result.support.is_none());
    }

    #[test]
    fn upward_jump_does_not_snap_back_to_floor() {
        let scene = Scene(vec![box_at(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(5.0, 0.5, 5.0),
        )]);
        let result = resolve(
            CharacterQuery::new(Vec3::Y * 0.001, Vec3::Y * 0.2, Vec3::Y, 0.3, 1.8),
            &scene,
        );

        assert!(result.support.is_none());
        assert!(result.position.y > 0.19);
    }

    #[test]
    fn support_normal_distinguishes_uphill_walking_from_short_jump() {
        for gravity_rotation in [Quat::IDENTITY, Quat::from_euler(EulerRot::XYZ, 0.7, -0.4, 0.6)] {
            let rotation = gravity_rotation * Quat::from_rotation_z(0.35);
            let up = gravity_rotation * Vec3::Y;
            let normal = rotation * Vec3::Y;
            let scene = Scene(vec![CollisionBox {
                center: Vec3::ZERO, rotation,
                half_extents: Vec3::new(5.0, 0.2, 5.0), surface: 7,
            }]);
            let start = normal * (0.2 + 0.3 + 0.0006) - up * 0.3;
            let mut walking = CharacterQuery::new(start, rotation * Vec3::X * 0.02, up, 0.3, 1.8);
            walking.was_grounded = true;
            assert!(resolve(walking, &scene).support.is_some());
            let jumping = CharacterQuery { displacement: up * 0.01, ..walking };
            let result = resolve(jumping, &scene);
            assert!(result.support.is_none());
            assert!((result.position - start).dot(up) > 0.009);
        }
    }

    #[test]
    fn gravity_rotation_preserves_floor_contact() {
        for rotation in [
            Quat::IDENTITY,
            Quat::from_rotation_z(1.2),
            Quat::from_rotation_x(2.0),
        ] {
            let scene = Scene(vec![CollisionBox {
                center: rotation * Vec3::new(0.0, -0.5, 0.0),
                rotation,
                half_extents: Vec3::new(5.0, 0.5, 5.0),
                surface: 0,
            }]);
            let q = CharacterQuery::new(
                rotation * Vec3::Y,
                rotation * Vec3::new(0.2, -2.0, 0.0),
                rotation * Vec3::Y,
                0.3,
                1.8,
            );
            let result = resolve(q, &scene);

            assert!(result.support.is_some());
            assert!((result.position.dot(rotation * Vec3::Y)).abs() < 0.005);
        }
    }

    #[test]
    fn convex_edge_is_collision_but_not_ground_support() {
        let scene = Scene(vec![box_at(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(1.0, 0.5, 1.0),
        )]);

        let mut on_face =
            CharacterQuery::new(Vec3::new(0.99, 0.001, 0.0), Vec3::ZERO, Vec3::Y, 0.3, 1.8);
        on_face.ground_probe = 0.05;

        let mut beyond_edge =
            CharacterQuery::new(Vec3::new(1.10, 0.001, 0.0), Vec3::ZERO, Vec3::Y, 0.3, 1.8);
        beyond_edge.ground_probe = 0.05;

        assert!(support(on_face, &scene).is_some());
        assert!(support(beyond_edge, &scene).is_none());
    }

    #[test]
    fn grounded_character_does_not_snap_down_around_convex_edge() {
        let scene = Scene(vec![box_at(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(1.0, 0.5, 1.0),
        )]);

        let start = Vec3::new(1.10, 0.001, 0.0);
        let mut q = CharacterQuery::new(start, Vec3::ZERO, Vec3::Y, 0.3, 1.8);
        q.was_grounded = true;
        q.ground_probe = 0.05;

        let result = resolve(q, &scene);

        assert!((result.position.y - start.y).abs() < 1e-6);
        assert!(result.support.is_none());
    }
}
