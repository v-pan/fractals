// Camera params
// TODO: Feed in through a uniform buffer
const camera_position = vec3f(-4, 1, 1);
const camera_direction = vec3f(1, 0, -0.3);
const jitter_strength = 0.00005;

// Light params
// TODO: Feed in through a uniform buffer
const light_position = vec3f(-4, 0, 5);
const light_color = vec3f(1.0);
const diffuse_power = 20.0;
const specular_power = 10.0;

const ambient_color = vec3f(0.1);

// Material params
// TODO: Feed in through a uniform buffer
const diffuse_color = vec3f(0.5);
const specular_color = vec3f(1.0);
const shininess = 1.0;

// Screen params
// TODO: Feed in through a uniform buffer
const gamma = 2.2; // sRGB

@compute @workgroup_size(16, 16)
fn main_image(@builtin(global_invocation_id) id: vec3u) {
    // Viewport resolution (in pixels)
    let screen_size = textureDimensions(screen);

    // Prevent overdraw for workgroups on the edge of the viewport
    if (id.x >= screen_size.x || id.y >= screen_size.y) { return; }

    // Pixel coordinates (centre of pixel, origin at bottom left)
    let fragCoord = vec2f(f32(id.x) + .5, f32(screen_size.y - id.y) - .5);

    // Jitter rays
    // TODO: Feed in a low discrepancy noise texture
    let tex_size = vec2f(textureDimensions(channel0));
    let tex_uv = fragCoord / tex_size;
    var noise = textureSampleLevel(channel0, nearest, fract(tex_uv), 0).rgb;
    // rescale to jitter_strength * [-1, 1]
    let jitter = jitter_strength * (noise.xy - 0.5);

    // Normalised pixel coordinates (from -0.5 to 0.5)
    let uv = fragCoord / vec2f(screen_size) - 0.5 + jitter;

    let cam_x = cross(normalize(camera_direction), vec3f(0,0,1));
    let cam_y = cross(cam_x, normalize(camera_direction));

    let aspect_ratio = f32(screen_size.x) / f32(screen_size.y);

    let ray_direction = normalize(camera_direction + (uv.x * cam_x * aspect_ratio) + (uv.y * cam_y));

    // Output to screen
    textureStore(screen, id.xy, trace(camera_position, ray_direction));
}

fn sphere_sdf(point: vec3f) -> f32 {
    let x = sign(point.x) * (point.x % 1.0);
    let y = sign(point.y) * (point.y % 1.0);

    let instance = vec3f(x, y, point.z) - vec3f(0.5);

    return length(instance) - 0.15;

}

fn sierpinsky_sdf(point: vec3f) -> f32 {
    let max_iterations = 3;
    let scale = 0.5;

    var p = point;

    let a1 = vec3f(1.0, 1.0, 1.0);
    let a2 = vec3f(-1.0, -1.0, 1.0);
    let a3 = vec3f(1.0, -1.0, -1.0);
    let a4 = vec3f(-1.0, 1.0, -1.0);
    var c = vec3f();
    var d = 0.0;

    var dist = 0.0;

    for (var steps = 0; steps < max_iterations; steps++) {
        c = a1;
        dist = length(p - a1);

        d = length(p - a2);
        if d < dist {
            c = a2;
            dist = d;
        }

        d = length(p - a3);
        if d < dist {
            c = a3;
            dist = d;
        }

        d = length(p - a4);
        if d < dist {
            c = a4;
            dist = d;
        }

        p = scale * p - c * (scale - 1.0);
    }

    return length(p) * pow(scale, f32(-max_iterations));
}

fn box_fold(point: vec3f, dr: f32) -> vec3f {
    let fold_limit = 1.0;
    return (2.0 * clamp(point, vec3f(-fold_limit), vec3f(fold_limit))) - point;
}

fn sphere_fold(point: vec3f, dr: f32) -> vec4f {
    let radius = length(point);
    let min_radius = 0.1;
    let max_radius = 1.0;

    if radius < min_radius {
        let ratio = max_radius / min_radius;
        let coords = point * ratio;
        return vec4f(coords.x, coords.y, coords.z, ratio);
    } else if radius < max_radius {
        let ratio = max_radius / radius;
        let coords = point * ratio;
        return vec4f(coords.x, coords.y, coords.z, ratio);
    } else {
        return vec4f(point.x, point.y, point.z, dr);
    }
}

fn mandelbox_sdf(point: vec3f) -> f32 {
    let max_iterations = 39;
    let scale = 3.0;

    var p = point;
    var dr: f32 = 1.0;

    for (var steps = 0; steps < max_iterations; steps++) {
        p = box_fold(p, dr);

        let fold = sphere_fold(p, dr);
        p = fold.xyz;
        dr = fold.w;

        p = (scale * p) + point;
        dr = dr * abs(scale) + 1.0;
    }
    return length(p) / abs(dr);
}

fn sdf(point: vec3f) -> f32 {
    return sphere_sdf(point);
}

const max_steps = 1000;
const max_distance = 1000.0;
const min_distance = 0.000001;

const normal_sampling_distance = 0.000001;

fn trace(src: vec3f, direction: vec3f) -> vec4f {
    var total_distance: f32 = 0.0;

    for(var steps = 0; steps < max_steps; steps++) {
        let current_point = src + (total_distance * direction);
        let distance_to_surface = sdf(current_point);

        if distance_to_surface > max_distance {
            break;
        }

        // Adaptive step size: smaller steps when close to surfaces
        let step_size = max(distance_to_surface, min_distance * (1.0 + total_distance * 0.01));
        total_distance += step_size;

        var lambertian = 0.0;
        if distance_to_surface < min_distance {
            // Approximate normal with finite differences
            let dx = normal_sampling_distance * vec3f(1, 0, 0);
            let dy = normal_sampling_distance * vec3f(0, 1, 0);
            let dz = normal_sampling_distance * vec3f(0, 0, 1);
            let normal = normalize(vec3f(
                sdf(current_point + dx) - sdf(current_point - dx),
                sdf(current_point + dy) - sdf(current_point - dy),
                sdf(current_point + dz) - sdf(current_point - dz),
            ));

            // Get light vectors
            var light_direction = light_position - current_point;
            let light_distance = dot(light_direction, light_direction);
            light_direction = normalize(light_direction);

            lambertian = max(dot(normal, light_direction), 0.0);

            // Blinn-Phong shading
            var specular = 0.0;
            if lambertian != 0 {
                let halfway = normalize(light_direction + camera_direction);
                let specular_angle = max(dot(halfway, normal), 0.0);
                specular = pow(specular_angle, shininess);
            }
            // Fog
            let fog = vec3f(f32(steps) / f32(max_steps));

            // Linear colorspace intensity mix
            let linear_color = ambient_color +
                                diffuse_color * lambertian * light_color * diffuse_power / light_distance +
                                specular_color * specular * light_color * specular_power / light_distance;
            let gamma_corrected = pow(linear_color, vec3(1.0 / gamma));
            return vec4f(fog, 1.0);
        }
    }

    return vec4f(0.5, 0, 0, 0);
}
