
use std::f64::consts::PI;
use std::{thread, time};
fn main(){
    // Donut.rs
    let mut a:f64 = 0.0;
    let mut b:f64 = 0.0;

    loop {
        a += 0.035;
        b += 0.015;

        let (sin_a , cos_a) = a.sin_cos();
        let (sin_b , cos_b) = b.sin_cos();

        let mut output = vec![' ';80*24];
        let mut z_buffer = vec![0.0; 80*24];

        let mut theta: f64 = 0.0;
        while theta < 2.0 * 3.14 {
            let (sin_theta, cos_theta) = theta.sin_cos();
            let mut phi: f64 = 0.0;
            while phi < 2.0 * 3.14 {
                let (sin_phi, cos_phi) = phi.sin_cos();

                let circle_x = 2.0 + cos_theta;
                let circle_y = sin_theta;

                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;
                let z = 5.0 + cos_a * circle_x * sin_phi + circle_y * sin_a;
                let ooz = 1.0 / z;

                let xp = (40.0 + 30.0 * ooz * x) as usize;
                let yp = (12.0 + 15.0 * ooz * y) as usize;

                let L = cos_theta * cos_phi * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
                if L > 0.0 {
                    if ooz > z_buffer[yp * 80 + xp] {
                        z_buffer[yp * 80 + xp] = ooz;
                        let luminance_index = L * 46.6;
                        output[yp * 80 + xp] = ".'`^,:;Il!i><~+_-?][}{1)(|tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".chars().nth(luminance_index as usize).or(Some('_')).unwrap();
                    }
                }

                phi += 0.02;
            }
            theta += 0.07;
        }

        print!("\x1b[H");
        for j in 0..24 {
            for i in 0..80 {
                print!("{}", output[j * 80 + i]);
            }
            print!("\n");
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}