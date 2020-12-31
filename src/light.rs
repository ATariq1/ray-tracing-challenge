use crate::color;
use crate::geo;

pub struct Light {
    pub intensity:color::Color,
    pub position:geo::Geo,
}

impl Light {
   pub fn point(intensity:color::Color, position:geo::Geo) -> Light {
       Light {intensity:intensity, position:position}
   }
}

#[test]
fn light_struct() {

    let i = color::Color::new(1.0, 1.0, 1.0);
    let p = geo::Geo::point(0.0, 0.0, 0.0);
    let l = Light::point(i, p);

    assert_eq!(l.position,geo::Geo::point(0.0, 0.0, 0.0));
    assert_eq!(l.intensity, color::Color::new(1.0, 1.0, 1.0));

}

