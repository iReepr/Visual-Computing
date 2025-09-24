#version 430 core

layout(location = 0) in vec3 position;
in layout(location = 1) vec4 color;
in layout(location = 2) vec3 normal;

out vec4 fragColor;
out vec3 fragNormal;

uniform mat4 mvp;

void main()
{

     gl_Position = mvp * vec4(position, 1.0f);
     fragColor = color;
     fragNormal = normal;
}