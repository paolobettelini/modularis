#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    view_transformations::position_world_to_clip,
}

struct GrassMaterialUniform {
    wind: vec4<f32>,
    appearance: vec4<f32>,
    base_color: vec4<f32>,
    interaction_header: vec4<f32>,
    interaction_positions: array<vec4<f32>, 8>,
    interaction_axes: array<vec4<f32>, 8>,
    interaction_parameters: array<vec4<f32>, 8>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: GrassMaterialUniform;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) height: f32,
};

fn hue_rotate(color: vec3<f32>, angle: f32) -> vec3<f32> {
    let axis = normalize(vec3<f32>(1.0, 1.0, 1.0));
    return color * cos(angle)
        + cross(axis, color) * sin(angle)
        + axis * dot(axis, color) * (1.0 - cos(angle));
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var world_position =
        mesh_functions::mesh_position_local_to_world(
            world_from_local,
            vec4<f32>(vertex.position, 1.0),
        ).xyz;

    let height = clamp(vertex.uv.y, 0.0, 1.0);
    let root_weight = height * height;
    let wind_length = max(length(material.wind.xy), 0.0001);
    let wind_direction = material.wind.xy / wind_length;
    let wind_intensity = clamp(material.wind.z, 0.0, 5.0);
    let spatial_phase = dot(world_position.xz, vec2<f32>(0.31, 0.23));
    let blade_phase = vertex.uv.x * 6.2831853;
    let broad_gust = sin(spatial_phase + globals.time * (0.9 + wind_intensity * 0.35));
    let detail_gust =
        sin(spatial_phase * 2.17 - globals.time * (1.7 + wind_intensity * 0.55) + blade_phase);
    let gust = 0.58 + broad_gust * 0.27 + detail_gust * 0.15;
    let bend = wind_intensity * 0.16 * gust * root_weight;

    let cross_wind = vec2<f32>(-wind_direction.y, wind_direction.x);
    let flutter = sin(globals.time * (4.8 + vertex.uv.x * 1.7) + blade_phase)
        * wind_intensity
        * 0.018
        * root_weight
        * height;
    var horizontal_offset = wind_direction * bend + cross_wind * flutter;
    var vertical_offset = 0.0;
    for (var index = 0u; index < 8u; index += 1u) {
        if (f32(index) >= material.interaction_header.x) {
            continue;
        }
        let source = material.interaction_positions[index];
        let axis_data = material.interaction_axes[index];
        let capsule_axis = normalize(axis_data.xyz);
        let from_center = world_position - source.xyz;
        let along_axis =
            clamp(dot(from_center, capsule_axis), -axis_data.w, axis_data.w);
        let nearest = source.xyz + capsule_axis * along_axis;
        let contact_delta = world_position - nearest;
        let contact_distance = length(contact_delta);
        let contact =
            1.0 - smoothstep(source.w * 0.35, source.w, contact_distance);
        let influence = contact
            * material.interaction_parameters[index].x
            * root_weight;
        let horizontal_delta = contact_delta.xz;
        let horizontal_distance = length(horizontal_delta);
        let phase_direction = vec2<f32>(cos(blade_phase), sin(blade_phase));
        let away = select(
            phase_direction,
            horizontal_delta / max(horizontal_distance, 0.001),
            horizontal_distance > 0.001,
        );
        horizontal_offset += away * influence * material.interaction_header.y;
        vertical_offset -= influence * material.interaction_header.z;
    }
    world_position += vec3<f32>(horizontal_offset.x, vertical_offset, horizontal_offset.y);

    let jitter = (vertex.uv.x * 2.0 - 1.0) * material.base_color.w;
    let gradient = mix(
        material.appearance.y,
        material.appearance.z,
        pow(height, material.appearance.w),
    );
    let wave = 0.96 + 0.04 * sin(spatial_phase - globals.time * 0.7);
    var output: VertexOutput;
    output.position = position_world_to_clip(world_position);
    output.color = vec4<f32>(
        clamp(
            hue_rotate(vertex.color.rgb, jitter)
                * material.base_color.rgb
                * gradient
                * wave
                * material.appearance.x,
            vec3<f32>(0.0),
            vec3<f32>(1.0),
        ),
        1.0,
    );
    output.height = height;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
