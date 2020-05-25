use std::ops;

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
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs:Color) -> Color {


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
}
