use std::ops;

const VECTOR_W:f64 = 0.0;
const  POINT_W:f64 = 1.0;
const  EPSILON:f64 = 0.0000001;

#[derive(Debug,Copy,Clone)]
pub struct Geo {
    pub x:f64,
    pub y:f64,
    pub z:f64,
    pub w:f64
}

impl Geo {

    pub fn new(x:f64, y:f64, z:f64, w:f64) -> Geo {
        Geo {x:x, y:y, z:z, w:w}
    }

    pub fn point (x:f64, y:f64, z:f64) -> Geo {
        Geo {x:x, y:y, z:z, w:POINT_W}
    }                
                 
    pub fn vector (x:f64, y:f64, z:f64) -> Geo {
        Geo {x:x, y:y, z:z, w:VECTOR_W}
    }

    pub fn is_point (&self) -> bool {
        self.w == POINT_W
    }

    pub fn is_vector (&self) -> bool {
        self.w == VECTOR_W
    }

    pub fn len(&self) -> f64 {
        let s = self.x*self.x +
                self.y*self.y +
                self.z*self.z +
                self.w*self.w;
        s.sqrt()
    }

    pub fn norm(&self) -> Geo {
        let mag = self.len();
        return *self/mag;
    }
    
    pub fn dot(&self, other:Self) -> f64 {
        self.x*other.x +
        self.y*other.y +
        self.z*other.z +
        self.w*other.w 
    }

    pub fn cross(&self, other:Self) -> Geo {
       Geo::vector( self.y*other.z - self.z*other.y,
                    self.z*other.x - self.x*other.z,
                    self.x*other.y - self.y*other.x)
    }
}

impl ops::Add for Geo {
    type Output = Geo;

    fn add(self,rhs:Geo) -> Geo {
       Geo::new(self.x + rhs.x,
                self.y + rhs.y,
                self.z + rhs.z,
                self.w + rhs.w)
    }
}


impl ops::Sub for Geo {
    type Output = Geo;

    fn sub(self,rhs:Geo) -> Geo {
        Geo::new(self.x - rhs.x,
                 self.y - rhs.y,
                 self.z - rhs.z,
                 self.w - rhs.w)
    }
}

impl ops::Neg for Geo {
    type Output = Geo;

    fn neg(self) -> Geo {
        Geo::new(-self.x, -self.y, -self.z, -self.w)
    }                
}                    
                     
impl ops::Mul<f64> for Geo {
    type Output = Geo;

    fn mul(self,rhs:f64) -> Geo {
       Geo::new(self.x * rhs,
                self.y * rhs,
                self.z * rhs,
                self.w * rhs)
    }
}

impl ops::Mul<Geo> for f64 {
    type Output = Geo;

    fn mul(self,rhs:Geo) -> Geo {
       Geo::new(self * rhs.x,
                self * rhs.y,
                self * rhs.z,
                self * rhs.w)
    }
}

impl ops::Div<f64> for Geo {
    type Output = Geo;

    fn div(self,rhs:f64) -> Geo {
        Geo::new(self.x / rhs,
                 self.y / rhs,
                 self.z / rhs,
                 self.w / rhs)
    }
}

impl PartialEq for Geo {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON &&
        (self.y - other.y).abs() < EPSILON &&
        (self.z - other.z).abs() < EPSILON &&
        (self.w - other.w).abs() < EPSILON 
    }
}



impl Eq for Geo {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tup_is_point () {

        let p = Geo::new(4.3,-4.2,3.1,1.0);

        assert_eq!(p.is_point(),  true);
        assert_eq!(p.is_vector(), false);
    }

    #[test]
    fn test_tup_is_vector() {
        let v = Geo::new(4.3,-4.2,3.1,0.0);

        assert_eq!(v.is_point(),  false);
        assert_eq!(v.is_vector(), true);
    }
    
    #[test]
    fn test_point_w_1() {
        let t = Geo::point(1.0,-5.0,10.0);
        assert_eq!(t.w,POINT_W);
    }

    #[test]
    fn test_vector_w_0() {
        let t = Geo::vector(1.0,-5.0,10.0);
        assert_eq!(t.w,VECTOR_W);
    }

    #[test]
    fn test_equality() {
        let v  = Geo::vector(1.0, -1.0, 2.0);
        let p  = Geo::point( 1.0, -1.0, 2.0);
        let tp = Geo::new(1.0,-1.0,2.0,1.0);
        let tv = Geo::new(1.0,-1.0,2.0,0.0);
        
        assert_eq!(v,tv);
        
        assert_ne!(v,p);
        assert_ne!(v,tp);
        
        assert_eq!(p,tp);
        
        assert_ne!(p,v);
        assert_ne!(p,tv);
    }

