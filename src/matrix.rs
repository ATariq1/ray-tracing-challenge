use std::f64::consts::PI;
use std::ops;
use std::fmt;
use crate::geo;

#[derive(Clone)]
pub struct Matrix {
    dim : usize,
    matrix: Vec<f64>
}


impl Matrix {

    pub fn with_dim( dim:usize) -> Matrix {

        let matrix = vec!(0.0f64; dim*dim);
        Matrix {dim, matrix}
    }

    pub fn with_vec( matrix:Vec<f64>) -> Matrix {
        
        let dim = match matrix.len() {
            16 => 4,
            9  => 3,
            4  => 2,
            _  => 0,
        };

        Matrix {dim, matrix}
    }

    pub fn transpose(&self) -> Matrix {
        
        let mut ret = Matrix::with_dim(4);

        for row in 0..4 {
            for col in 0..4 {

                ret.set(col,row,self.get(row,col));
            }
        }

        ret
    }

    pub fn identity() -> Matrix {

        let dim = 4usize;
        let matrix = vec![1.0, 0.0, 0.0, 0.0,
                         0.0, 1.0, 0.0, 0.0,
                         0.0, 0.0, 1.0, 0.0,
                         0.0, 0.0, 0.0, 1.0];
 
        Matrix { dim, matrix}

    }

    pub fn get( &self,row:usize, col:usize) -> f64 {
        
        self.matrix[row*self.dim + col]
    }

    pub fn set(&mut self,row:usize, col:usize, val:f64) {

        self.matrix[row*self.dim + col] = val;

    }

    pub fn det(&self) -> f64 {

        if self.dim == 2 { 
            return self.get(0,0)*self.get(1,1) - self.get(0,1)*self.get(1,0);
        }

        let mut ret = 0.0;

        for col in 0..self.dim {

            ret += self.get(0,col)*self.cofactor(0,col);
        }

        return ret;

    }

    pub fn sub(&self, i:usize,j:usize) -> Matrix {

        let mut vec = Vec::new();

        for row in 0..self.dim {
            for col in 0..self.dim {
                if row != i && col != j {
                        
                    vec.push(self.get(row,col));
                }
            }
        }

        Matrix::with_vec(vec)

    }

    pub fn minor(&self, i:usize, j:usize) -> f64 {

        self.sub(i,j).det()

    }

    pub fn cofactor(&self, i:usize, j:usize) -> f64 {

        if (i + j) % 2 == 0 {
            return self.minor(i,j);
        } else {
            return self.minor(i,j)*-1.0;
        }

    }

    pub fn inverse(&self) -> Matrix {

        let det = self.det();
        let mut ret = Matrix::with_dim(self.dim);

        assert!(det != 0.0);
        
        for row in 0..self.dim {
            for col in 0..self.dim {
                let c   = self.cofactor(row,col);
                let val = c/det;
                ret.set(col,row,val);
            }
        }

        ret
    }

    pub fn translate(x:f64,y:f64,z:f64) -> Matrix {

        let vec = vec![1.0, 0.0, 0.0, x,
                       0.0, 1.0, 0.0, y,
                       0.0, 0.0, 1.0, z,
                       0.0, 0.0, 0.0, 1.0];

        Matrix { dim:4, matrix:vec}

    }

    pub fn scale(x:f64,y:f64,z:f64) -> Matrix {

        let vec = vec![  x, 0.0, 0.0, 0.0,
                       0.0,   y, 0.0, 0.0,
                       0.0, 0.0,   z, 0.0,
                       0.0, 0.0, 0.0, 1.0];

        Matrix { dim:4, matrix:vec}

    }

    pub fn rotate_x(r:f64) -> Matrix {

        let vec = vec![1.0, 0.0,     0.0,      0.0,
                       0.0, r.cos(),-r.sin(),  0.0,
                       0.0, r.sin(), r.cos(),  0.0,
                       0.0, 0.0,     0.0,      1.0];

        Matrix { dim:4, matrix:vec}
    }

    pub fn rotate_y(r:f64) -> Matrix {

        let vec = vec![r.cos(), 0.0, r.sin(), 0.0,
                       0.0,     1.0, 0.0,     0.0,
                      -r.sin(), 0.0, r.cos(), 0.0,
                       0.0,     0.0, 0.0,     1.0];

        Matrix { dim:4, matrix:vec}
    }

    pub fn rotate_z(r:f64) -> Matrix {

        let vec = vec![r.cos(),-r.sin(), 0.0, 0.0,
                       r.sin(), r.cos(), 0.0, 0.0,
                       0.0,     0.0,     1.0, 0.0,
                       0.0,     0.0,     0.0, 1.0];

        Matrix { dim:4, matrix:vec}
    }

