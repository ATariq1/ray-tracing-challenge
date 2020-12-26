mod geo;
mod ray;
mod color;
mod canvas;
mod matrix;
mod projectile;

fn main() {

    let ray_origin    = geo::Geo::point(0.0, 0.0, -5.0);
    let wall_z        = 10.0;
    let wall_size     = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size/(canvas_pixels as f64);
    let half = wall_size/2.0; 

    let mut image = canvas::Canvas::new(canvas_pixels,canvas_pixels);
    let red   = color::Color::new(1.0, 0.0, 0.0);
    let mut shape = ray::Sphere::unit();
    shape.set_transform(matrix::Matrix::shear(-1.0, 0.0, 0.0, 0.0, 0.0, 0.5));

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


    image.to_ppm("ppm/sphere.ppm");

}
