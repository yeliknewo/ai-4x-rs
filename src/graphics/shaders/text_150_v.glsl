#version 150 core

in vec3 a_Pos;

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

void main() {
    gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos + vec3(u_Offset, 0.0), 1.0);
}
