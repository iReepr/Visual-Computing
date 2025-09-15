#version 430 core

layout(location = 0) in vec3 position;
in layout(location = 1) vec4 color;

out vec4 fragColor;

/*
uniform float a;
uniform float b;
uniform float c;
uniform float d;
uniform float e;
uniform float f;
*/

uniform mat4 transform_matrix;

void main()
{
     // mat4 transform_matrix = mat4(1.0);

     /*
     transform_matrix[0][0] = a; // scale X
     transform_matrix[0][1] = b; // shear XY
     transform_matrix[0][3] = c; // translation X

     transform_matrix[1][0] = d; // shear YX
     transform_matrix[1][1] = e; // scale Y
     transform_matrix[1][3] = f; // translation Y
     */

     gl_Position = transform_matrix * vec4(position, 1.0f);
     fragColor = color;
}