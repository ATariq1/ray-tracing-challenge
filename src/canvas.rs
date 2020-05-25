extern crate sdl2;

use sdl2::pixels::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_pixels () {


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

        assert_eq!(1,1-1+1);

    }
}


