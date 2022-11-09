#version 330 core
layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 txt;
layout (location = 3) in 
layout (location = 2) in vec4 col;
//uniform mat4 u_mvp;

out vec2 v_text;
out vec4 v_col;

void main() {
    gl_Position = pos;
    // gl_Position = u_mvp * pos;
    v_text = txt;
    v_col = col;
}

# frag
#version 330 core
layout (location = 0) out vec4 color;

in vec2 v_text;
in vec4 v_col;

uniform sampler2D u_tex;

vec4 s;

void main() {
    // color = vec4(1.0, 0.0, 0.0, 1.0);
    s = texture(u_tex, v_text);
    color = vec4(mix(s.xyz, v_col.xyz, v_col.a), s.a);
    // color = vec4(textColor, 1.0) * sampled;
}