    pub fn shear(xy:f64,xz:f64,yx:f64,yz:f64,zx:f64,zy:f64) -> Matrix {

        let vec = vec![1.0,  xy,  xz, 0.0,
                        yx, 1.0,  yz, 0.0,
                        zx,  zy, 1.0, 0.0,
                       0.0, 0.0, 0.0, 1.0];

        Matrix { dim:4, matrix:vec}
    }
}
                      
impl ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self,rhs:Matrix) -> Matrix {

        let mut ret = Matrix::with_dim(4);

        for row in 0..4 {
            for col in 0..4 {
                    
                let prod = self.get(row,0) * rhs.get(0,col) +
                           self.get(row,1) * rhs.get(1,col) +
                           self.get(row,2) * rhs.get(2,col) + 
                           self.get(row,3) * rhs.get(3,col);

       		ret.set(row,col,prod);
	     }
	}

        return ret;
    }
}

impl ops::Mul<geo::Geo> for Matrix {
    type Output = geo::Geo;

    fn mul(self,rhs:geo::Geo) -> geo::Geo {

        let mut ret = geo::Geo::new(0.0,0.0,0.0,0.0);

        ret.x = self.get(0,0)*rhs.x + self.get(0,1)*rhs.y + self.get(0,2)*rhs.z  + self.get(0,3)*rhs.w;
        ret.y = self.get(1,0)*rhs.x + self.get(1,1)*rhs.y + self.get(1,2)*rhs.z  + self.get(1,3)*rhs.w;
        ret.z = self.get(2,0)*rhs.x + self.get(2,1)*rhs.y + self.get(2,2)*rhs.z  + self.get(2,3)*rhs.w;
        ret.w = self.get(3,0)*rhs.x + self.get(3,1)*rhs.y + self.get(3,2)*rhs.z  + self.get(3,3)*rhs.w;
        
        ret
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matrix")
         .field("dim", &self.dim)
         .field("matrix", &self.matrix.iter().fold(String::new(), |acc, &arg| acc + "   " + &arg.to_string()))
         .finish()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
    
        self.dim == rhs.dim && 
        self.matrix.iter().zip(&rhs.matrix).all(|(a,b)| (a-b).abs() < geo::EPSILON)
    
    }
}

impl Eq for Matrix {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_matrix () {

        let m = Matrix::with_vec(vec![1.0, 2.0, 3.0, 4.0,
                                      5.5, 6.5, 7.5, 8.5,
                                      9.0, 10.0,11.0,12.0,
                                      13.5,14.5,15.5,16.5]);

        assert_eq!(m.get(0,0),1.0);
        assert_eq!(m.get(0,3),4.0);
        assert_eq!(m.get(1,0),5.5);
        assert_eq!(m.get(1,2),7.5);
        assert_eq!(m.get(2,2),11.0);
        assert_eq!(m.get(3,0),13.5);
        assert_eq!(m.get(3,2),15.5);
    }

    #[test]
    fn test_3x3_matrix () {

        let m = Matrix::with_vec(vec![-3.0, 5.0, 0.0,
                                       1.0,-2.0,-7.0,
                                       0.0, 1.0, 1.0]);

        assert_eq!(m.get(0,0),-3.0);
        assert_eq!(m.get(1,1),-2.0);
        assert_eq!(m.get(2,2), 1.0);
    }

    #[test]
    fn test_2x2_matrix () {

        let m = Matrix::with_vec(vec![-3.0, 5.0,
                                       1.0,-2.0]);

        assert_eq!(m.get(0,0),-3.0);
        assert_eq!(m.get(0,1), 5.0);
        assert_eq!(m.get(1,0), 1.0);
        assert_eq!(m.get(1,1),-2.0);
    }

    #[test]
    fn test_matrix_equality () {

        let a = Matrix::with_vec(vec![1.0,2.0,3.0,4.0,
                                      5.0,6.0,7.0,8.0,
                                      9.0,8.0,7.0,6.0,
                                      5.0,4.0,3.0,2.0]);

        let b = Matrix::with_vec(
                vec![1.0,2.0,3.0,4.0,
                     5.0,6.0,7.0,8.0,
                     9.0,8.0,7.0,6.0,
                     5.0,4.0,3.0,2.0]);

        assert_eq!(a,b);
        
    }

