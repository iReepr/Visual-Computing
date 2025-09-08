#version 430 core

layout(location = 0) in vec3 position;
in layout(location = 1) vec4 color;

out vec4 fragColor;

uniform mat4 mirrorMatrix;

void main()
{
     gl_Position = vec4(position, 1.0f);
     fragColor = color;
    // gl_Position = mirrorMatrix * vec4(position, 1.0f); // Part 2 (Mirror)
}