#version 150 core

uniform b_Color {
    vec4 u_Color;
};

out vec4 Target0;

void main() {
    Target0 = u_Color;
}
