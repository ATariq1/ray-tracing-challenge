mod rt;
mod rtx;

const ITER:i32  = 1_000_000;
const OUTER:u128 = 100;

use std::time::{Instant};

fn main() {

    let mut total = 0;

    for _t in 0..OUTER { 
    
        let now = Instant::now();
        let mut t = rt::point(1.0,-1.0,10.0);
        for x in 0..ITER {
            let y = x as f64;
            t = rt::add(t,rt::cmul(y,t));
        }
        total += now.elapsed().as_nanos();
    }

    println!("TUPLE AVG: {}",total/OUTER);

    let mut total = 0;

    for _t in 0..OUTER {
    
        let now = Instant::now();
        let mut t = rtx::point(1.0,-1.0,10.0);
        for x in 0..ITER {
            let y = x as f64;
            t = t + y*t;
        }
        total += now.elapsed().as_nanos();
    }

    println!("STRUCT AVG: {}",total/OUTER);
}
