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

    println!("starting render");
    let now = Instant::now();

    // OLD SEQUENTIAL WAY
    /*
    for y in 0..(canvas_pixels-1) {

        let world_y = half - pixel_size*(y as f64);
        
        for x in 0..(canvas_pixels-1) {

            let world_x = -half + pixel_size*(x as f64);

            let position = geo::Geo::point(world_x, world_y, wall_z);
            let r = ray::Ray::new(ray_origin, position-ray_origin);

            // TODO: change to a reference function
            let xs = r.intersect(shape.clone());
            if ray::Isect::hit(xs).id >= 0 {
                image.write_pixel(x, y, red);
            }

        }

    }
    */

    // NEW RAYON METHOD
    let mut idx:Vec<(usize,usize)> = Vec::new();

    for y in 0..canvas_pixels-1 {
        for x in 0..canvas_pixels-1 {
            idx.push((x,y));
        }
    }

    let ray_vec:Vec<bool> = idx.par_iter()
        .map(|(y,x)| {

            let world_y = half - pixel_size*(*y as f64);
            let world_x = -half + pixel_size*(*x as f64);

            let position = geo::Geo::point(world_x, world_y, wall_z);
            let r = ray::Ray::new(ray_origin, position-ray_origin);

            // TODO: change to a reference function instead of cloning
            let xs = r.intersect(shape.clone());
            ray::Isect::hit(xs).id >= 0 
        })
        .collect();

        println!("{}",ray_vec.len());

    println!("{}", now.elapsed().as_millis());

    //image.to_ppm("ppm/sphere.ppm");

    println!("render complete");

}
