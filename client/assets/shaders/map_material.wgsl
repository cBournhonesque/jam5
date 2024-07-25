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
    var scaled_pos = vec2f(pos.x * 0.45, pos.y);
    scaled_pos += vec2f(0.0, -2.5);

    let local_x = fract(scaled_pos.x / grid_size) - 0.5;
    let local_y = fract(scaled_pos.y / grid_size) - 0.5;
    let diamond_dist = abs(local_x) + abs(local_y);

    // scrolling perlin noise
    let noise_scrolled_large_screen = perlin(in.position.xy / 500.0 + vec2f(time * 0.1, time * 0.1)) * color.rgb * 6.0;
    let noise_scrolled_slow_screen = perlin(in.position.xy / 500.0 + vec2f(0.1, time * 0.1)) * color.rgb * 6.0;
    
    let noise_scrolled_large_world = perlin(in.world_position.xy / 100.0 + vec2f(time * 0.5, time * 0.5)) * color.rgb * 2.0;
    let noise_scrolled_slow_world = perlin(in.world_position.xy / 100.0 + vec2f(0.1, time * 0.1)) * color.rgb * 5.0;
    
    let noise = mix(
        noise_scrolled_large_screen + noise_scrolled_slow_world,
        noise_scrolled_slow_screen + noise_scrolled_large_world,
        0.5,
    );

    // fresnel
    let fresnel = 1.0 - dot(normalize(in.position.xy), vec2f(0.0, 1.0));

    let animation_progress = (sin(time * 3.5) + 1.0) * 0.5;
    let checkerboard_min = mix(0.2, 0.25, animation_progress);
    let checkerboard_max = checkerboard_min + 0.05;

    let base_checkboard_color = 1.0 - smoothstep(checkerboard_min, checkerboard_max, diamond_dist) + color.rgb * 0.5;
    let base = mix(
        vec3<f32>(0.0, 0.0, 0.0),
        color.rgb,
        base_checkboard_color,
    );
    
    return vec4<f32>( 
        base.rgb + noise.rgb * 0.05,
        fresnel
    );
}

fn rotate(pos: vec2<f32>, angle: f32) -> vec2<f32> {
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    return vec2<f32>(
        pos.x * cos_a - pos.y * sin_a,
        pos.x * sin_a + pos.y * cos_a,
    );
}
fn soft_circle(uv: vec2<f32>, radius: f32, softness: f32) -> f32 {
    return smoothstep(radius - softness, radius + softness, length(uv));
}

// https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39#perlin-noise
fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }
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