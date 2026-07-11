use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use bevy_mod::BevyMod;
use blocky_formats::{BlockyModel, BlockyShape, QuadNormal, RuntimeModel, ShapeType, UvFace};
use blocky_model_api::{
    BlockyModelApi, BlockyModelNode, BlockyModelRoot, BlockyModelSpawned, BlockyModelVisual,
    SpawnBlockyModel,
};
use client_game_state_api::{GameState, GameStateApi};
use std::{collections::HashMap, path::PathBuf};
use tokio::task::JoinHandle;

pub struct ClientBlockyModelBevyRenderMod;

impl ClientBlockyModelBevyRenderMod {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game_state: &mut G) -> Self {
        bevy.app
            .add_message::<SpawnBlockyModel>()
            .add_message::<BlockyModelSpawned>()
            .init_resource::<BlockyModelCache>()
            .init_resource::<BlockyMaterialCache>()
            .add_systems(
                Update,
                spawn_requested_blocky_models.run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl BlockyModelApi for ClientBlockyModelBevyRenderMod {}

#[derive(Resource, Default)]
struct BlockyModelCache {
    models: HashMap<String, RuntimeModel>,
}

#[derive(Resource, Default)]
struct BlockyMaterialCache {
    materials: HashMap<Option<String>, Handle<StandardMaterial>>,
}

fn spawn_requested_blocky_models(
    mut commands: Commands,
    mut requests: MessageReader<SpawnBlockyModel>,
    mut spawned: MessageWriter<BlockyModelSpawned>,
    mut cache: ResMut<BlockyModelCache>,
    mut material_cache: ResMut<BlockyMaterialCache>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for request in requests.read() {
        let material = material_cache
            .materials
            .entry(request.texture_path.clone())
            .or_insert_with(|| {
                let base_color_texture = request
                    .texture_path
                    .as_ref()
                    .map(|texture_path| asset_server.load(texture_path.clone()));
                materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture,
                    perceptual_roughness: 0.9,
                    cull_mode: None,
                    ..default()
                })
            })
            .clone();

        let runtime = match load_runtime_model(&mut cache, &request.model_path) {
            Ok(runtime) => runtime.clone(),
            Err(error) => {
                warn!(
                    "failed to load blocky model '{}': {error}",
                    request.model_path
                );
                continue;
            }
        };

        let root = commands
            .spawn((
                Transform::from_translation(request.transform.translation)
                    .with_rotation(request.transform.rotation)
                    .with_scale(request.transform.scale * request.scale),
                Visibility::Visible,
                DespawnOnExit(GameState::InGame),
                Name::new(format!("BlockyModel({})", request.model_path)),
            ))
            .id();

        let mut node_entities = vec![Entity::PLACEHOLDER; runtime.nodes.len()];
        let mut visual_entities = vec![None; runtime.nodes.len()];
        for node_index in runtime.roots.iter().copied() {
            spawn_node_recursive(
                &mut commands,
                &runtime,
                node_index,
                root,
                root,
                &material,
                request.primitive_scale,
                request.texture_size,
                &mut meshes,
                &mut node_entities,
                &mut visual_entities,
            );
        }

        commands.entity(root).insert(BlockyModelRoot {
            model_path: request.model_path.clone(),
            node_entities,
            visual_entities,
        });
        spawned.write(BlockyModelSpawned {
            spawn_id: request.spawn_id,
            root,
            model_path: request.model_path.clone(),
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_node_recursive(
    commands: &mut Commands,
    runtime: &RuntimeModel,
    node_index: usize,
    root: Entity,
    parent: Entity,
    material: &Handle<StandardMaterial>,
    primitive_scale: f32,
    texture_size: Option<UVec2>,
    meshes: &mut Assets<Mesh>,
    node_entities: &mut [Entity],
    visual_entities: &mut [Option<Entity>],
) -> Entity {
    let node = &runtime.nodes[node_index];
    let local_position = runtime
        .resolved_local_position(node_index)
        .expect("node index comes from this runtime model");
    let translation = coord_vec3(local_position) * primitive_scale;
    let rotation = quat(node.orientation);
    let entity = commands
        .spawn((
            Transform::from_translation(translation).with_rotation(rotation),
            Visibility::Visible,
            Name::new(format!("BlockyNode({})", node.name)),
        ))
        .id();
    commands.entity(parent).add_child(entity);
    node_entities[node_index] = entity;

    let visual = node.shape.as_ref().and_then(|shape| {
        spawn_visual_for_shape(
            commands,
            shape,
            root,
            entity,
            node_index,
            &node.name,
            material,
            primitive_scale,
            texture_size,
            meshes,
        )
    });
    visual_entities[node_index] = visual;

    commands.entity(entity).insert(BlockyModelNode {
        root,
        node_index,
        name: node.name.clone(),
        visual,
        primitive_scale,
        base_translation: translation,
        base_rotation: rotation,
        base_scale: Vec3::ONE,
    });

    for child in &node.children {
        spawn_node_recursive(
            commands,
            runtime,
            *child,
            root,
            entity,
            material,
            primitive_scale,
            texture_size,
            meshes,
            node_entities,
            visual_entities,
        );
    }
    entity
}

#[allow(clippy::too_many_arguments)]
fn spawn_visual_for_shape(
    commands: &mut Commands,
    shape: &BlockyShape,
    root: Entity,
    node: Entity,
    node_index: usize,
    node_name: &str,
    material: &Handle<StandardMaterial>,
    primitive_scale: f32,
    texture_size: Option<UVec2>,
    meshes: &mut Assets<Mesh>,
) -> Option<Entity> {
    let mesh = mesh_for_shape(shape, primitive_scale, texture_size)?;
    let translation = visual_translation(shape) * primitive_scale;
    let scale = scale_vec3(shape.stretch);
    let visible = shape.visible;
    let visual = commands
        .spawn((
            Transform::from_translation(translation).with_scale(scale),
            if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            BlockyModelVisual {
                root,
                node,
                node_index,
                base_translation: translation,
                base_rotation: Quat::IDENTITY,
                base_scale: scale,
                base_visible: visible,
            },
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material.clone()),
            Name::new(format!("BlockyVisual({node_name})")),
        ))
        .id();
    commands.entity(node).add_child(visual);
    Some(visual)
}

fn load_runtime_model<'a>(
    cache: &'a mut BlockyModelCache,
    model_path: &str,
) -> blocky_formats::Result<&'a RuntimeModel> {
    if !cache.models.contains_key(model_path) {
        let model = BlockyModel::from_path(resolve_runtime_path(model_path))?;
        cache
            .models
            .insert(model_path.to_string(), RuntimeModel::from(&model));
    }
    Ok(cache
        .models
        .get(model_path)
        .expect("model was just inserted"))
}

fn resolve_runtime_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.is_absolute() || direct.exists() {
        direct
    } else {
        PathBuf::from("assets").join(path)
    }
}

