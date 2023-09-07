#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_functions    mesh_normal_local_to_world, mesh_position_local_to_world, mesh_tangent_local_to_world, mesh_position_world_to_clip
#import bevy_pbr::mesh_bindings     mesh
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct Time {
    time_since_startup: f32,
};
@group(1) @binding(0)
var<uniform> time: Time;
@group(1) @binding(1)
var<uniform> start: f32;
@group(1) @binding(2)
var<uniform> end: f32;


struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};


@vertex
fn vertex(vertex: Vertex) -> MeshVertexOutput {
    //let thickness = 5.0;
    // higher is shorter
    //let how_long_to_stay_in_opposite_state = 30.0;
    //let frequency = 0.2;
    //let sine = sin(frequency * time.time_since_startup + vertex.position.y + vertex.position.z);
    //let position_diff = 1.0 - pow(thickness * sine, how_long_to_stay_in_opposite_state);
    let elapsed = start - time.time_since_startup;
    let position = (vertex.normal * elapsed) * vertex.position;

    var out: MeshVertexOutput;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
    // out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
    out.position = mesh_position_world_to_clip(out.world_position);
    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};

@fragment
fn fragment(
    in: FragmentInput,
    ) -> @location(0) vec4<f32> {
        let elapsed = start - time.time_since_startup;
        let length = end - start;
        let mag = (in.color.r + in.color.g + in.color.b)/3.0;
        let value = 1.0 - (elapsed / length);
        return vec4(value, value * mag, value * mag * mag, 0.4);
}