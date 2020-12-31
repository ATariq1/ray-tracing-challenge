use crate::geo;
use crate::matrix;
use crate::material;
use std::f64::consts::PI;
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

        let r = self.transform(s.transform.inverse());
        let sphere_to_ray = r.orig - s.orig;
        let a = r.dir.dot(r.dir);
        let b = 2.0*r.dir.dot(sphere_to_ray);
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


#[derive(Debug,Clone)]
pub struct Sphere {

        id:i32,
    pub orig:geo::Geo,
    pub radius: f64,
    pub transform: matrix::Matrix,
    pub material: material::Material,
}


impl Sphere {

    pub fn unit() -> Sphere {
        
        let id = SHAPE_ID.fetch_add(1,Ordering::SeqCst);
        Sphere {id:id, 
                orig: geo::Geo::point(0.0,0.0,0.0),
                radius:1.0, transform:matrix::Matrix::identity(), 
                material:material::Material::default()
            }
    }

    pub fn set_transform(&mut self, m:matrix::Matrix) {
        self.transform = m;
    }


    pub fn normal_at(&self, wld_point:geo::Geo) -> geo::Geo {

        let obj_point  = self.transform.inverse()*wld_point;
        let obj_normal = obj_point - geo::Geo::point(0.0, 0.0, 0.0);
        let mut wld_normal = self.transform.inverse().transpose()*obj_normal;
        wld_normal.w = 0.0;

        return wld_normal.norm();

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
    fn ray() {

        let origin    = geo::Geo::point( 1.0,2.0,3.0);
        let direction = geo::Geo::vector(4.0,5.0,6.0);

        let r = Ray::new(origin,direction);

        assert_eq!(r.orig,geo::Geo::point(1.0,2.0,3.0));
        assert_eq!(r.dir,geo::Geo::vector(4.0,5.0,6.0));

    }

    #[test]
    fn ray_dist() {

        let r = Ray::new(geo::Geo::point(2.0,3.0,4.0),geo::Geo::vector(1.0,0.0,0.0));
        
        assert_eq!(r.position(0.0), geo::Geo::point(2.0,3.0,4.0));
        assert_eq!(r.position(1.0), geo::Geo::point(3.0,3.0,4.0));
        assert_eq!(r.position(-1.0),geo::Geo::point(1.0,3.0,4.0));
        assert_eq!(r.position(2.5), geo::Geo::point(4.5,3.0,4.0));
    }


    #[test]
    fn intersect1() {

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
    fn intersect2() {

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
    fn intersect3() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 2.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();

        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),0);
    
    }

