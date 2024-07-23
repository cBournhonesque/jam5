#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_render::view View

@group(0) @binding(0) var<uniform> view: View;

@group(2) @binding(100)
var<uniform> color: vec4<f32>;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.world_position.xy;
    let time = globals.time;

    // grid size
    let grid_size = 75.0;

    // scale to create diamonds
    var scaled_pos = vec2f(pos.x * 0.5, pos.y);
    scaled_pos += vec2f(0.0, time * -2.5);

    let local_x = fract(scaled_pos.x / grid_size) - 0.5;
    let local_y = fract(scaled_pos.y / grid_size) - 0.5;
    let diamond_dist = abs(local_x) + abs(local_y);

    // scrolling perlin noise
    let noise_scrolled_large = perlin(scaled_pos / 100.0 + vec2f(time * 0.5, time * 0.5));
    let noise_scrolled_slow = perlin(scaled_pos / 100.0 + vec2f(time * 0.1, time * 0.1));
    // fresnel
    var fresnel = 1.0 - dot(normalize(in.position.xy), vec2f(0.0, 1.0));
    fresnel = pow(fresnel, 2.0);

    let base_checkboard_color = 1.0 - smoothstep(0.45, 0.55, diamond_dist);
    let final_color = mix(
        base_checkboard_color * color * 0.55,
        vec4<f32>(0.0, 0.0, 0.0, 0.15),
        .5,
    ) * vec4(0.0, 0.0, 0.0, 0.15) + vec4(0.0, 0.0, 0.0, 0.25) + noise_scrolled_large * 0.05;

    let final_color2 = mix(
        final_color,
        vec4<f32>(0.0, 0.0, 0.0, 0.15),
        fresnel,
    ) + color * 0.15 + noise_scrolled_slow * 0.05;

    return vec4<f32>(final_color2.r, final_color2.g, final_color2.b, 0.5);
}

fn rotate(pos: vec2<f32>, angle: f32) -> vec2<f32> {
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    return vec2<f32>(
        pos.x * cos_a - pos.y * sin_a,
        pos.x * sin_a + pos.y * cos_a,
    );
}

// Simple value noise function
fn simple_noise(st: vec2<f32>) -> f32 {
    let i = floor(st);
    let f = fract(st);

    // Four corners in 2D of a tile
    let a = random(i);
    let b = random(i + vec2<f32>(1.0, 0.0));
    let c = random(i + vec2<f32>(0.0, 1.0));
    let d = random(i + vec2<f32>(1.0, 1.0));

    // Smooth Interpolation
    let u = f * f * (3.0 - 2.0 * f);

    // Mix 4 corners percentages
    return mix(a, b, u.x) +
            (c - a)* u.y * (1.0 - u.x) +
            (d - b) * u.x * u.y;
}

fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}
fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn soft_circle(uv: vec2<f32>, radius: f32, softness: f32) -> f32 {
    return smoothstep(radius - softness, radius + softness, length(uv));
}

fn perlin(P: vec2f) -> f32 {
    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 * vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}