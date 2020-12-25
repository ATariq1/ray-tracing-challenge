use crate::geo;
use crate::matrix;
use std::sync::atomic::{AtomicI32,Ordering};
use std::cmp::Ordering as Order;

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

            return vec![Isect::isect(t1,s.id),
                        Isect::isect(t2,s.id)]; 
        }

    }

    pub fn transform(&self,m:matrix::Matrix) -> Ray {

        return Ray::new(m.clone()*self.orig, m.clone()*self.dir);
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
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Sphere {}

#[derive(Debug,Copy,Clone)]
pub struct Isect {
    pub t: f64,
    pub id:i32
}

impl Isect {

    pub fn isect(t:f64,id:i32) -> Isect {

        Isect {t:t, id:id }
    }

    pub fn hit(isects:Vec<Isect>) -> Isect {

        let ret = isects.iter()
                    .filter(|i| i.t > 0.0)
                    .min();

        return *ret.unwrap_or(&Isect::isect(0.0,-1));
    }
}


impl PartialOrd for Isect {

    fn partial_cmp(&self,other: &Self) -> Option<Order> {
        if self.t < other.t { Some(Order::Less)} else { Some(Order::Greater)}
    }
}

impl Ord for Isect {

    fn cmp(&self,other: &Self) -> Order {
        if self.t < other.t { Order::Less} else { Order::Greater}
    }
}

impl PartialEq for Isect {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.id == other.id
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
        let i = Isect::isect(3.5,s.id);

        assert_eq!(i.t,3.5);
        assert_eq!(i.id,s.id);
    }

    #[test]
    fn test_isect_struct() {

        let s = Sphere::unit();
        let i1 = Isect::isect(1.0,s.id);
        let i2 = Isect::isect(2.0,s.id);

        assert_eq!(i1.id,s.id);
        assert_eq!(i2.id,s.id);

    }

    #[test]
    fn test_isect3() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].id,s.id);
        assert_eq!(xs[1].id,s.id);
    }

    #[test]
    fn test_hit_all_positive() {

        let s = Sphere::unit();

        let i1 = Isect::isect(1.0,s.id);
        let i2 = Isect::isect(2.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        assert_eq!(i,i1);
    }

    #[test]
    fn test_hit_some_negative() {

        let s = Sphere::unit();

        let i1 = Isect::isect(-1.0,s.id);
        let i2 = Isect::isect(1.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        assert_eq!(i,i2);
    }

    #[test]
    fn test_hit_all_negative() {

        let s = Sphere::unit();

        let i1 = Isect::isect(-2.0,s.id);
        let i2 = Isect::isect(-1.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        // default id for no objects intersected is -1
        assert_eq!(i.id,-1);
    }

    
    #[test]
    fn test_hit_many() {

        let s = Sphere::unit();

        let i1 = Isect::isect( 5.0,s.id);
        let i2 = Isect::isect( 7.0,s.id);
        let i3 = Isect::isect(-3.0,s.id);
        let i4 = Isect::isect( 2.0,s.id);

        let xs = vec![i1,i2,i3,i4];

        let i = Isect::hit(xs);

        assert_eq!(i,i4);
    }
}

#[test]
fn test_translate_ray() {

    let r = Ray::new(
        geo::Geo::point( 1.0, 2.0, 3.0),
        geo::Geo::vector(0.0, 1.0, 0.0));

    let m = matrix::Matrix::translate(3.0, 4.0, 5.0);

    let r2 = r.transform(m);

    assert_eq!(r2.orig,geo::Geo::point(4.0, 6.0, 8.0));
    assert_eq!(r2.dir,geo::Geo::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_scale_ray() {

    let r = Ray::new(
        geo::Geo::point( 1.0, 2.0, 3.0),
        geo::Geo::vector(0.0, 1.0, 0.0));

    let m = matrix::Matrix::scale(2.0, 3.0, 4.0);

    let r2 = r.transform(m);

    assert_eq!(r2.orig,geo::Geo::point(2.0, 6.0, 12.0));
    assert_eq!(r2.dir,geo::Geo::vector(0.0, 3.0, 0.0));
}


