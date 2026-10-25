use std::{f32::consts::PI, sync::LazyLock};
use terminal_size::{Height, Width, terminal_size};

/// "So we have a circle of radius R1 centered at point (R2,0,0), drawn on the xy-plane. We can draw this by sweeping an angle — let's call it θ — from 0 to 2π..."
const THETA_SPACING: f32 = 0.07;
/// "Now we take that circle and rotate it around the y-axis by another angle — let's call it φ. To
/// rotate an arbitrary 3D point around one of the cardinal axes, the standard technique is to
/// multiply by a rotation matrix. So if we take the previous points and rotate about the y-axis we
/// get..."
const PHI_SPACING: f32 = 0.02;

/// Torus's internal circle radius
const R1: f32 = 1.0;
/// Distance from torus's internal circle center and pivot axis (The "external" circle the internal circle rotates around)
const R2: f32 = 2.0;
/// Distance of the donut from the viewer
const K2: f32 = 5.0;

static SCREEN_DIMENSIONS: LazyLock<(usize, usize)> = LazyLock::new(|| {
    let (Width(w), Height(h)) = terminal_size().expect("Failed to get terminal dimensions");
    (usize::from(w), usize::from(h))
});

static SCREEN_WIDTH: LazyLock<usize> = LazyLock::new(|| {
    let (w, _) = *SCREEN_DIMENSIONS;
    w
});

static SCREEN_HEIGHT: LazyLock<usize> = LazyLock::new(|| {
    let (_, h) = *SCREEN_DIMENSIONS;
    h
});

static K1: LazyLock<f32> = LazyLock::new(|| *SCREEN_WIDTH as f32 * K2 * 3.0 / (8.0 * (R1 + R2)));

const CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

fn render_frame(a: f32, b: f32) {
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    let mut output: Vec<Vec<char>> = vec![vec![' '; *SCREEN_HEIGHT]; *SCREEN_WIDTH];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![0.0; *SCREEN_HEIGHT]; *SCREEN_WIDTH];

    let mut theta: f32 = 0.0;
    while theta < (2.0 * PI) {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let mut phi: f32 = 0.0;
        while phi < (2.0 * PI) {
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let circle_x = R2 + R1 + cos_theta;
            let circle_y = R1 * sin_theta;

            let x: f32 =
                circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
            let y: f32 =
                circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;

            let z: f32 = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let one_over_z: f32 = 1.0 / z;

            let x_prime: usize = (*SCREEN_WIDTH as f32 / 2.0 + *K1 * one_over_z * x) as usize;
            let y_prime: usize = (*SCREEN_HEIGHT as f32 / 2.0 + *K1 * one_over_z * y) as usize;

            let luminance: f32 =
                cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                    + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            if luminance > 0.0 && one_over_z > z_buffer[x_prime][y_prime] {
                z_buffer[x_prime][y_prime] = one_over_z;
                let luminance_index: usize = (luminance * 8.0) as usize;
                output[x_prime][y_prime] = CHARS[luminance_index];
            }

            phi += PHI_SPACING;
        }

        theta += THETA_SPACING;
    }

    print!("\x1b[H");
    (0..*SCREEN_HEIGHT).for_each(|j| {
        (0..*SCREEN_WIDTH).for_each(|i| {
            print!("{}", output[i][j]);
        });
        println!();
    });
}

fn main() {
    (0..100).for_each(|a| {
        (0..100).for_each(|b| {
            render_frame(a as f32, b as f32);
        })
    })
}
