mod geo;
mod color;
mod canvas;
mod matrix;
mod projectile;

fn main() {

    let start    = geo::Geo::point( 0.0, 200.0, 0.0);
    let velocity = geo::Geo::vector(1.0, 0.0, 0.0).norm()*11.25;

    let p = projectile::Projectile  { position: start,
                                      velocity: velocity};

    let e = projectile::Environment { gravity:  geo::Geo::vector(0.0,-0.1, 0.0),
                                      wind:     geo::Geo::vector(-0.01,0.0,0.0) };

    let mut c = canvas::Canvas::new(900,550);

    projectile::simulate(p,e,c);

}
