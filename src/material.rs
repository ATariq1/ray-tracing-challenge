use crate::color;

#[derive(Debug,Copy,Clone)]
pub struct Material {
    pub color:color::Color,
    pub ambient:f64,
    pub diffuse:f64,
    pub specular:f64,
    pub shininess:f64,
}

impl Material {

    pub fn default() -> Material {
        Material {
                color:color::Color::new(1.0, 1.0, 1.0),
                ambient:0.1f64,
                diffuse: 0.9f64,
                specular: 0.9f64,
                shininess: 200.0f64,    
            }
    }
}

#[test]
fn material_default() {

    let m = Material::default();

    assert_eq!(m.color,color::Color::new(1.0, 1.0, 1.0));
    assert_eq!(m.ambient, 0.1f64 );
    assert_eq!(m.diffuse, 0.9f64);
    assert_eq!(m.specular, 0.9f64);
    assert_eq!(m.shininess, 200.0f64);

}

