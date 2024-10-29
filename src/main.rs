
use std::time::Duration;

use mat::{rotate_obj, Ln3};
use sdl2::{event::Event, pixels::Color, rect::Point, render::Canvas, video::Window, EventPump, Sdl};

mod mat;

fn main() {
    let (context, mut canvas) = init_render();
    let mut event_pump = context.event_pump().unwrap();   

    let mut cube: Vec<mat::Ln3> = mat::makeCube();

    loop {
        cube  = rotate_obj(&cube);
        draw_things(&mut canvas, &cube);
        handle_events(&mut event_pump);
        std::thread::sleep(Duration::new(0, 100000000));
    }


}

fn init_render() -> (Sdl, Canvas<Window>) {
    let context: sdl2::Sdl = sdl2::init().unwrap();
    let vid: sdl2::VideoSubsystem = context.video().unwrap();
    let window = vid.window("render", 800, 800).build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    (context, canvas)
}

fn draw_things(canvas: &mut Canvas<Window>, object: &Vec<mat::Ln3>) {
    let scr_cube = mat::t_wts_object(object);
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::WHITE);

    for ln in scr_cube {
        let pt1 = Point::new(ln.start.x as i32, ln.start.y as i32);
        let pt2 = Point::new(ln.end.x as i32, ln.end.y as i32);
        canvas.draw_line(pt1, pt2).unwrap();
    }
    canvas.present();
}

fn handle_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
           Event::Quit{..} | Event::KeyDown {..} => {std::process::exit(0);},
           _ => {}
        }
    }
}