    #[test]
    fn test_matrix_multiply () {

        let a = Matrix::with_vec(
                vec![1.0,2.0,3.0,4.0,
                     5.0,6.0,7.0,8.0,
                     9.0,8.0,7.0,6.0,
                     5.0,4.0,3.0,2.0]);

        let b = Matrix::with_vec(
                vec![-2.0, 1.0, 2.0, 3.0,
                      3.0, 2.0, 1.0,-1.0,
                      4.0, 3.0, 6.0, 5.0,
                      1.0, 2.0, 7.0, 8.0]);

        let result = a*b;

        let exp = Matrix::with_vec(
                  vec![20.0, 22.0, 50.0, 48.0,
                       44.0, 54.0,114.0,108.0,
                       40.0, 58.0,110.0,102.0,
                       16.0, 26.0, 46.0, 42.0]);

        assert_eq!(result,exp);
        
    }

    #[test]
    fn test_matrix_vector_multiply () {

        let a = Matrix::with_vec(
                vec![1.0, 2.0, 3.0, 4.0,
                     2.0, 4.0, 4.0, 2.0,
                     8.0, 6.0, 4.0, 1.0,
                     0.0, 0.0, 0.0, 1.0]);

        let v = geo::Geo::new(1.0,2.0,3.0,1.0);

        assert_eq!(a*v, geo::Geo::new(18.0,24.0,33.0,1.0));

    }


    #[test]
    fn test_matrix_identity_multiply () {

        let a = Matrix::with_vec(
                vec![0.0, 1.0, 2.0, 4.0,
                     1.0, 2.0, 4.0, 8.0,
                     2.0, 4.0, 8.0, 16.0,
                     4.0, 8.0, 16.0, 32.0]);

        let i = Matrix::identity();

        let r1 = a.clone()*i;

        assert_eq!(r1, a);

        let b = geo::Geo::new(1.0,2.0,3.0,4.0);
        
        let r2 = Matrix::identity() * b;

        assert_eq!(r2,b);
    }

    #[test]
    fn test_transpose () {

        let a = Matrix::with_vec(
                vec![0.0, 9.0, 3.0, 0.0,
                     9.0, 8.0, 0.0, 8.0,
                     1.0, 8.0, 5.0, 3.0,
                     0.0, 0.0, 5.0, 8.0]);

        let t = a.transpose();

        let c = Matrix::with_vec(
            vec![0.0, 9.0, 1.0, 0.0,
                 9.0, 8.0, 8.0, 0.0,
                 3.0, 0.0, 5.0, 5.0,
                 0.0, 8.0, 3.0, 8.0]);

        assert_eq!(t,c);

        assert_eq!(Matrix::identity(), Matrix::identity().transpose());
    }

    #[test]
    fn test_det () {
    
        let a = Matrix::with_vec(
                vec![1.0,5.0,-3.0,2.0]);


        assert_eq!(a.det(),17.0);

    }

    #[test]
    fn test_submatrix () {

        let a = Matrix::with_vec(
                vec![ 1.0, 5.0, 0.0,
                     -3.0, 2.0, 7.0,
                      0.0, 6.0, -3.0]);
        
        let e1= Matrix::with_vec(
                vec![-3.0, 2.0,
                      0.0, 6.0]);

        assert_eq!(a.sub(0,2),e1);    


        let b = Matrix::with_vec(
                vec![-6.0, 1.0, 1.0, 6.0,
                     -8.0, 5.0, 8.0, 6.0,
                     -1.0, 0.0, 8.0, 2.0,
                     -7.0, 1.0,-1.0, 1.0]);

        let e2= Matrix::with_vec(
                vec![-6.0, 1.0, 6.0,
                     -8.0, 8.0, 6.0,
                     -7.0,-1.0, 1.0]);

        assert_eq!(b.sub(2,1),e2)
    }

    #[test]
    fn test_minor () {

        let a = Matrix::with_vec(
                vec![ 3.0, 5.0, 0.0,
                      2.0,-1.0,-7.0,
                      6.0,-1.0, 5.0]);

        assert_eq!(a.minor(1,0),25.0)
    }

    #[test]
    fn test_cofactor () {

        let a = Matrix::with_vec(
                vec![ 3.0, 5.0, 0.0,
                      2.0,-1.0,-7.0,
                      6.0,-1.0, 5.0]);

        assert_eq!(a.minor(0,0),   -12.0);
        assert_eq!(a.cofactor(0,0),-12.0);
        assert_eq!(a.minor(1,0),    25.0);
        assert_eq!(a.cofactor(1,0),-25.0);
    }

    #[test]
    fn test_3x3_determinant () {

        let a = Matrix::with_vec(
                vec![ 1.0, 2.0, 6.0,
                     -5.0, 8.0,-4.0,
                      2.0, 6.0, 4.0]);

        assert_eq!(a.cofactor(0,0), 56.0);
        assert_eq!(a.cofactor(0,1), 12.0);
        assert_eq!(a.cofactor(0,2),-46.0);
        assert_eq!(a.det(),-196.0);
    }

