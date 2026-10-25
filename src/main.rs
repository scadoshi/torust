use std::{f32::consts::PI, sync::LazyLock};
use terminal_size::{Height, Width, terminal_size};

/// How far θ moves each step around the small circle
const THETA_STEP: f32 = 0.07;
/// How far φ moves each step as the circle spins around the y-axis
const PHI_STEP: f32 = 0.02;

/// Radius of the small circle that makes the tube
const R1: f32 = 1.0;
/// Distance from the center of the donut to the center of the tube
const R2: f32 = 2.0;
/// How far the donut sits from the viewer
const K2: f32 = 5.0;

/// Terminal size in columns and rows, read once at startup
static DIMENSIONS: LazyLock<(usize, usize)> = LazyLock::new(|| {
    let (Width(w), Height(h)) = terminal_size().expect("Failed to get terminal dimensions");
    (usize::from(w), usize::from(h))
});

/// Smallest terminal width the donut still looks right in
const MIN_WIDTH: usize = 40;
/// Terminal width in columns
static WIDTH: LazyLock<usize> = LazyLock::new(|| DIMENSIONS.0);

/// Smallest terminal height the donut still looks right in
const MIN_HEIGHT: usize = 20;
/// Terminal height in rows
static HEIGHT: LazyLock<usize> = LazyLock::new(|| DIMENSIONS.1);

/// Screen distance, sized so the donut fits whichever side runs out first.
/// Width is halved because a terminal cell is about twice as tall as it is wide
static K1: LazyLock<f32> = LazyLock::new(|| {
    let fit = (*WIDTH as f32 / 2.0).min(*HEIGHT as f32);
    fit * K2 * 3.0 / (8.0 * (R1 + R2))
});

/// Brightness ramp from darkest to brightest
const CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

/// Draws one frame of the donut rotated by `a` around the x-axis and `b` around the z-axis
fn render_frame(a: f32, b: f32) {
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    let mut output: Vec<Vec<char>> = vec![vec![' '; *WIDTH]; *HEIGHT];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![0.0; *WIDTH]; *HEIGHT];

    let theta_steps = (2.0 * PI / THETA_STEP).ceil() as usize;
    let phi_steps = (2.0 * PI / PHI_STEP).ceil() as usize;

    for i in 0..theta_steps {
        let theta = i as f32 * THETA_STEP;
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let circle_x = R2 + R1 * cos_theta;
        let circle_y = R1 * sin_theta;

        for j in 0..phi_steps {
            let phi = j as f32 * PHI_STEP;
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let x: f32 =
                circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
            let y: f32 =
                circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;

            let z: f32 = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz: f32 = 1.0 / z;

            let xp: f32 = *WIDTH as f32 / 2.0 + 2.0 * *K1 * ooz * x;
            let yp: f32 = *HEIGHT as f32 / 2.0 + *K1 * ooz * y;

            if !(0.0..*WIDTH as f32).contains(&xp) || !(0.0..*HEIGHT as f32).contains(&yp) {
                continue;
            }

            let xp = xp as usize;
            let yp = yp as usize;

            let luminance: f32 =
                cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                    + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            if luminance > 0.0 && ooz > z_buffer[yp][xp] {
                z_buffer[yp][xp] = ooz;
                let luminance_index: usize = (luminance * 8.0) as usize;
                output[yp][xp] = CHARS[luminance_index];
            }
        }
    }

    print!("\x1b[H");
    let rows: Vec<String> = output.iter().map(|r| r.iter().collect()).collect();
    print!("{}", rows.join("\n"));
}

/// Spins a donut forever
fn main() {
    if *WIDTH < MIN_WIDTH || *HEIGHT < MIN_HEIGHT {
        eprintln!("Terminal too small: need at least {MIN_WIDTH}x{MIN_HEIGHT}");
        return;
    }
    print!("\x1b[2J");

    let (mut a, mut b) = (0.0_f32, 0.0_f32);
    loop {
        render_frame(a, b);
        a += 0.04;
        b += 0.02;
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
