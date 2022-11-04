#version 330 core
layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 txt;
layout (location = 2) in vec4 col;

//uniform mat4 u_mvp;

out vec2 v_text;
out vec4 v_col;
out sampler2D v_tex;

void main() {
    gl_Position = pos;
    // gl_Position = u_mvp * pos;
    v_text = txt;
    v_col = col;
    v_tex = tind;
}

# frag
#version 330 core
layout (location = 0) out vec4 color;

in vec2 v_text;
in vec4 v_col;
in sampler2D v_tex;

uniform sampler2D u_tex;

void main() {
    // color = vec4(1.0, 0.0, 0.0, 1.0);
    color = v_col;
    // color = vec4(textColor, 1.0) * sampled;
}
