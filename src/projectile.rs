use crate::rtx;

#[derive(Debug,Copy,Clone)]
pub struct Environment {
     pub gravity: rtx::Geo,
     pub wind   : rtx::Geo
}

#[derive(Debug,Copy,Clone)]
pub struct Projectile {
    pub position : rtx::Geo,
    pub velocity : rtx::Geo 
}

pub fn tick(env:Environment, proj:Projectile) -> Projectile {

    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;

    Projectile { position:pos, velocity: vel}
}

pub fn simulate(p:Projectile, e:Environment) {
    let mut projectile = p;
    
    while projectile.position.y > 0.0 {
        println!("Projectile position = {:?}",projectile.position);
        projectile = tick(e,projectile);
    }
    println!("Projectile crashed!");
}