fn mesh_for_shape(
    shape: &BlockyShape,
    primitive_scale: f32,
    texture_size: Option<UVec2>,
) -> Option<Mesh> {
    match shape.shape_type {
        ShapeType::Box => Some(box_mesh(shape, primitive_scale, texture_size)),
        ShapeType::Quad => Some(quad_mesh(shape, primitive_scale, texture_size)),
        ShapeType::None | ShapeType::Unknown => None,
    }
}

fn visual_translation(shape: &BlockyShape) -> Vec3 {
    // The format stores the shape center relative to the node pivot. Meshes
    // are centered at their visual entity's origin, so no half-size correction
    // is needed here.
    coord_vec3(shape.offset)
}

fn box_mesh(shape: &BlockyShape, primitive_scale: f32, texture_size: Option<UVec2>) -> Mesh {
    let size = shape.settings.size.unwrap_or(blocky_formats::Vec3f::ONE);
    let raw_size = abs_vec3(scale_vec3(size));
    let size = raw_size * primitive_scale;
    let half = size * 0.5;
    let min = -half;
    let max = half;

    let faces = [
        (
            "right",
            [1.0, 0.0, 0.0],
            [
                [max.x, min.y, min.z],
                [max.x, max.y, min.z],
                [max.x, max.y, max.z],
                [max.x, min.y, max.z],
            ],
        ),
        (
            "left",
            [-1.0, 0.0, 0.0],
            [
                [min.x, min.y, max.z],
                [min.x, max.y, max.z],
                [min.x, max.y, min.z],
                [min.x, min.y, min.z],
            ],
        ),
        (
            "top",
            [0.0, 1.0, 0.0],
            [
                [min.x, max.y, min.z],
                [min.x, max.y, max.z],
                [max.x, max.y, max.z],
                [max.x, max.y, min.z],
            ],
        ),
        (
            "bottom",
            [0.0, -1.0, 0.0],
            [
                [min.x, min.y, max.z],
                [min.x, min.y, min.z],
                [max.x, min.y, min.z],
                [max.x, min.y, max.z],
            ],
        ),
        (
            "front",
            [0.0, 0.0, 1.0],
            [
                [max.x, min.y, max.z],
                [max.x, max.y, max.z],
                [min.x, max.y, max.z],
                [min.x, min.y, max.z],
            ],
        ),
        (
            "back",
            [0.0, 0.0, -1.0],
            [
                [min.x, min.y, min.z],
                [min.x, max.y, min.z],
                [max.x, max.y, min.z],
                [max.x, min.y, min.z],
            ],
        ),
    ];
    build_mesh(faces.map(|(name, normal, vertices)| {
        (
            normal,
            vertices,
            box_face_uvs(shape, name, raw_size, texture_size),
        )
    }))
}

