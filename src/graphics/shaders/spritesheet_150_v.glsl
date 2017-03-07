#version 150 core

in vec3 a_Pos;
in vec2 a_Uv;

uniform b_ModelData {
    mat4 u_Model;
};

uniform b_ProjData {
    mat4 u_View;
    mat4 u_Proj;
};

out vec2 v_Uv;

void main() {
    v_Uv = a_Uv;
    gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
}
