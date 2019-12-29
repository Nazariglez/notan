in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

in vec2 a_texcoord;
out vec2 v_texcoord;

uniform mat3 u_matrix;
uniform mat3 u_tex_matrix;

void main() {
    v_color = a_color;
    v_texcoord = (u_tex_matrix * vec3(a_texcoord, 0.0)).xy;
    gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}