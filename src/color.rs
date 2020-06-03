use std::ops;

const EPSILON:f64 = 0.0000001;

#[derive(Debug,Copy,Clone)]
pub struct Color {
    pub red   : f64,
    pub green : f64,
    pub blue  : f64 
}

impl Color {

    pub fn new(red:f64, green:f64, blue:f64) -> Color {
        Color { red:red, green:green, blue:blue }
    }

    fn scale_color(color_value:f64, scale_factor:i32) -> i32 {
        
        if color_value >= 1.0 {
            
            return scale_factor

        } else if color_value <= 0.0 {

            return 0;

        } else {

            let sf = scale_factor as f64;
            return (color_value * sf) as i32;
        }
    }

    pub fn to_ppm(&self, sf:i32) -> String {

        format!("{} {} {} " ,Color::scale_color(self.red,   sf),
                             Color::scale_color(self.green, sf),
                             Color::scale_color(self.blue,  sf))
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs:Color) -> Color {
 
        Color::new(self.red   + rhs.red,
                   self.green + rhs.green,
                   self.blue  + rhs.blue)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs:Color) -> Color {
 
        Color::new(self.red   - rhs.red,
                   self.green - rhs.green,
                   self.blue  - rhs.blue)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs:Color) -> Color {
 
        Color::new(self.red   * rhs.red,
                   self.green * rhs.green,
                   self.blue  * rhs.blue)
    }
}



impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self,rhs:Color) -> Color {
       Color::new(self * rhs.red,
                self * rhs.green,
                self * rhs.blue)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self,rhs:f64) -> Color {
        Color::new(self.red * rhs,
                 self.green * rhs,
                 self.blue * rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.red   - other.red  ).abs() < EPSILON &&
        (self.green - other.green).abs() < EPSILON &&
        (self.blue  - other.blue ).abs() < EPSILON
    }
}

impl Eq for Color {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_values () {

        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red,  -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue,  1.7);
    }

    #[test]
    fn test_color_adding () {
        let c1 = Color::new(0.9,0.6,0.75);
        let c2 = Color::new(0.7,0.1,0.25);
        let result = c1 + c2;

        assert_eq!(result,Color::new(1.6,0.7,1.0));
    }

 
    #[test]
    fn test_color_subtracting () {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c1 - c2;

        assert_eq!(result,Color::new(0.2,0.5,0.5));
    }

    #[test]
    fn test_cmul () {

        let result = 2.0*Color::new(0.2,0.3,0.4);
        assert_eq!(result,Color::new(0.4,0.6,0.8));
    }

    #[test]
    fn test_hadamard () {
        let c1 = Color::new(1.0,0.2,0.4);
        let c2 = Color::new(0.9,1.0,0.1);

        let result = c1 * c2;

        assert_eq!(result,Color::new(0.9,0.2,0.04));
    }

    #[test]
    fn test_ppm () {

        let c1 = Color::new(1.0,1.0,1.0);
        let r1 = c1.to_ppm(255);

        assert_eq!(r1,"255 255 255 ");

        let c3 = Color::new(1.0,0.8,0.6);
        let r3 = c3.to_ppm(255);

        assert_eq!(r3,"255 204 153 ");
    }
}
