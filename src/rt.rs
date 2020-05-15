
pub fn point (x:f64, y:f64, z:f64) -> (f64, f64, f64, f64) {
    (x,y,z,1.0)
}

pub fn vector (x:f64, y:f64, z:f64) -> (f64, f64, f64, f64) {
    (x,y,z,0.0)
}

fn is_point (p:(f64,f64,f64,f64)) -> bool {
    p.3 == 1.0
}

fn is_vector (p:(f64,f64,f64,f64)) -> bool {
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
    fn equality() {
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
}