    #[test]
    fn test_addition() {
        let a1 = Geo::new(3.0,-2.0,5.0,1.0);
        let a2 = Geo::new(-2.0,3.0,1.0,0.0);

        let result = a1 + a2;

        assert_eq!(result,Geo::new(1.0,1.0,6.0,1.0));
    }

    #[test]
    fn test_subtraction1() {
        let a1 = Geo::point(3.0,2.0,1.0);
        let a2 = Geo::point(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geo::new(-2.0,-4.0,-6.0,0.0));
        assert!(result.is_vector());
    }

    #[test]
    fn test_subtraction2() {
        let a1 = Geo::point(3.0,2.0,1.0);
        let a2 = Geo::vector(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geo::new(-2.0,-4.0,-6.0,1.0));
        assert!(result.is_point());
    }

    #[test]
    fn test_subtraction3() {
        let a1 = Geo::vector(3.0,2.0,1.0);
        let a2 = Geo::vector(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geo::new(-2.0,-4.0,-6.0,0.0));
        assert!(result.is_vector());
    }

    #[test]
    fn test_negate1() {
        let z  = Geo::vector(0.0,0.0,0.0);
        let a2 = Geo::vector(1.0,-2.0,-3.0);

        let result = z - a2;

        assert_eq!(result,Geo::new(-1.0,2.0,3.0,0.0));
        assert!(result.is_vector());
    }

    #[test]
    fn test_negate2() {
        let a = Geo::new(1.0,-2.0,-3.0,-4.0);

        let result = -a;

        assert_eq!(result,Geo::new(-1.0,2.0,3.0,4.0));
    }

    #[test]
    fn test_cmul1() {
        let a = Geo::new(1.0,-2.0,3.0,-4.0);
        let result = 3.5*a;

        assert_eq!(result,Geo::new(3.5,-7.0,10.5,-14.0));
    }

    #[test]
    fn test_cmul2() {
        let a = Geo::new(1.0,-2.0,3.0,-4.0);
        let result = 0.5 * a;

        assert_eq!(result,Geo::new(0.5,-1.0,1.5,-2.0));
    }

    #[test]
    fn test_cdiv() {
        let a = Geo::new(1.0,-2.0,3.0,-4.0);
        let result = a/2.0;
        assert_eq!(result,Geo::new(0.5,-1.0,1.5,-2.0));
    }

    #[test]
    fn test_magnitude() {
        let v1 = Geo::vector(1.0,0.0,0.0);
        assert_eq!(v1.len(),1.0);

        let v2 = Geo::vector(0.0,1.0,0.0);
        assert_eq!(v2.len(),1.0);

        let v3 = Geo::vector(0.0,0.0,1.0);
        assert_eq!(v3.len(),1.0);
        
        let v4 = Geo::vector(1.0,2.0,3.0);
        assert_eq!(v4.len(),14.0f64.sqrt());
        
        let v5 = Geo::vector(-1.0,-2.0,-3.0);
        assert_eq!(v5.len(),14.0f64.sqrt());
    }

    #[test]
    fn test_normalization() {
        let v1 = Geo::vector(4.0,0.0,0.0);
        assert_eq!(v1.norm(),Geo::vector(1.0,0.0,0.0));

        let v2 = Geo::vector(1.0,2.0,3.0);
        let mag = 14.0f64.sqrt();

        assert_eq!(v2.norm(),Geo::vector(1.0/mag, 2.0/mag, 3.0/mag));
        assert_eq!(v2.norm().len(),1.0)

    }

    #[test]
    fn test_dot() {
        let v1 = Geo::vector(1.0,2.0,3.0);
        let v2 = Geo::vector(2.0,3.0,4.0);

        let result1 = v1.dot(v2);
        let result2 = v2.dot(v1);

        assert_eq!(result1,result2);
        assert_eq!(result1,20.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Geo::vector(1.0,2.0,3.0);
        let v2 = Geo::vector(2.0,3.0,4.0);

        let result1 = v1.cross(v2);
        let result2 = v2.cross(v1);

        assert_eq!(result1,-result2);
        assert_eq!(result1,Geo::vector(-1.0,2.0,-1.0));
        assert_eq!(result2,Geo::vector(1.0,-2.0,1.0));
    }
}

