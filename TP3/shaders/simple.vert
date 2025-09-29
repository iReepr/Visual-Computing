#version 430 core

in layout(location = 0) vec3 position;
in layout(location = 1) vec4 color;
in layout(location = 2) vec3 normal;

out vec4 fragColor;
out vec3 fragNormal;
out vec3 fragPos;

uniform mat4 mvp;
uniform mat4 model;

void main()
{

     gl_Position = mvp * vec4(position, 1.0f);
     fragColor = color;
     // fragNormal = normal;
     fragNormal = normalize(mat3(model) * normal);
     fragPos = position;
}