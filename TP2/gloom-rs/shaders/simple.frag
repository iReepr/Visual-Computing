#version 430 core

in vec4 fragColor; 
out vec4 color;

uniform float squareSize;

void main()
{
    color = fragColor;

    // Assignment 1
    /* Bonus Checkboard
    int x = int(floor(gl_FragCoord.x / squareSize));
    int y = int(floor(gl_FragCoord.y / squareSize));
    
    if ((x + y) % 2 == 0)
        color = vec4(1.0, 1.0, 1.0, 1.0); // white
    else
        color = vec4(0.0, 0.0, 1.0, 1.0); // blue
    */
}

