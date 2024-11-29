#version 460


// Doit sortir un out vec4 qui represente la couleur
out vec4 color;

in vec4 position_in_world_space;
in vec3 normal_in_world_space;
in vec2 uv_frag;

layout(location=1) uniform vec4 color_uniform;
layout(location=2) uniform vec3 light_position;

layout(location=5) uniform mat4 camera_to_world;

void main()
{
    // Quantité de lumière:
    //
    // 1 / dist(lampe) ^ 2 * albedo * f(wi, wo) * cos(wi, N)

    vec3 light_vector = light_position - position_in_world_space.xyz;

    float d2 = dot(light_vector, light_vector);
    vec3 wi = light_vector / sqrt(d2);

    vec3 light_emission = vec3(200, 200, 200);

    float cos_term = max(dot(normal_in_world_space, wi), 0);

    vec4 centre_camera_world = vec4(0, 0, 0, 1) * camera_to_world;
    vec3 wo = normalize(centre_camera_world.xyz - position_in_world_space.xyz);

    float f = pow(dot(normalize(wi + wo), normal_in_world_space.xyz), 10);
    if(isnan(f))
        f = 0.0;

    vec3 emit = vec3(0.5, 0.5, 0.5);

    vec3 light_contrib = color_uniform.xyz / d2 * f * light_emission * cos_term;

    bool n = isnan(f);
    color = vec4(light_contrib + emit, 1);
}
