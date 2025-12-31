@group(0) @binding(0) var screen: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;

// Vertex shader - generates full-screen triangle
@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32) -> @builtin(position) vec4<f32> {
    // Generate clip-space coordinates directly
    let x = f32(vert_idx & 1) * 4.0 - 1.0;
    let y = f32(vert_idx >> 1) * 4.0 - 1.0;
    return vec4(x, y, 0.0, 1.0);
}

// Fragment shader - samples texture
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // Viewport resolution (in pixels)
    let screen_size = textureDimensions(screen);

    let uv = pos.xy / vec2f(screen_size);

    let color = textureSample(screen, samp, uv.xy);

    //let color = vec4f(
    //    floor(uv.x + 0.5),  // R: Left=0, Right=1
    //    floor(uv.y + 0.5),  // G: Top=0, Bottom=1
    //    0.5,                // B: Constant
    //    1.0
    //);

    // Convert RGBA to BGRA and apply sRGB correction
    return vec4f(pow(color.rgb, vec3f(2.2)), color.a);

}