    #[test]
    fn intersect4() {

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
    fn intersect5() {

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
    fn isect() {

        let s = Sphere::unit();
        let i = Isect::isect(3.5,s.id);

        assert_eq!(i.t,3.5);
        assert_eq!(i.id,s.id);
    }

    #[test]
    fn isect_struct() {

        let s = Sphere::unit();
        let i1 = Isect::isect(1.0,s.id);
        let i2 = Isect::isect(2.0,s.id);

        assert_eq!(i1.id,s.id);
        assert_eq!(i2.id,s.id);

    }

    #[test]
    fn isect3() {

        let r = Ray::new(
            geo::Geo::point( 0.0, 0.0,-5.0),
            geo::Geo::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let check = s.id;
        let xs = r.intersect(s);
        
        assert_eq!(xs.len(),2);
 
        assert_eq!(xs[0].id,check);
        assert_eq!(xs[1].id,check);
    }

    #[test]
    fn hit_all_positive() {

        let s = Sphere::unit();

        let i1 = Isect::isect(1.0,s.id);
        let i2 = Isect::isect(2.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        assert_eq!(i,i1);
    }

    #[test]
    fn hit_some_negative() {

        let s = Sphere::unit();

        let i1 = Isect::isect(-1.0,s.id);
        let i2 = Isect::isect(1.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        assert_eq!(i,i2);
    }

    #[test]
    fn hit_all_negative() {

        let s = Sphere::unit();

        let i1 = Isect::isect(-2.0,s.id);
        let i2 = Isect::isect(-1.0,s.id);

        let xs = vec![i1,i2];

        let i = Isect::hit(xs);

        // default id for no objects intersected is -1
        assert_eq!(i.id,-1);
    }

    
    #[test]
    fn hit_many() {

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
fn translate_ray() {

    let r = Ray::new(
        geo::Geo::point( 1.0, 2.0, 3.0),
        geo::Geo::vector(0.0, 1.0, 0.0));

    let m = matrix::Matrix::translate(3.0, 4.0, 5.0);

    let r2 = r.transform(m);

    assert_eq!(r2.orig,geo::Geo::point(4.0, 6.0, 8.0));
    assert_eq!(r2.dir,geo::Geo::vector(0.0, 1.0, 0.0));
}

#[test]
fn scale_ray() {

    let r = Ray::new(
        geo::Geo::point( 1.0, 2.0, 3.0),
        geo::Geo::vector(0.0, 1.0, 0.0));

    let m = matrix::Matrix::scale(2.0, 3.0, 4.0);

    let r2 = r.transform(m);

    assert_eq!(r2.orig,geo::Geo::point(2.0, 6.0, 12.0));
    assert_eq!(r2.dir,geo::Geo::vector(0.0, 3.0, 0.0));
}

#[test]
fn sphere_transform_setting() {

    let mut s = Sphere::unit();
    let t = matrix::Matrix::translate(2.0, 3.0, 4.0);
    s.set_transform(t.clone());

    assert_eq!(s.transform,t);
}

#[test]
fn scaled_intersection() {

    let r = Ray::new(
        geo::Geo::point( 0.0, 0.0,-5.0),
        geo::Geo::vector(0.0, 0.0, 1.0));

    let mut s = Sphere::unit();
    s.set_transform(matrix::Matrix::scale(2.0, 2.0, 2.0));

    let xs = r.intersect(s);
    
    assert_eq!(xs.len(),2);
    assert_eq!(xs[0].t,3.0);
    assert_eq!(xs[1].t,7.0);

}

#[test]
fn translated_intersection() {

    let r = Ray::new(
        geo::Geo::point( 0.0, 0.0,-5.0),
        geo::Geo::vector(0.0, 0.0, 1.0));

    let mut s = Sphere::unit();
    s.set_transform(matrix::Matrix::translate(5.0, 0.0, 0.0));

    let xs = r.intersect(s);
    
    assert_eq!(xs.len(),0);
}

#[test]
fn normal_x_axis() {
    let s = Sphere::unit();
    assert_eq!(s.normal_at(geo::Geo::point(1.0, 0.0, 0.0)),geo::Geo::vector(1.0, 0.0, 0.0));
}

#[test]
fn normal_y_axis() {
    let s = Sphere::unit();
    assert_eq!(s.normal_at(geo::Geo::point(0.0, 1.0, 0.0)),geo::Geo::vector(0.0, 1.0, 0.0));
}

#[test]
fn normal_z_axis() {
    let s = Sphere::unit();
    assert_eq!(s.normal_at(geo::Geo::point(0.0, 0.0, 1.0)),geo::Geo::vector(0.0, 0.0, 1.0));
}

#[test]
fn normal_non_axial() {
    let s = Sphere::unit();
    assert_eq!(s.normal_at(geo::Geo::point((3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0)),
                          geo::Geo::vector((3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0));
}

#[test]
fn normalized_normal() {
    let s = Sphere::unit();
    let n = s.normal_at(geo::Geo::point((3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0));
    assert_eq!(n.norm(),geo::Geo::vector((3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0,(3.0 as f64).sqrt()/3.0));
}

#[test]
fn translated_normal() {
    let mut s = Sphere::unit();
    s.set_transform(matrix::Matrix::translate(0.0, 1.0, 0.0));
    let n = s.normal_at(geo::Geo::point(0.0, 1.70711, -0.70711));
    assert_eq!(n,geo::Geo::vector(0.0, 0.70711, -0.70711));
}

#[test]
fn scaled_rotated_normal() {

    let mut s = Sphere::unit();
    let m = matrix::Matrix::scale(1.0, 0.5, 1.0) * matrix::Matrix::rotate_z(PI/5.0);
    s.set_transform(m);
    let n = s.normal_at(geo::Geo::point(0.0, 1.0/(2.0 as f64).sqrt(), -1.0/(2.0 as f64).sqrt()));
    assert_eq!(n,geo::Geo::vector(0.0, 0.97014, -0.24254));

}