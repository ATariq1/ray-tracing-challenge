mod rt;
mod rtx;
mod projectile;

const ITER:i32  = 25_000;
const OUTER:u128 = 1000;

use std::time::{Instant};

#[allow(dead_code)]
fn type_benchmark() {
    let mut total = 0;

    for _t in 0..OUTER { 
    
        let now = Instant::now();
        let mut t = rt::vector(1.0,-1.0,10.0);
        for x in 0..ITER {
            let y = x as f64;
            if rt::is_vector(t) {
                t = rt::cdiv(2.0,rt::add(t,rt::cmul(y,t)));
            }
        }

        total += now.elapsed().as_nanos();
    }

    println!("TUPLE AVG : {}",total/OUTER);

    let mut total = 0;

    for _t in 0..OUTER {
    
        let now = Instant::now();
        let mut t = rtx::Geo::vector(1.0,-1.0,10.0);
        for x in 0..ITER {
            let y = x as f64;
            if t.is_vector() {
                t = (t + y*t)/2.0;
            }
        }
        total += now.elapsed().as_nanos();
    }

    println!("STRUCT AVG: {}",total/OUTER);

}



fn main() {

    let p = projectile::Projectile  { position: rtx::Geo::point( 0.0, 1.0, 0.0),
                                      velocity: rtx::Geo::vector(10.0, 10.0, 0.0) };

    let e = projectile::Environment { gravity:  rtx::Geo::vector(0.0,-0.1, 0.0),
                                      wind:     rtx::Geo::vector(-0.01,0.0,0.0) };

    projectile::simulate(p,e);

}
