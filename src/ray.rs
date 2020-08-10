use crate::geo;
use std::sync::atomic::{AtomicI32, Ordering};

static SHAPE_ID:AtomicI32 = AtomicI32::new(0);

#[derive(Debug,Copy,Clone)]
pub struct Ray {

    pub orig:geo::Geo,
    pub dir:geo::Geo
}

impl Ray {

    pub fn new(origin:geo::Geo, direction:geo::Geo) -> Ray {
        Ray {orig:origin, dir:direction}
    }

    pub fn position(&self,t:f64) -> geo::Geo {

        self.orig + self.dir*t

    }
}

pub struct Sphere {

        id:i32,
    pub orig:geo::Geo,
    pub radius: f64
}

impl Sphere {

    pub fn unit() -> Sphere {
        
        let id = SHAPE_ID.fetch_add(1,Ordering::SeqCst);
        Sphere {id:id ,orig: geo::Geo::point(0.0,0.0,0.0), radius:1.0}
    }

    pub fn new(origin:geo::Geo, radius:f64 ) -> Sphere {

        let id = SHAPE_ID.fetch_add(1,Ordering::SeqCst);
        Sphere {id:id, orig:origin, radius:radius}

    }
}

pub struct Intersection {

    i32:count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray() {

        let origin    = geo::Geo::point( 1.0,2.0,3.0);
        let direction = geo::Geo::vector(4.0,5.0,6.0);

        let r = Ray::new(origin,direction);

        assert_eq!(r.orig,geo::Geo::point(1.0,2.0,3.0));
        assert_eq!(r.dir,geo::Geo::vector(4.0,5.0,6.0));

    }

    #[test]
    fn test_ray_dist() {

        let r = Ray::new(geo::Geo::point(2.0,3.0,4.0),geo::Geo::vector(1.0,0.0,0.0));
        
        assert_eq!(r.position(0.0), geo::Geo::point(2.0,3.0,4.0));
        assert_eq!(r.position(1.0), geo::Geo::point(3.0,3.0,4.0));
        assert_eq!(r.position(-1.0),geo::Geo::point(1.0,3.0,4.0));
        assert_eq!(r.position(2.5), geo::Geo::point(4.5,3.0,4.0));
    }

}


