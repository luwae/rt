use std::io::{self, Write};

use crate::cgmath::{vec_to_rgb, Color};

mod cgmath;

fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            let (ir, ig, ib) = vec_to_rgb(&color);
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\rDone.                                ");
}
