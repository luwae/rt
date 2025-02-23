use std::cmp;
use std::collections::VecDeque;
use std::io::{self, Write};

use ray::Ray;

use crate::cgmath::{vec_to_rgb, Color, Point3, Vec3};

mod cgmath;
mod ray;

fn ray_color(ray: &Ray) -> Color {
    let a = 0.5 * (ray.dir().y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let image_height = cmp::max(image_height, 1usize);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = (pixel_center - camera_center).normalize();
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            let (ir, ig, ib) = vec_to_rgb(&pixel_color);
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\rDone.                                ");
}
