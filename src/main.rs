mod geo;
mod ray;
mod color;
mod canvas;
mod matrix;
mod projectile;
use rayon::prelude::*;
use std::time::{Instant};

fn main() {

    let ray_origin    = geo::Geo::point(0.0, 0.0, -5.0);
    let wall_z        = 10.0;
    let wall_size     = 20.0;
    let canvas_pixels = 1080;
    let pixel_size    = wall_size/(canvas_pixels as f64);
    let half          = wall_size/2.0; 

    let mut image = canvas::Canvas::new(canvas_pixels,canvas_pixels);
    let red       = color::Color::new(1.0, 0.0, 0.0);
    let mut shape = ray::Sphere::unit();
    shape.set_transform(matrix::Matrix::shear(-1.0, 0.0, 0.0, 0.0, 0.0, 0.5));

    println!("         STARTING RENDER");
    println!("==================================");
    let now = Instant::now();

    // NEW RAYON METHOD
    let mut idx:Vec<(usize,usize)> = Vec::new();

    for y in 0..canvas_pixels-1 {
        for x in 0..canvas_pixels-1 {
            idx.push((x,y));
        }
    }

    let ray_vec:Vec<bool> = idx.par_iter()
        .map(|(x,y)| {

            let world_y = half - pixel_size*(*y as f64);
            let world_x = -half + pixel_size*(*x as f64);

            let position = geo::Geo::point(world_x, world_y, wall_z);
            let r = ray::Ray::new(ray_origin, position-ray_origin);

            // TODO: change to a reference function instead of cloning
            let xs = r.intersect(shape.clone());
            ray::Isect::hit(xs).id >= 0 
        })
        .collect();

    println!("{} milliseconds elapsed", now.elapsed().as_millis());
    println!("{} pixels calculated",ray_vec.len());

    let num_hits:i32 = idx.iter()
        .zip(ray_vec.iter())
        .map(|((x,y),hit)| {

            if *hit {
                image.write_pixel(*x, *y, red);
                1
            } else { 0 }
        })
        .sum();

    println!("{} hits detected",num_hits);

    image.to_ppm("ppm/sphere.ppm");
    println!("==================================");
    println!("         RENDER COMPLETE");

}
