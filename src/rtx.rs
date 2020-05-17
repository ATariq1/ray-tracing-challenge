
use std::ops;

#[derive(Debug,Copy,Clone)]
pub struct Geometry(f64,f64,f64,f64);

impl ops::Add for Geometry {
    type Output = Geometry;

    fn add(self,rhs:Geometry) -> Geometry {
       Geometry(self.0 + rhs.0,
                self.1 + rhs.1,
                self.2 + rhs.2,
                self.3 + rhs.3)
    }
}


impl ops::Sub for Geometry {
    type Output = Geometry;

    fn sub(self,rhs:Geometry) -> Geometry {
        Geometry(self.0 - rhs.0,
                 self.1 - rhs.1,
                 self.2 - rhs.2,
                 self.3 - rhs.3)
    }
}

impl ops::Neg for Geometry {
    type Output = Geometry;

    fn neg(self) -> Geometry {
        Geometry(-self.0, -self.1, -self.2, -self.3)
    }
}

impl ops::Mul<f64> for Geometry {
    type Output = Geometry;

    fn mul(self,rhs:f64) -> Geometry {
       Geometry(self.0 * rhs,
                self.1 * rhs,
                self.2 * rhs,
                self.3 * rhs)
    }
}

impl ops::Mul<Geometry> for f64 {
    type Output = Geometry;

    fn mul(self,rhs:Geometry) -> Geometry {
       Geometry(self * rhs.0,
                self * rhs.1,
                self * rhs.2,
                self * rhs.3)
    }
}

impl ops::Div<f64> for Geometry {
    type Output = Geometry;

    fn div(self,rhs:f64) -> Geometry {
        let rpl = 1.0/rhs;
        Geometry(self.0 * rpl,
                 self.1 * rpl,
                 self.2 * rpl,
                 self.3 * rpl)
    }
}

impl PartialEq for Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 &&
        self.1 == other.1 &&
        self.2 == other.2 &&
        self.3 == other.3 
    }
}



impl Eq for Geometry {}

pub fn point (x:f64, y:f64, z:f64) -> Geometry {
    Geometry(x,y,z,1.0)
}

pub fn vector (x:f64, y:f64, z:f64) -> Geometry {
   Geometry(x,y,z,0.0)
}

pub fn is_point (p:Geometry) -> bool {
    p.3 == 1.0
}

pub fn is_vector (p:Geometry) -> bool {
    p.3 == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tup_is_point () {

        let p = Geometry(4.3,-4.2,3.1,1.0);

        assert_eq!(is_point(p),  true);
        assert_eq!(is_vector(p), false);
    }

    #[test]
    fn test_tup_is_vector() {
        let v = Geometry(4.3,-4.2,3.1,0.0);

        assert_eq!(is_point(v),  false);
        assert_eq!(is_vector(v), true);
    }
    
    #[test]
    fn test_point_w_1() {
        let t = point(1.0,-5.0,10.0);
        assert_eq!(t.3,1.0);
    }

    #[test]
    fn test_vector_w_0() {
        let t = vector(1.0,-5.0,10.0);
        assert_eq!(t.3,0.0);
    }

    #[test]
    fn test_equality() {
        let v  = vector(1.0, -1.0, 2.0);
        let p  = point( 1.0, -1.0, 2.0);
        let tp = Geometry(1.0,-1.0,2.0,1.0);
        let tv = Geometry(1.0,-1.0,2.0,0.0);
        
        assert_eq!(v,tv);
        
        assert_ne!(v,p);
        assert_ne!(v,tp);
        
        assert_eq!(p,tp);
        
        assert_ne!(p,v);
        assert_ne!(p,tv);
    }

    #[test]
    fn test_addition() {
        let a1 = Geometry(3.0,-2.0,5.0,1.0);
        let a2 = Geometry(-2.0,3.0,1.0,0.0);

        let result = a1 + a2;

        assert_eq!(result,Geometry(1.0,1.0,6.0,1.0));
    }

    #[test]
    fn test_subtraction1() {
        let a1 = point(3.0,2.0,1.0);
        let a2 = point(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geometry(-2.0,-4.0,-6.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_subtraction2() {
        let a1 = point(3.0,2.0,1.0);
        let a2 = vector(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geometry(-2.0,-4.0,-6.0,1.0));
        assert!(is_point(result));
    }

    #[test]
    fn test_subtraction3() {
        let a1 = vector(3.0,2.0,1.0);
        let a2 = vector(5.0,6.0,7.0);

        let result = a1 - a2;

        assert_eq!(result,Geometry(-2.0,-4.0,-6.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_negate1() {
        let z  = vector(0.0,0.0,0.0);
        let a2 = vector(1.0,-2.0,-3.0);

        let result = z - a2;

        assert_eq!(result,Geometry(-1.0,2.0,3.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_negate2() {
        let a = Geometry(1.0,-2.0,-3.0,-4.0);

        let result = -a;

        assert_eq!(result,Geometry(-1.0,2.0,3.0,4.0));
    }

    #[test]
    fn test_cmul1() {
        let a = Geometry(1.0,-2.0,3.0,-4.0);
        let result = 3.5*a;

        assert_eq!(result,Geometry(3.5,-7.0,10.5,-14.0));
    }

    #[test]
    fn test_cmul2() {
        let a = Geometry(1.0,-2.0,3.0,-4.0);
        let result = 0.5 * a;

        assert_eq!(result,Geometry(0.5,-1.0,1.5,-2.0));
    }

    #[test]
    fn test_cdiv() {
        let a = Geometry(1.0,-2.0,3.0,-4.0);
        let result = a/2.0;
        assert_eq!(result,Geometry(0.5,-1.0,1.5,-2.0));
    }
}

