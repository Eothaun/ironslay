#version 450

// Attributes
// ============================================================================
// Input
layout(location = 0) in vec2 i_Uv;
// Output 
layout(location = 0) out vec4 o_Target;

// Uniforms
layout(set = 2, binding = 0) uniform HexMaterial_color {
    vec4 color;
};
layout(set = 2, binding = 1) uniform HexMaterial_highlighted_coord {
    vec2 highlighted_coord;
};
layout(set = 2, binding = 2) uniform HexMaterial_selected_coord {
    vec2 selected_coord;
};
layout(set = 2, binding = 3) uniform texture2D HexMaterial_background_texture;
layout(set = 2, binding = 4) uniform sampler HexMaterial_background_texture_sampler;
layout(set = 2, binding = 5) uniform utexture2D HexMaterial_map_state;
layout(set = 2, binding = 6) uniform sampler HexMaterial_map_state_sampler;
// ============================================================================


// Hex functions
float hex_dist(vec2 p) {
    p = abs(p);

    float c = dot(p, normalize(vec2(1.0, 1.73)));
    c = max(c, p.x);

    return c;
}

vec2 hex_relative_uv(vec2 uv) {
    vec2 r = vec2(1.0, 1.73);
    vec2 h = r * 0.5;

    vec2 a = mod(uv, r) - h;
    vec2 b = mod(uv + h, r) - h;

    if (length(a) < length(b)) {
        return a;
    } else {
        return b;
    }
}

ivec2 hex_grid_coord(vec2 id) {
    vec2 r = vec2(1.0, 1.73);
    vec2 h = r * 0.5;
    // Meh floating point precision
    return ivec2(id / h + vec2(0.1, 0.1));
}


// Fragment shader
void main() {
    vec2 uv = i_Uv * 5.0;

    vec2 gv = hex_relative_uv(uv);
    float hex_dist = 0.5 - hex_dist(gv);
    vec2 id = uv - gv;
    ivec2 coord = hex_grid_coord(id);

    vec3 col = color.rgb;

    bool fragment_in_highlight = distance(coord, highlighted_coord) < 0.1;
    col *= mix(0.4, 1.0, float(fragment_in_highlight));

    bool fragment_in_selected = distance(coord, selected_coord) < 0.1;
    col *= mix(vec3(1.0), vec3(1.0, 1.0, 0.2), float(fragment_in_selected));

    bool fragment_in_border = hex_dist < 0.04;
    col += vec3(float(fragment_in_border));

    col *= texture(sampler2D(HexMaterial_background_texture, HexMaterial_background_texture_sampler), i_Uv).xyz;

    uint map_data = texelFetch(usampler2D(HexMaterial_map_state, HexMaterial_map_state_sampler), coord, 0).r;
    if(map_data == 0)
        col *= vec3(0.0, 1.0, 0.0);
    else if(map_data == 1)
        col *= vec3(0.0, 0.0, 1.0);

    o_Target = vec4(col.rgb, color.a);
}
