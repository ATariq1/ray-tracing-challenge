use std::ops;

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

    pub fn get( &self,row:usize, col:usize) -> f64 {
        
        self.matrix[row*self.dim + col]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
    
        return (self.dim == other.dim && self.matrix == other.matrix);
    
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

        let A = vec![1.0,2.0,3.0,4.0,
                     5.0,6.0,7.0,8.0,
                     9.0,8.0,7.0,6.0,
                     5.0,4.0,3.0,2.0];

        let B = vec![1.0,2.0,3.0,4.0,
                     5.0,6.0,7.0,8.0,
                     9.0,8.0,7.0,6.0,
                     5.0,4.0,3.0,2.0];

        assert_eq!(A,B);
        
    }
}