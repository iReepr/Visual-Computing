#version 430 core

out vec4 color;

uniform float squareSize;

void main()
{
    // color = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    // Bonus
    int x = int(floor(gl_FragCoord.x / squareSize));
    int y = int(floor(gl_FragCoord.y / squareSize));
    
    // Alternate color depending on sum of coordinates
    if ((x + y) % 2 == 0)
        color = vec4(1.0, 1.0, 1.0, 1.0); // white
    else
        color = vec4(0.0, 0.0, 1.0, 1.0); // blue
}

