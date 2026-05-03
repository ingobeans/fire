#version 100

precision highp float;

uniform sampler2D normal;
uniform sampler2D regular;
varying vec2 uv;
uniform vec4 _Time;
uniform vec2 lightPos;

void main() {
    // gl_FragColor = vec4(lightPos.x,lightPos.y,1.0,1.0);return;
    vec4 normal = texture2D(normal, uv);
    vec4 vector = (2.0*normal)-1.0;
    vec3 lightDir = vec3(normalize(lightPos-uv), 1.0);
    lightDir.y = -lightDir.y;

    float diff = max(dot(vector.xyz, lightDir), 0.0);
    vec4 c = texture2D(regular, uv) * diff * max(1.0-length(lightPos-uv),0.0);
    gl_FragColor = c;
}