    #[test]
    fn test_4x4_determinant () {

        let a = Matrix::with_vec(
                vec![-2.0,-8.0, 3.0, 5.0,
                     -3.0, 1.0, 7.0, 3.0,
                      1.0, 2.0,-9.0, 6.0,
                     -6.0, 7.0, 7.0,-9.0]);

        assert_eq!(a.cofactor(0,0), 690.0);
        assert_eq!(a.cofactor(0,1), 447.0);
        assert_eq!(a.cofactor(0,2), 210.0);
        assert_eq!(a.cofactor(0,3), 51.0);
        assert_eq!(a.det(),-4071.0);
    }

    #[test]
    fn test_invertible () {

        let a = Matrix::with_vec(
                vec![ 6.0, 4.0, 4.0, 4.0,
                      5.0, 5.0, 7.0, 6.0,
                      4.0,-9.0, 3.0,-7.0,
                      9.0, 1.0, 7.0,-6.0]);

        assert_eq!(a.det(),-2120.0);
        
        let a = Matrix::with_vec(
                vec![-4.0, 2.0,-2.0,-3.0,
                      9.0, 6.0, 2.0, 6.0,
                      0.0,-5.0, 1.0,-5.0,
                      0.0, 0.0, 0.0, 0.0]);

        assert_eq!(a.det(),0.0);

    }


    #[test]
    fn test_inverse0 () {

        let a = Matrix::with_vec(
                vec![-5.0, 2.0, 6.0,-8.0,
                      1.0,-5.0, 1.0, 8.0,
                      7.0, 7.0,-6.0,-7.0,
                      1.0,-3.0, 7.0, 4.0]);

        assert_eq!(a.det(),532.0);
        assert_eq!(a.cofactor(2,3),-160.0);
        
        let b = a.inverse();

        assert_eq!(b.get(3,2),-160.0/532.0);
        assert_eq!(a.cofactor(3,2),105.0);
        assert_eq!(b.get(2,3),105.0/532.0);

        assert_eq!(a * b,Matrix::identity());
    }

    #[test]
    fn test_inverse1 () {

        let a = Matrix::with_vec(
                vec![ 4.0, 0.0, 0.0, 0.0,
                      0.0, 0.0, 2.0, 0.0,
                      0.0, 1.0, 2.0, 0.0,
                      1.0, 0.0, 0.0, 1.0]);

        let e = Matrix::with_vec(
                vec![ 0.25, 0.0, 0.0, 0.0,
                      0.0, -1.0, 1.0, 0.0,
                      0.0,  0.5, 0.0, 0.0,
                     -0.25, 0.0, 0.0, 1.0]);

        assert_eq!(a.inverse(),e);
    }



    #[test]
    fn test_inverse2 () {

        let a = Matrix::with_vec(
                vec![ 8.0,-5.0, 9.0, 2.0,
                      7.0, 5.0, 6.0, 1.0,
                     -6.0, 0.0, 9.0, 6.0,
                     -3.0, 0.0,-9.0,-4.0]);

        let e = Matrix::with_vec(
                vec![ -0.15385, -0.15385, -0.28205, -0.53846,
                      -0.07692,  0.12308,  0.02564,  0.03077,
                       0.35897,  0.35897,  0.43590,  0.92308,
                      -0.69231, -0.69231, -0.76923, -1.92308]);

        let b = a.inverse();

        assert_eq!(b,e);
        assert_eq!(a*b,Matrix::identity());
    }

    #[test]
    fn test_translate0 () {

        let t = Matrix::translate(5.0, -3.0, 2.0);
        let p = geo::Geo::point(-3.0, 4.0, 5.0);

        assert_eq!(t*p, geo::Geo::point(2.0,1.0,7.0));

    }

    #[test]
    fn test_translate1 () {

        let t0 = Matrix::translate(5.0, -3.0, 2.0);
        let t1 = t0.inverse();
        let p = geo::Geo::point(-3.0, 4.0, 5.0);

        assert_eq!(t1*p, geo::Geo::point(-8.0,7.0,3.0));

    }

    #[test]
    fn test_translate2 () {

        let t = Matrix::translate(5.0, -3.0, 2.0);
        let v = geo::Geo::vector(-3.0, 4.0, 5.0);

        assert_eq!(t*v, geo::Geo::vector(-3.0,4.0,5.0));

    }


