mod rtx;
mod color;
mod canvas;
mod projectile;

fn main() {

    let p = projectile::Projectile  { position: rtx::Geo::point( 0.0, 1.0, 0.0),
                                      velocity: rtx::Geo::vector(0.0, 10.0, 0.0) };

    let e = projectile::Environment { gravity:  rtx::Geo::vector(0.0,-0.1, 0.0),
                                      wind:     rtx::Geo::vector(-0.01,0.0,0.0) };

    projectile::simulate(p,e);

}
