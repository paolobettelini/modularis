use std::collections::{BTreeMap, BTreeSet};

use crate::{Axis, Direction, Element, Error, ResolvedModel, ResourceLocation, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct BakedQuad {
    pub positions: [[f32; 3]; 4],
    pub uvs: [[f32; 2]; 4],
    pub normal: [f32; 3],
    pub texture: ResourceLocation,
    pub cull_face: Option<Direction>,
    pub tint_index: Option<i32>,
    pub shade: bool,
    pub light_emission: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BakedMeshPart {
    pub texture: ResourceLocation,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub tint_indices: Vec<i32>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct BakeOptions {
    pub normalize_coordinates: bool,
    pub missing_texture: ResourceLocation,
    pub generated_item_depth: f32,
}

impl Default for BakeOptions {
    fn default() -> Self {
        Self {
            normalize_coordinates: true,
            missing_texture: ResourceLocation::parse("minecraft:missingno")
                .expect("hardcoded resource location is valid"),
            generated_item_depth: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelTransform {
    pub x_degrees: i32,
    pub y_degrees: i32,
    pub uv_lock: bool,
}

impl Default for ModelTransform {
    fn default() -> Self {
        Self {
            x_degrees: 0,
            y_degrees: 0,
            uv_lock: false,
        }
    }
}

pub fn bake_model(model: &ResolvedModel, options: &BakeOptions) -> Result<Vec<BakedQuad>> {
    bake_model_with_transform(model, options, ModelTransform::default())
}

pub fn bake_model_with_transform(
    model: &ResolvedModel,
    options: &BakeOptions,
    transform: ModelTransform,
) -> Result<Vec<BakedQuad>> {
    let mut quads = if model.elements.is_empty() && model.generated_item {
        bake_generated_item(model, options)?
    } else {
        let mut output = Vec::new();
        for element in &model.elements {
            output.extend(bake_element(model, element, options)?);
        }
        output
    };

    for quad in &mut quads {
        apply_block_transform(quad, transform);
        if options.normalize_coordinates {
            for position in &mut quad.positions {
                for coordinate in position {
                    *coordinate /= 16.0;
                }
            }
        }
    }
    Ok(quads)
}

pub fn group_quads_by_texture(quads: &[BakedQuad]) -> Vec<BakedMeshPart> {
    let mut groups: BTreeMap<ResourceLocation, BakedMeshPart> = BTreeMap::new();
    for quad in quads {
        let part = groups
            .entry(quad.texture.clone())
            .or_insert_with(|| BakedMeshPart {
                texture: quad.texture.clone(),
                positions: Vec::new(),
                normals: Vec::new(),
                uvs: Vec::new(),
                tint_indices: Vec::new(),
                indices: Vec::new(),
            });
        let base = part.positions.len() as u32;
        part.positions.extend(quad.positions);
        part.normals.extend([quad.normal; 4]);
        part.uvs.extend(quad.uvs);
        part.tint_indices.extend([quad.tint_index.unwrap_or(-1); 4]);
        part.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    groups.into_values().collect()
}

fn bake_element(
    model: &ResolvedModel,
    element: &Element,
    options: &BakeOptions,
) -> Result<Vec<BakedQuad>> {
    let mut output = Vec::with_capacity(element.faces.len());
    for (direction, face) in &element.faces {
        let mut positions = face_positions(*direction, element.from, element.to);
        if let Some(rotation) = &element.rotation {
            for position in &mut positions {
                *position = rotate_element_point(*position, rotation);
            }
        }
        let mut normal = face_normal_from_positions(positions);
        if length_squared(normal) <= f32::EPSILON {
            normal = direction.normal();
        }
        let texture = resolve_texture(&model.textures, &face.texture, &options.missing_texture)?;
        let uv = face
            .uv
            .unwrap_or_else(|| default_uv(*direction, element.from, element.to));
        let mut uvs = uv_corners(uv);
        rotate_uvs(&mut uvs, face.rotation);
        output.push(BakedQuad {
            positions,
            uvs,
            normal,
            texture,
            cull_face: face.cullface,
            tint_index: face.tintindex,
            shade: element.shade,
            light_emission: element.light_emission,
        });
    }
    Ok(output)
}

fn bake_generated_item(model: &ResolvedModel, options: &BakeOptions) -> Result<Vec<BakedQuad>> {
    let mut layers = model
        .textures
        .iter()
        .filter_map(|(key, value)| {
            key.strip_prefix("layer")
                .and_then(|suffix| suffix.parse::<u32>().ok())
                .map(|index| (index, value))
        })
        .collect::<Vec<_>>();
    layers.sort_by_key(|(index, _)| *index);

    let mut output = Vec::new();
    for (_, texture_token) in layers {
        let texture = resolve_texture(&model.textures, texture_token, &options.missing_texture)?;
        let front_z = 8.0 - options.generated_item_depth / 2.0;
        let back_z = 8.0 + options.generated_item_depth / 2.0;
        output.push(BakedQuad {
            positions: [
                [0.0, 0.0, front_z],
                [16.0, 0.0, front_z],
                [16.0, 16.0, front_z],
                [0.0, 16.0, front_z],
            ],
            uvs: [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            normal: [0.0, 0.0, -1.0],
            texture: texture.clone(),
            cull_face: None,
            tint_index: None,
            shade: false,
            light_emission: None,
        });
        output.push(BakedQuad {
            positions: [
                [16.0, 0.0, back_z],
                [0.0, 0.0, back_z],
                [0.0, 16.0, back_z],
                [16.0, 16.0, back_z],
            ],
            uvs: [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            normal: [0.0, 0.0, 1.0],
            texture,
            cull_face: None,
            tint_index: None,
            shade: false,
            light_emission: None,
        });
    }
    Ok(output)
}

fn resolve_texture(
    textures: &BTreeMap<String, String>,
    token: &str,
    missing: &ResourceLocation,
) -> Result<ResourceLocation> {
    let mut current = token;
    let mut visited = BTreeSet::new();
    while let Some(variable) = current.strip_prefix('#') {
        if !visited.insert(variable.to_owned()) {
            return Err(Error::TextureCycle(variable.to_owned()));
        }
        current = textures
            .get(variable)
            .map(String::as_str)
            .ok_or_else(|| Error::MissingTextureVariable(variable.to_owned()))?;
    }
    ResourceLocation::parse(current).or_else(|_| Ok(missing.clone()))
}

fn face_positions(direction: Direction, from: [f32; 3], to: [f32; 3]) -> [[f32; 3]; 4] {
    let [x0, y0, z0] = from;
    let [x1, y1, z1] = to;
    match direction {
        Direction::North => [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]],
        Direction::South => [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
        Direction::West => [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]],
        Direction::East => [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]],
        Direction::Down => [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]],
        Direction::Up => [[x0, y1, z1], [x1, y1, z1], [x1, y1, z0], [x0, y1, z0]],
    }
}

fn default_uv(direction: Direction, from: [f32; 3], to: [f32; 3]) -> [f32; 4] {
    let [x0, y0, z0] = from;
    let [x1, y1, z1] = to;
    match direction {
        Direction::Down => [x0, 16.0 - z1, x1, 16.0 - z0],
        Direction::Up => [x0, z0, x1, z1],
        Direction::North => [16.0 - x1, 16.0 - y1, 16.0 - x0, 16.0 - y0],
        Direction::South => [x0, 16.0 - y1, x1, 16.0 - y0],
        Direction::West => [z0, 16.0 - y1, z1, 16.0 - y0],
        Direction::East => [16.0 - z1, 16.0 - y1, 16.0 - z0, 16.0 - y0],
    }
}

fn uv_corners(uv: [f32; 4]) -> [[f32; 2]; 4] {
    let [u0, v0, u1, v1] = uv;
    [
        [u0 / 16.0, v1 / 16.0],
        [u1 / 16.0, v1 / 16.0],
        [u1 / 16.0, v0 / 16.0],
        [u0 / 16.0, v0 / 16.0],
    ]
}

fn rotate_uvs(uvs: &mut [[f32; 2]; 4], rotation: u16) {
    let steps = (rotation / 90) as usize % 4;
    uvs.rotate_right(steps);
}

fn rotate_element_point(mut point: [f32; 3], rotation: &crate::ElementRotation) -> [f32; 3] {
    for axis in 0..3 {
        point[axis] -= rotation.origin[axis];
    }
    let radians = rotation.angle.to_radians();
    point = rotate_axis(point, rotation.axis, radians);
    if rotation.rescale {
        let scale = radians.cos().abs().recip();
        match rotation.axis {
            Axis::X => {
                point[1] *= scale;
                point[2] *= scale;
            }
            Axis::Y => {
                point[0] *= scale;
                point[2] *= scale;
            }
            Axis::Z => {
                point[0] *= scale;
                point[1] *= scale;
            }
        }
    }
    for axis in 0..3 {
        point[axis] += rotation.origin[axis];
    }
    point
}

fn rotate_axis(point: [f32; 3], axis: Axis, radians: f32) -> [f32; 3] {
    let (sin, cos) = radians.sin_cos();
    let [x, y, z] = point;
    match axis {
        Axis::X => [x, y * cos - z * sin, y * sin + z * cos],
        Axis::Y => [x * cos + z * sin, y, -x * sin + z * cos],
        Axis::Z => [x * cos - y * sin, x * sin + y * cos, z],
    }
}

fn apply_block_transform(quad: &mut BakedQuad, transform: ModelTransform) {
    let center = [8.0, 8.0, 8.0];
    for position in &mut quad.positions {
        for axis in 0..3 {
            position[axis] -= center[axis];
        }
        *position = rotate_axis(
            *position,
            Axis::X,
            (transform.x_degrees as f32).to_radians(),
        );
        *position = rotate_axis(
            *position,
            Axis::Y,
            (transform.y_degrees as f32).to_radians(),
        );
        for axis in 0..3 {
            position[axis] += center[axis];
        }
    }
    quad.normal = normalize(rotate_axis(
        rotate_axis(
            quad.normal,
            Axis::X,
            (transform.x_degrees as f32).to_radians(),
        ),
        Axis::Y,
        (transform.y_degrees as f32).to_radians(),
    ));
    quad.cull_face = transform_direction(quad.cull_face, transform);
    if transform.uv_lock {
        let steps = ((transform.y_degrees.rem_euclid(360)) / 90) as usize;
        quad.uvs.rotate_left(steps % 4);
    }
}

fn transform_direction(
    direction: Option<Direction>,
    transform: ModelTransform,
) -> Option<Direction> {
    let direction = direction?;
    let vector = direction.normal();
    let vector = rotate_axis(
        rotate_axis(vector, Axis::X, (transform.x_degrees as f32).to_radians()),
        Axis::Y,
        (transform.y_degrees as f32).to_radians(),
    );
    Some(direction_from_normal(vector))
}

fn direction_from_normal(normal: [f32; 3]) -> Direction {
    let [x, y, z] = normal;
    let ax = x.abs();
    let ay = y.abs();
    let az = z.abs();
    if ay >= ax && ay >= az {
        if y >= 0.0 {
            Direction::Up
        } else {
            Direction::Down
        }
    } else if ax >= az {
        if x >= 0.0 {
            Direction::East
        } else {
            Direction::West
        }
    } else if z >= 0.0 {
        Direction::South
    } else {
        Direction::North
    }
}

fn face_normal_from_positions(positions: [[f32; 3]; 4]) -> [f32; 3] {
    let a = subtract(positions[1], positions[0]);
    let b = subtract(positions[2], positions[0]);
    normalize(cross(a, b))
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn length_squared(value: [f32; 3]) -> f32 {
    value[0] * value[0] + value[1] * value[1] + value[2] * value[2]
}

fn normalize(value: [f32; 3]) -> [f32; 3] {
    let length = length_squared(value).sqrt();
    if length <= f32::EPSILON {
        value
    } else {
        [value[0] / length, value[1] / length, value[2] / length]
    }
}