    #[test]
    fn test_scale0 () {

        let t = Matrix::scale(2.0, 3.0, 4.0);
        let p = geo::Geo::point(-4.0, 6.0, 8.0);

        assert_eq!(t*p, geo::Geo::point(-8.0,18.0,32.0));

    }

    #[test]
    fn test_scale1 () {

        let t = Matrix::scale(2.0, 3.0, 4.0);
        let p = geo::Geo::vector(-4.0, 6.0, 8.0);

        assert_eq!(t*p, geo::Geo::vector(-8.0,18.0,32.0));

    }

    #[test]
    fn test_scale2 () {

        let t = Matrix::scale(2.0, 3.0, 4.0).inverse(); 
        let p  = geo::Geo::vector(-4.0, 6.0, 8.0);

        assert_eq!(t*p, geo::Geo::vector(-2.0,2.0,2.0));

    }

    #[test]
    fn test_reflect () {

        let t = Matrix::scale(-1.0, 1.0, 1.0); 
        let p  = geo::Geo::point(2.0, 3.0, 4.0);

        assert_eq!(t*p, geo::Geo::point(-2.0,3.0,4.0));

    }

 
    #[test]
    fn test_rotate_x () {

        let t1 = Matrix::rotate_x(PI/4.0);
        let t2 = Matrix::rotate_x(PI/2.0);

        let p  = geo::Geo::point(0.0, 1.0, 0.0);

        assert_eq!(t1*p, geo::Geo::point(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        assert_eq!(t2*p, geo::Geo::point(0.0, 0.0, 1.0));

    }   

    #[test]
    fn test_rotate_y () {

        let t1 = Matrix::rotate_y(PI/4.0);
        let t2 = Matrix::rotate_y(PI/2.0);

        let p  = geo::Geo::point(0.0, 0.0, 1.0);

        assert_eq!(t1*p, geo::Geo::point(2.0_f64.sqrt()/2.0, 0.0 ,2.0_f64.sqrt()/2.0));
        assert_eq!(t2*p, geo::Geo::point(1.0, 0.0, 0.0));

    }   


    #[test]
    fn test_rotate_z () {

        let t1 = Matrix::rotate_z(PI/4.0);
        let t2 = Matrix::rotate_z(PI/2.0);

        let p  = geo::Geo::point(0.0, 1.0, 0.0);

        assert_eq!(t1*p, geo::Geo::point(-2.0_f64.sqrt()/2.0,2.0_f64.sqrt()/2.0, 0.0));
        assert_eq!(t2*p, geo::Geo::point(-1.0, 0.0, 0.0));

    }   

    #[test]
    fn test_shear () {

        let p = geo::Geo::point(2.0,3.0,4.0);

        assert_eq!(Matrix::shear(1.0,0.0,0.0,0.0,0.0,0.0)*p, geo::Geo::point(5.0,3.0,4.0));
        assert_eq!(Matrix::shear(0.0,1.0,0.0,0.0,0.0,0.0)*p, geo::Geo::point(6.0,3.0,4.0));
        assert_eq!(Matrix::shear(0.0,0.0,1.0,0.0,0.0,0.0)*p, geo::Geo::point(2.0,5.0,4.0));
        assert_eq!(Matrix::shear(0.0,0.0,0.0,1.0,0.0,0.0)*p, geo::Geo::point(2.0,7.0,4.0));
        assert_eq!(Matrix::shear(0.0,0.0,0.0,0.0,1.0,0.0)*p, geo::Geo::point(2.0,3.0,6.0));
        assert_eq!(Matrix::shear(0.0,0.0,0.0,0.0,0.0,1.0)*p, geo::Geo::point(2.0,3.0,7.0));
    }

    #[test]
    fn test_chain1() {

        let p = geo::Geo::point(1.0,0.0,1.0);

        let a = Matrix::rotate_x(PI/2.0);
        let b = Matrix::scale(5.0,5.0,5.0);
        let c = Matrix::translate(10.0,5.0,7.0);

        let p2 = a*p;
        assert_eq!(p2,geo::Geo::point(1.0,-1.0,0.0));

        let p3 = b*p2;
        assert_eq!(p3,geo::Geo::point(5.0,-5.0,0.0));

        let p4 = c*p3;
        assert_eq!(p4,geo::Geo::point(15.0,0.0,7.0));
    }

    #[test]
    fn test_chain2() {

        let p = geo::Geo::point(1.0,0.0,1.0);

        let a = Matrix::rotate_x(PI/2.0);
        let b = Matrix::scale(5.0,5.0,5.0);
        let c = Matrix::translate(10.0,5.0,7.0);

        let p5 = c*b*a*p;
        assert_eq!(p5,geo::Geo::point(15.0,0.0,7.0));

    }


}

