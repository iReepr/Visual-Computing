#version 430 core

in vec3 position;

uniform mat4 mirrorMatrix;

void main()
{
    // gl_Position = vec4(position, 1.0f);
    gl_Position = mirrorMatrix * vec4(position, 1.0f); // Part 2
}