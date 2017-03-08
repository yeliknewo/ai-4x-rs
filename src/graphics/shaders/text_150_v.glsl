#version 150 core

in vec3 a_Pos;
in vec4 a_Color;

uniform b_ModelData {
    mat4 u_Model;
};

uniform b_ProjData {
    mat4 u_View;
    mat4 u_Proj;
};

uniform b_Offset {
    vec2 u_Offset;
};

out vec4 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos + vec3(u_Offset, 0.0), 1.0);
}
