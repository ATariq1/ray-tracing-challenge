type Geometry = (f64,f64,f64,f64);

pub fn add(lhs:Geometry,rhs:Geometry) -> Geometry {
        (lhs.0 + rhs.0,
         lhs.1 + rhs.1,
         lhs.2 + rhs.2,
         lhs.3 + rhs.3)
}

pub fn sub(lhs:Geometry,rhs:Geometry) -> Geometry {
        (lhs.0 - rhs.0,
         lhs.1 - rhs.1,
         lhs.2 - rhs.2,
         lhs.3 - rhs.3)
}

pub fn neg(t:Geometry) -> Geometry {
    (-t.0,
     -t.1,
     -t.2,
     -t.3)
}

pub fn cmul(a:f64,g:Geometry) -> Geometry {
    (a*g.0, a*g.1, a*g.2, a*g.3)
}

pub fn cdiv(d:f64,g:Geometry) -> Geometry {
    let a = 1.0/d;
    (a*g.0, a*g.1, a*g.2, a*g.3)
}

pub fn point (x:f64, y:f64, z:f64) -> Geometry {
    (x,y,z,1.0)
}

pub fn vector (x:f64, y:f64, z:f64) -> Geometry {
    (x,y,z,0.0)
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

        let p = (4.3,-4.2,3.1,1.0);

        assert_eq!(is_point(p),  true);
        assert_eq!(is_vector(p), false);
    }

    #[test]
    fn test_tup_is_vector() {
        let v = (4.3,-4.2,3.1,0.0);

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
        let tp = (1.0,-1.0,2.0,1.0);
        let tv = (1.0,-1.0,2.0,0.0);
        
        assert_eq!(v,tv);
        
        assert_ne!(v,p);
        assert_ne!(v,tp);
        
        assert_eq!(p,tp);
        
        assert_ne!(p,v);
        assert_ne!(p,tv);
    }

    #[test]
    fn test_addition() {
        let a1 = (3.0,-2.0,5.0,1.0);
        let a2 = (-2.0,3.0,1.0,0.0);

        let result = add(a1,a2);

        assert_eq!(result,(1.0,1.0,6.0,1.0));
    }

    #[test]
    fn test_subtraction1() {
        let a1 = point(3.0,2.0,1.0);
        let a2 = point(5.0,6.0,7.0);

        let result = sub(a1,a2);

        assert_eq!(result,(-2.0,-4.0,-6.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_subtraction2() {
        let a1 = point(3.0,2.0,1.0);
        let a2 = vector(5.0,6.0,7.0);

        let result = sub(a1,a2);

        assert_eq!(result,(-2.0,-4.0,-6.0,1.0));
        assert!(is_point(result));
    }

    #[test]
    fn test_subtraction3() {
        let a1 = vector(3.0,2.0,1.0);
        let a2 = vector(5.0,6.0,7.0);

        let result = sub(a1,a2);

        assert_eq!(result,(-2.0,-4.0,-6.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_negate1() {
        let z  = vector(0.0,0.0,0.0);
        let a2 = vector(1.0,-2.0,-3.0);

        let result = sub(z,a2);

        assert_eq!(result,(-1.0,2.0,3.0,0.0));
        assert!(is_vector(result));
    }

    #[test]
    fn test_negate2() {
        let a = (1.0,-2.0,-3.0,-4.0);

        let result = neg(a);

        assert_eq!(result,(-1.0,2.0,3.0,4.0));
    }

    #[test]
    fn test_cmul1() {
        let a = (1.0,-2.0,3.0,-4.0);
        let result = cmul(3.5,a);

        assert_eq!(result,(3.5,-7.0,10.5,-14.0));
    }

    #[test]
    fn test_cmul2() {
        let a = (1.0,-2.0,3.0,-4.0);
        let result = cmul(0.5,a);

        assert_eq!(result,(0.5,-1.0,1.5,-2.0));
    }
}

