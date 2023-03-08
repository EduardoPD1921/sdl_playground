mod error;
use error::SDLError;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::process::exit;
use std::time::{Duration, Instant};
use std::path::Path;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn main() {
    let sdl_context = get_sdl_context();
    let video_subsystem = get_video_subsystem(&sdl_context);
    let window = get_window(video_subsystem);
    let ttf_context = get_ttf_context();

    let mut canvas = get_canvas(window);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = Path::new("./assets/Minecraft.ttf");

    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut circle_radius = 200;

    let mut fps_timer = Instant::now();
    let mut frame_counter = 0;
    let mut fps_text = "FPS: ".to_owned();

    'eventloop: loop {
        frame_counter += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'eventloop;
                },
                Event::MouseWheel { y, .. } => {
                    circle_radius += y * 5;
                },
                _ => {}
            }
        }

        let mouse_state = event_pump.mouse_state();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps_text = "FPS: ".to_owned();
            fps_text.push_str(&frame_counter.to_string());

            frame_counter = 0;
            fps_timer = Instant::now();
        }

        let surface = font
            .render(fps_text.as_str())
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let target = sdl2::rect::Rect::new(0, 0, 100, 20);

        canvas.copy(&texture, None, Some(target)).unwrap();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_circle(mouse_state.x(), mouse_state.y(), circle_radius);

        canvas.present();
    }
}

trait Circle {
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32);
}

impl Circle for sdl2::render::Canvas<sdl2::video::Window> {
    // this function implements the basic equation of a circle -> (x - cx)2 + (y - cy)2 = r2
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32) {
        for x in (cx - r)..(cx + r) {
            let y = ((r as f64).powf(2.0) - ((x - cx) as f64).powf(2.0)).sqrt() as i32 + cy;
            self.draw_point(Point::new(x, y)).unwrap();
            self.draw_point(Point::new(x, 2 * cy - y)).unwrap();
        }
        for y in (cy - r)..(cy + r) {
            let x = ((r as f64).powf(2.0) - ((y - cy) as f64).powf(2.0)).sqrt() as i32 + cx;
            self.draw_point(Point::new(x, y)).unwrap();
            self.draw_point(Point::new(2 * cx - x, y)).unwrap();
        }
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
    if let Ok(window) = video_subsystem.window("Playground", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().build() {
        window
    } else {
        eprintln!("{}", SDLError::BuildWindow);
        exit(1);
    }
}

fn get_canvas(window: sdl2::video::Window) -> sdl2::render::Canvas<sdl2::video::Window> {
    if let Ok(canvas) = window.into_canvas().build() {
        canvas
    } else {
        eprintln!("{}", SDLError::BuildCanvas);
        exit(1);
    }
}

fn get_ttf_context() -> sdl2::ttf::Sdl2TtfContext {
    if let Ok(ttf_context) = sdl2::ttf::init() {
        ttf_context
    } else {
        eprintln!("{}", SDLError::LoadTtfContext);
        exit(1);
    }
}
