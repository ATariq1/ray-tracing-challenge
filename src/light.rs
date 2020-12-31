use crate::material;
use crate::color;
use crate::geo;

#[derive(Debug,Copy,Clone)]
pub struct Light {
    pub intensity:color::Color,
    pub position:geo::Geo,
}

impl Light {
   pub fn point(intensity:color::Color, position:geo::Geo) -> Light {
       Light {intensity:intensity, position:position}
   }
}

pub fn lighting(material:material::Material, light:Light, point:geo::Geo, eyev:geo::Geo, norm:geo::Geo) -> color::Color {

    let black = color::Color::new(0.0, 0.0, 0.0);
    // surface color combined with light
    let effective_color = material.color * light.intensity;
    // direction of light source
    let lightv = (light.position - point).norm();
    // ambient light contribution to render
    let ambient = effective_color*material.ambient;
    let mut diffuse  = black;
    let mut specular = black;


    let light_dot_normal = lightv.dot(norm);

    if light_dot_normal < 0.0 { 
        return ambient + diffuse + specular;
    } else {
        diffuse             = effective_color*material.diffuse*light_dot_normal;
        let reflectv        = (-lightv).reflect(norm);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0.0 {
            specular = black;
        } else {

            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity*material.specular*factor;
        }
    }

    return ambient + diffuse + specular;
}

#[test]
fn light_struct() {

    let i = color::Color::new(1.0, 1.0, 1.0);
    let p = geo::Geo::point(0.0, 0.0, 0.0);
    let l = Light::point(i, p);

    assert_eq!(l.position,geo::Geo::point(0.0, 0.0, 0.0));
    assert_eq!(l.intensity, color::Color::new(1.0, 1.0, 1.0));

}

#[test]
fn light_eye_surface() {

    let m    = material::Material::default();
    let p    = geo::Geo::point(0.0, 0.0, 0.0);

    let eyev = geo::Geo::vector(0.0, 0.0, -1.0);
    let norm = geo::Geo::vector(0.0, 0.0, -1.0);

    let light  = Light::point(color::Color::new(1.0, 1.0, 1.0), geo::Geo::point(0.0, 0.0, -10.0));
    let result = lighting(m, light, p, eyev, norm);
    assert_eq!(result,color::Color::new(1.9, 1.9, 1.9));
}

#[test]
fn light_eye_45_surface() {

    let m    = material::Material::default();
    let p    = geo::Geo::point(0.0, 0.0, 0.0);

    let eyev = geo::Geo::vector(0.0, 1.0/(2.0f64).sqrt(), 1.0/(2.0f64).sqrt());
    let norm = geo::Geo::vector(0.0, 0.0, -1.0);

    let light  = Light::point(color::Color::new(1.0, 1.0, 1.0), geo::Geo::point(0.0, 0.0, -10.0));
    let result = lighting(m, light, p, eyev, norm);
    assert_eq!(result,color::Color::new(1.0, 1.0, 1.0));
}

#[test]
fn light_45_eye_surface() {

    let m    = material::Material::default();
    let p    = geo::Geo::point(0.0, 0.0, 0.0);

    let eyev = geo::Geo::vector(0.0, 0.0, -1.0);
    let norm = geo::Geo::vector(0.0, 0.0, -1.0);

    let light  = Light::point(color::Color::new(1.0, 1.0, 1.0), geo::Geo::point(0.0, 10.0, -10.0));
    let result = lighting(m, light, p, eyev, norm);
    assert_eq!(result,color::Color::new(0.736396103, 0.736396103, 0.736396103));
}

#[test]
fn light_eye_reflect_off_surface() {

    let m    = material::Material::default();
    let p    = geo::Geo::point(0.0, 0.0, 0.0);

    let eyev = geo::Geo::vector(0.0, -1.0/(2.0f64).sqrt(), -1.0/(2.0f64).sqrt());
    let norm = geo::Geo::vector(0.0, 0.0, -1.0);

    let light  = Light::point(color::Color::new(1.0, 1.0, 1.0), geo::Geo::point(0.0, 10.0, -10.0));
    let result = lighting(m, light, p, eyev, norm);
    assert_eq!(result,color::Color::new(1.636396103, 1.636396103, 1.636396103));
}


#[test]
fn light_behind_surface() {

    let m    = material::Material::default();
    let p    = geo::Geo::point(0.0, 0.0, 0.0);

    let eyev = geo::Geo::vector(0.0, 0.0, -1.0);
    let norm = geo::Geo::vector(0.0, 0.0, -1.0);

    let light  = Light::point(color::Color::new(1.0, 1.0, 1.0), geo::Geo::point(0.0, 0.0, 10.0));
    let result = lighting(m, light, p, eyev, norm);
    assert_eq!(result,color::Color::new(0.1, 0.1, 0.1));
}
