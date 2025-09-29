#version 430 core

in vec3 fragNormal; 
in vec4 fragColor;
in vec3 fragPos;

out vec4 color;

void main()
{
   
    vec3 lightPos = vec3(50.0, 50.0, -10.0);
    vec3 lightColor = vec3(1.0, 1.0, 1.0);

    vec3 norm = normalize(fragNormal);

    // Ambient
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // Diffuse
    vec3 lightDir = normalize(lightPos - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * fragColor.rgb * lightColor;

    // Specular
    vec3 viewPos = vec3(0.0, 0.0, 5.0);
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float specularStrength = 0.5;
    float shininess = 32.0;
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = ambient + diffuse + specular;

    color = vec4(result, 1.0);
}
