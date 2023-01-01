#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_shader_utils::simplex_noise_3d

#import bevy_shader_utils::simplex_noise_2d

struct CustomMaterial {
    color: vec4<f32>,
    // center: vec3<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

// @group(1) @binding(1)
// var<uniform> material_center: vec3<f32>;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // var world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));

    // The noise we use to separate between sea and grounds
    var noise3 = simplexNoise3(vertex.normal + globals.time * 0.2);
    // The noise we use to generate "mountains".
    // The number we multiply for, it decides the granularity of the mountains.
    // Theoretically this number could be used to zoom in and out the details of the planet?
    var noise_ranged3 = (simplexNoise3(vec3<f32>(vertex.position.x * 5.0, vertex.position.y * 5.0, vertex.position.z * 5.0)) + 1.0) / 2.0;

    // var noise_x = simplexNoise2(vec2<f32>(vertex.position.x * 20.0, globals.time ));
    // var noise_y = simplexNoise2(vec2<f32>(vertex.position.y * 20.0, globals.time ));
    // var noise_z = simplexNoise2(vec2<f32>(vertex.position.z * 20.0, globals.time ));
    // var normal_noise = vec3<f32>((noise_x + 1.0) / 2.0, (noise_y + 1.0) / 2.0, (noise_z + 1.0) / 2.0);


    var position = vertex.position;
    // If noise is positive (between 0.0 and 1.0), then use it for the ground.
    // Otherwise stays flat (sea).
    if noise3 >= 0.0 {
        if noise3 >= 0.1 {
            position = vertex.position + vertex.normal * noise_ranged3 * 0.05;
        } else { // if between 0.0 and 0.1, use it for the sand (so stays flat).
            position = vertex.position;
        }
    }

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    // This distance indicates how much the computed position has diverted from the original position (from the sea level).
    var distance_to_origin = distance(position, vertex.position);

    var green = 0.0;
    var blue = 0.0;
    var red = 0.0;

    if noise3 >= 0.0 {
        if noise3 >= 0.1 {
            // Mountains
            green = distance_to_origin * 2.0;
        } else {
            // Sand
            green = 0.8;
            red = 0.95;
            blue = 0.15;
        }
        
    } else {
        // Sea color: here the noise is a negative number.
        // So we subtract the noise number to the maximum blue color (1.0).
        // The more the noise is negatively distant from 0, the deeper the blue color is.
        // We multiply for 1.5 because the noise is a very small number.
        blue = 1.0 + noise3 * 1.5;
    }

    out.blend_color = vec4<f32>(red, green, blue, 1.0);
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // return material.color * input.blend_color;
    return input.blend_color;
}