fn quad_mesh(shape: &BlockyShape, primitive_scale: f32, texture_size: Option<UVec2>) -> Mesh {
    let size = shape.settings.size.unwrap_or(blocky_formats::Vec3f::ONE);
    let raw_size = abs_vec3(scale_vec3(size));
    let size = raw_size * primitive_scale;
    let hx = size.x * 0.5;
    let hy = size.y * 0.5;
    let normal = shape.settings.normal.as_ref().unwrap_or(&QuadNormal::PosZ);
    let (normal, vertices) = match normal {
        QuadNormal::PosX => (
            [1.0, 0.0, 0.0],
            [
                [0.0, -hx, -hy],
                [0.0, hx, -hy],
                [0.0, hx, hy],
                [0.0, -hx, hy],
            ],
        ),
        QuadNormal::NegX => (
            [-1.0, 0.0, 0.0],
            [
                [0.0, -hx, hy],
                [0.0, hx, hy],
                [0.0, hx, -hy],
                [0.0, -hx, -hy],
            ],
        ),
        QuadNormal::PosY => (
            [0.0, 1.0, 0.0],
            [
                [-hx, 0.0, -hy],
                [-hx, 0.0, hy],
                [hx, 0.0, hy],
                [hx, 0.0, -hy],
            ],
        ),
        QuadNormal::NegY => (
            [0.0, -1.0, 0.0],
            [
                [-hx, 0.0, hy],
                [-hx, 0.0, -hy],
                [hx, 0.0, -hy],
                [hx, 0.0, hy],
            ],
        ),
        QuadNormal::PosZ => (
            [0.0, 0.0, 1.0],
            [
                [hx, -hy, 0.0],
                [hx, hy, 0.0],
                [-hx, hy, 0.0],
                [-hx, -hy, 0.0],
            ],
        ),
        QuadNormal::NegZ => (
            [0.0, 0.0, -1.0],
            [
                [-hx, -hy, 0.0],
                [-hx, hy, 0.0],
                [hx, hy, 0.0],
                [hx, -hy, 0.0],
            ],
        ),
    };
    build_mesh([(normal, vertices, quad_uvs(shape, raw_size, texture_size))])
}

fn build_mesh<const N: usize>(faces: [([f32; 3], [[f32; 3]; 4], [[f32; 2]; 4]); N]) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    for (normal, vertices, face_uvs) in faces {
        let base = positions.len() as u32;
        positions.extend(vertices.map(coord_array));
        normals.extend([normal_array(normal); 4]);
        uvs.extend(face_uvs);
        indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn box_face_uvs(
    shape: &BlockyShape,
    face_name: &str,
    raw_size: Vec3,
    texture_size: Option<UVec2>,
) -> [[f32; 2]; 4] {
    let (width, height) = match face_name {
        "top" | "bottom" => (raw_size.x, raw_size.z),
        "front" | "back" => (raw_size.x, raw_size.y),
        "left" | "right" => (raw_size.z, raw_size.y),
        _ => (raw_size.x, raw_size.y),
    };
    shape
        .texture_layout
        .get(face_name)
        .map(|uv| uv_rect(uv.clone(), width, height, texture_size))
        .unwrap_or(DEFAULT_UVS)
}

fn quad_uvs(shape: &BlockyShape, raw_size: Vec3, texture_size: Option<UVec2>) -> [[f32; 2]; 4] {
    shape
        .texture_layout
        .get("default")
        .or_else(|| shape.texture_layout.values().next())
        .map(|uv| uv_rect(uv.clone(), raw_size.x, raw_size.y, texture_size))
        .unwrap_or(DEFAULT_UVS)
}

const DEFAULT_UVS: [[f32; 2]; 4] = [[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

fn uv_rect(face: UvFace, width: f32, height: f32, texture_size: Option<UVec2>) -> [[f32; 2]; 4] {
    let texture_size = texture_size.unwrap_or(UVec2::ONE).as_vec2();
    let texture_width = texture_size.x.max(1.0);
    let texture_height = texture_size.y.max(1.0);
    let u0 = face.offset.x / texture_width;
    let v0 = face.offset.y / texture_height;
    let u1 = (face.offset.x + width.max(1.0)) / texture_width;
    let v1 = (face.offset.y + height.max(1.0)) / texture_height;
    let mut uvs = [[u0, v1], [u0, v0], [u1, v0], [u1, v1]];

    if face.mirror.x {
        for uv in &mut uvs {
            uv[0] = u0 + u1 - uv[0];
        }
    }
    if face.mirror.y {
        for uv in &mut uvs {
            uv[1] = v0 + v1 - uv[1];
        }
    }

    let turns = face.angle.rem_euclid(360) / 90;
    uvs.rotate_left(turns as usize);
    uvs
}

fn coord_vec3(value: blocky_formats::Vec3f) -> Vec3 {
    Vec3::new(value.x, value.y, -value.z)
}

fn scale_vec3(value: blocky_formats::Vec3f) -> Vec3 {
    Vec3::new(value.x, value.y, value.z)
}

fn coord_array(value: [f32; 3]) -> [f32; 3] {
    [value[0], value[1], -value[2]]
}

fn normal_array(value: [f32; 3]) -> [f32; 3] {
    [value[0], value[1], -value[2]]
}

fn quat(value: blocky_formats::Quatf) -> Quat {
    Quat::from_xyzw(-value.x, -value.y, value.z, value.w).normalize()
}

fn abs_vec3(value: Vec3) -> Vec3 {
    Vec3::new(value.x.abs(), value.y.abs(), value.z.abs())
}
