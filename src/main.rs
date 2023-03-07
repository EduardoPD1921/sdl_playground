mod error;
use error::SDLError;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::process::exit;

fn main() {
    let sdl_context = get_sdl_context();
    let video_subsystem = get_video_subsystem(&sdl_context);
    let window = get_window(video_subsystem);
    let mut canvas = get_canvas(window);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'eventloop: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'eventloop;
                }
                _ => println!("{:?}", event)
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_sdl_context() -> sdl2::Sdl {
    if let Ok(sdl_context) = sdl2::init() {
        sdl_context
    } else {
        eprintln!("{}", SDLError::LoadSDLContext);
        exit(1);
    }
}

fn get_video_subsystem(sdl_context: &sdl2::Sdl) -> sdl2::VideoSubsystem {
    if let Ok(video_subsystem) = sdl_context.video() {
        video_subsystem
    } else {
        eprintln!("{}", SDLError::LoadVideoSubsystem);
        exit(1);
    }
}

fn get_window(video_subsystem: sdl2::VideoSubsystem) -> sdl2::video::Window {
    if let Ok(window) = video_subsystem.window("Playground", 800, 800).position_centered().build() {
        window
    } else {
        eprintln!("{}", SDLError::BuildWindow);
        exit(1);
    }
}

fn get_canvas(window: sdl2::video::Window) -> sdl2::render::Canvas<sdl2::video::Window> {
    if let Ok(canvas) = window.into_canvas().present_vsync().build() {
        canvas
    } else {
        eprintln!("{}", SDLError::BuildCanvas);
        exit(1);
    }
}
