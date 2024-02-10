mod ray;
mod vec;
mod hit;
mod sphere;

use vec::{Color, Point3, Vec3};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;

use std::io::{stderr, Write};
use colored::{ColoredString, Colorize};

fn ray_ascii<'a>(r: &'a Ray, world: &'a World) -> ColoredString {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        let color = 255.0 * 0.5 * (rec.normal + Color::new(0.0, 1.0, 0.0));
        let mut chr = " ";
        if rec.normal.z() > 0.95 {
            chr = "@";
        } else if rec.normal.z() > 0.85 {
            chr = "X";
        } else if rec.normal.z() > 0.75 {
            chr = "/";
        } else {
            chr = ".";
        }

        chr.truecolor(color[0] as u8, color[1] as u8, color[2] as u8)
    } else {
        " ".normal()
    }
}

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        // looking at y after normalizing it causes horizontal gradient: why?
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        // linear interpolation
        // start from white and go to blue
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 4.0 / 3.0;
    const IMAGE_WIDTH: u64 = 128;
    const IMAGE_HEIGHT: u64 = ((128 as f64) / ASPECT_RATIO) as u64;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            
            let pixel_char = ray_ascii(&r, &world);
            print!("{}", pixel_char);
        }
        println!("");
    }
    eprint!("Done.");
}
