use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::color;
use std::io::LineWriter;

const COLOR_MAX:i32 = 255;

pub struct Canvas {
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

        if x < self.width &&
           y < self.height {
               
            self.grid[self.width*y + x] = c;
        }
    }

    pub fn to_ppm(&self, path:&str) {

	let path = Path::new(path);

	let file = File::create(&path).expect("failed to create");

        let mut file = LineWriter::new(file);

        file.write_all(b"P3\n")
            .expect("failed to write");
        
        file.write_all(format!("{} {}\n",self.width,self.height).as_bytes())
            .expect("failed to write");
        
        file.write_all(format!("{}\n",COLOR_MAX).as_bytes())
            .expect("failed to write");

        let mut count = 0;
        
        for color in self.grid.iter() {

            file.write_all(color.to_ppm(COLOR_MAX).as_bytes())
                .expect("failed to write");

            count += 1;

            if count%5 == 0 {

                file.write_all("\n".as_bytes())
                    .expect("failed to write");
            }
        }

        file.write_all("\n".as_bytes())
            .expect("failed to write");

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_canvas () {

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
    fn pixel_write () {
    
        let mut c = Canvas::new(10,20);
        let red = color::Color::new(1.0,0.0,0.0);
        
        c.write_pixel(2,3,red);
    
        assert_eq!(c.pixel_at(2,3),red);

    }

    #[test]
    fn ppm_conversion() {

        let c = Canvas::new(3,3);

        c.to_ppm("ppm/test1.ppm");
        
        let contents = fs::read_to_string("ppm/test1.ppm").expect("could not read");

        assert_eq!(contents.trim(),
        "P3\n\
        3 3\n\
        255\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 0 0 0 0 0");

    }

    #[test]
    fn color_bounds() {
    
        let mut c = Canvas::new(5,3);
        
        let c1 = color::Color::new( 1.5, 0.0, 0.0);
        let c2 = color::Color::new( 0.0, 0.5, 0.0);
        let c3 = color::Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0,0,c1);
        c.write_pixel(2,1,c2);
        c.write_pixel(4,2,c3);

        c.to_ppm("ppm/bounds.ppm");

        let contents = fs::read_to_string("ppm/bounds.ppm").expect("could not read");
        
        assert_eq!(contents.trim(), 
        "P3\n\
         5 3\n\
         255\n\
         255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
         0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 \n\
         0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn fill_ppm() {

        let mut c = Canvas::new(10,2);
        
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x,y,color::Color::new(1.0,0.8,0.6));
            }
        }

        c.to_ppm("ppm/fill.ppm");

        let contents = fs::read_to_string("ppm/fill.ppm").expect("could not read");

        assert_eq!(contents.trim(), 
        "P3\n\
        10 2\n\
        255\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
    
    }
} 
