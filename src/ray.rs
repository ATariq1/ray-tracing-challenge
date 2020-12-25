use crate::geo;
use std::sync::atomic::{AtomicI32, Ordering};
use std::cmp::Ordering;

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

    pub fn intersect(&self, s:Sphere) -> Vec<Isect> {

        let sphere_to_ray = self.orig - s.orig;
        let a = self.dir.dot(self.dir);
        let b = 2.0*self.dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return vec![]
        } else {

            let t1 = (-b - discriminant.sqrt())/(2.0*a);
            let t2 = (-b + discriminant.sqrt())/(2.0*a);

            return vec![Isect::isect(t1,s),
                        Isect::isect(t2,s)]; 
        }

    }
}


#[derive(Debug,Copy,Clone)]
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

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Sphere {}

#[derive(Debug,Copy,Clone)]
struct Isect {
    pub t: f64,
    pub object: Sphere
}

impl Isect {

    pub fn isect(t:f64,s:Sphere) -> Isect {

        Isect {t:t, object:s }
    }

    pub fn hit(isects:Vec<Isect>) -> Isect {

        let ret = isects.iter()
                    .filter(|i| i.t > 0.0)
                    .min();

        ret

    }
}


impl PartialOrd for Isect {

    fn cmp(&self,other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl Ord for Isect {

    fn cmp(&self,other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl PartialEq for Isect {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}
impl Eq for Isect {}



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


    #[test]
    fn test_intersect1() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,4.0);
        assert_eq!(xs[1].t,6.0);
    
    }

    #[test]
    fn test_intersect2() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 1.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,5.0);
        assert_eq!(xs[1].t,5.0);
    
    }

    #[test]
    fn test_intersect3() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 2.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),0);
    
    }

    #[test]
    fn test_intersect4() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0, 0.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,-1.0);
        assert_eq!(xs[1].t, 1.0);

    }

    #[test]
    fn test_intersect5() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0, 5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,-6.0);
        assert_eq!(xs[1].t,-4.0);
    
    }


    #[test]
    fn test_isect() {

        let s = Sphere::unit();
        let i = Isect::isect(3.5,s);

        assert_eq!(i.t,3.5);
        assert_eq!(i.object,s);
    }

    #[test]
    fn test_isect2() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].object,s);
        assert_eq!(xs[1].object,s);
    
    }



}


