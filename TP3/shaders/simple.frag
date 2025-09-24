#version 430 core

in vec3 fragNormal; 
in vec4 fragColor;
out vec4 color;

void main()
{

    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

    vec3 normal = normalize(fragNormal);

    float diffuse = max(0.0, dot(normal, -lightDirection));
    
    vec3 litColor = fragColor.rgb * diffuse;

    color = vec4(litColor, 1.0);
}
