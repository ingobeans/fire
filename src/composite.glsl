#version 100

precision highp float;

uniform sampler2D _ScreenTexture;
varying vec4  uv;
uniform vec4 _Time;
// uniform vec2 lightPos;

void main() {
    // gl_FragColor = vec4(uv.x,uv.y,1.0,1.0);return;
    if (uv.x > 0.5 ) {
        gl_FragColor = vec4(0.0,0.0,0.0,1.0);
        return;
    }
    vec2 uv1 = vec2(uv.x,uv.y);
    vec2 uv2 = vec2(uv.x+0.5,uv.y);

    vec4 normal = texture2D(_ScreenTexture, uv2);
    vec4 vector = (2.0*normal)-1.0;
    vec3 lightDir = vec3(cos(_Time.x),sin(_Time.x), 1.0);

    float diff = max(dot(vector.xyz, lightDir), 0.0);
    vec4 c = texture2D(_ScreenTexture, uv1) * diff;
    gl_FragColor = c;
}