@compute @workgroup_size(16, 16)
fn main_image(@builtin(global_invocation_id) id: vec3u) {
    // Viewport resolution (in pixels)
    let screen_size = textureDimensions(screen);

    // Prevent overdraw for workgroups on the edge of the viewport
    if (id.x >= screen_size.x || id.y >= screen_size.y) { return; }

    // Pixel coordinates (centre of pixel, origin at bottom left)
    let fragCoord = vec2f(f32(id.x) + .5, f32(screen_size.y - id.y) - .5);

    // Normalised pixel coordinates (from 0 to 1)
    let uv = fragCoord / vec2f(screen_size) - 0.5;

    let camera_position = vec3f(-3, 0, 0);
    let camera_direction = vec3f(1, 0, 0);

    let cam_x = cross(normalize(camera_direction), vec3f(0,0,1));
    let cam_y = cross(cam_x, normalize(camera_direction));

    let image_width: f32 = f32(screen_size.x);
    let image_height: f32 = f32(screen_size.y);
    let aspect_ratio: f32 = image_width / image_height;

    let ray_direction = normalize(camera_direction + (uv.x * cam_x * aspect_ratio) + (uv.y * cam_y));

    // Output to screen
    textureStore(screen, id.xy, trace(camera_position, ray_direction));
}

const max_steps = 10000;
const max_distance = 10000.0;
const min_distance = 0.1;

const normal_sampling_distance = 0.001;

fn sdf(point: vec3f) -> f32 {
    return length(point) - 1;
}

fn trace(src: vec3f, direction: vec3f) -> vec4f {
    var total_distance: f32 = 0.0;

    for(var i = 0; i < max_steps; i++) {
        let current_point = src + (total_distance * direction);
        let distance = sdf(current_point);

        if distance > max_distance {
            break;
        }

        total_distance += distance;

        if distance < min_distance {
            // Approximate normal with finite differences
            let dx = normal_sampling_distance * vec3f(1, 0, 0);
            let dy = normal_sampling_distance * vec3f(0, 1, 0);
            let dz = normal_sampling_distance * vec3f(0, 0, 1);
            let normal = normalize(vec3f(
                sdf(current_point + dx) - sdf(current_point - dx),
                sdf(current_point + dy) - sdf(current_point - dy),
                sdf(current_point + dz) - sdf(current_point - dz),
            ));

            let lambertian = dot(normal, direction);
            
            return vec4f(-lambertian);
        }
    }

    return vec4f(0.5, 0, 0, 0);
}