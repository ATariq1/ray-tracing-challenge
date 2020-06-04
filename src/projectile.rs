use crate::geo;
use crate::canvas;
use crate::color;

#[derive(Debug,Copy,Clone)]
pub struct Environment {
     pub gravity: geo::Geo,
     pub wind   : geo::Geo
}

#[derive(Debug,Copy,Clone)]
pub struct Projectile {
    pub position : geo::Geo,
    pub velocity : geo::Geo 
}

pub fn tick(env:Environment, proj:Projectile) -> Projectile {

    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;

    Projectile { position:pos, velocity: vel}
}

pub fn simulate(p:Projectile, e:Environment,mut c:canvas::Canvas) {
    let mut projectile = p;
    
    let green = color::Color::new(0.0,1.0,0.0);

    while projectile.position.y > 0.0 {
        
        c.write_pixel(projectile.position.x as usize,
                      c.get_height() - projectile.position.y as usize,
                      green);

        projectile = tick(e,projectile);
    }

    c.to_ppm("ppm/projectile.ppm");
    println!("Projectile crashed!");
}
