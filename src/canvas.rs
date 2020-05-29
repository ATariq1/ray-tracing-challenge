
use crate::color;

struct Canvas {
    width : usize,
    height: usize,
    grid: Vec<color::Color>
}

impl Canvas {

    pub fn new(width:usize, height:usize) -> Canvas {
        let grid = vec!( color::Color::new(0.0,0.0,0.0) ; (width*height) as usize );

        Canvas {width:width, height:height, grid:grid}
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self,x:usize,y:usize) -> color::Color {
        
        self.grid[self.width * y + x]
    }

    pub fn write_pixel(&mut self,x:usize,y:usize,c:color::Color) {

        self.grid[self.width*y + x] = c;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_canvas () {

        let c = Canvas::new(10,20);
        let black = color::Color::new(0.0,0.0,0.0); 

        assert_eq!(10,c.get_width());
        assert_eq!(20,c.get_height());
        
        for x in 0..9 {
            for y in 0..19 {
                assert_eq!(c.pixel_at(x,y),black);
            }
        }
    }

    #[test]
    fn test_pixel_write () {
    
        let mut c = Canvas::new(10,20);
        let red = color::Color::new(1.0,0.0,0.0);
        
        c.write_pixel(2,3,red);
    
        assert_eq!(c.pixel_at(2,3),red);

    }


} 
