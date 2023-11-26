use sdl2;
use serde_json;
use std::{self, str::FromStr};
use chrono_tz;
mod clock;
const FPS: i32 = 60;
const NOTOSANS: &[u8; 556216] = include_bytes!("assets/NotoSans-Regular.ttf");
const DIGITAL: &[u8; 20984] = include_bytes!("assets/digital.ttf");

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let mut window = video
        .window("Clockery", 800, 600)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .metal_view()
        .build()
        .unwrap();
    let config = std::fs::File::options().read(true).write(true).open("./config.json").or(std::fs::File::create("./config.json"));
    let config = match config {
        Ok(c) => {
            load_config(c) // being dropped lmao
            
        },
        Err(_) => {
            panic!("Failed To Open File.")
        },
    };

    let max_fps = 0;
    let min_fps = 0;

    let num_surfaces = config.clocks.len() as i32;
    let surfaces = create_surfaces(num_surfaces, &window.size(), config.revert);

    let am_pm = config.am_pm;
    let revert = config.revert;

    window.set_minimum_size(800, 600).unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.present();
    let mut event_pump = ctx.event_pump().unwrap();    

    let running = true;
    let clock = ctx.timer().unwrap();

    let font_loader = sdl2::ttf::init().unwrap();

    let font_file = sdl2::rwops::RWops::from_bytes(NOTOSANS).unwrap();
    let font_file2 = sdl2::rwops::RWops::from_bytes(NOTOSANS).unwrap();

    let date_font_file = sdl2::rwops::RWops::from_bytes(DIGITAL).unwrap();
    let normal_font_file = sdl2::rwops::RWops::from_bytes(DIGITAL).unwrap();

    let font = font_loader.load_font_from_rwops(font_file, 50).unwrap();
    let font2 = font_loader.load_font_from_rwops(font_file2, 20).unwrap();

    let date_font = font_loader.load_font_from_rwops(date_font_file, 40).unwrap();
    let normal_font_file = font_loader.load_font_from_rwops(normal_font_file, 20).unwrap();

    'running: while running {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::Window {win_event, .. } => {
                    if let sdl2::event::WindowEvent::Resized(w, h) = win_event {
                        println!("w: {}, h:{}", w,h)
                    }                    
                }
                _ => {}
            }
        }
        canvas.present();
        clock.delay((1000 / FPS) as u32)
    }
    let mut converted: Vec<String> = Vec::new();
    for e in config.clocks {
        if e.is_none() {
            converted.push("local".to_owned());
            continue;
        }
        converted.push(e.unwrap().name().to_owned())
    }

    match std::fs::File::options().read(true).write(true).open("./config.json").or(std::fs::File::create("./config.json")) {
        Ok(f) => {
            let v = serde_json::json!(
                {
                    "clocks": converted,
                    "am_pm": am_pm,
                    "revert": revert
                }
            );
            serde_json::to_writer_pretty(f, &v).expect("Failed to save configuration");
        },
        Err(_) => {
            panic!("Failed to save configuration");
        }
    }


}

#[derive(Clone)]
struct Configuration {
    pub clocks: Vec<Option<chrono_tz::Tz>>,
    pub am_pm: bool,
    pub revert: bool
}

fn load_config(file: std::fs::File) -> Configuration {
    let config: Result<serde_json::Value, serde_json::Error> = serde_json::from_reader(&file);

    let config = match config {
        Ok(values) => {
            values
        },
        Err(_) => {
            let write = serde_json::json!({
                "clocks": ["local"],
                "revert": false,
                "am_pm": false
            });
            serde_json::to_writer_pretty(&file, &write).expect("Failed to write default configuration to a file.");
            write
        }
    };

    let clocks = parse_timezones(config["clocks"].as_array().unwrap().to_owned());
    let am_pm: bool = config["am_pm"].as_bool().unwrap();
    let revert: bool = config["revert"].as_bool().unwrap();

    drop(file);

    return Configuration { clocks: clocks, am_pm: am_pm, revert: revert }
}

fn parse_timezones(timezones: Vec<serde_json::Value>) -> Vec<Option<chrono_tz::Tz>>{
    let mut c: Vec<String> = Vec::new();
    for e in timezones {
        c.push(e.as_str().unwrap().to_owned());
    }

    let mut a: Vec<Option<chrono_tz::Tz>> = Vec::new();
    for x in c {
        let tz = if x.to_lowercase() == "local" {
            None
        } else {
            match chrono_tz::Tz::from_str(&x) {
                Ok(t) => Some(t),
                Err(_) => {
                    println!("Failed to parse timezone: {}", x);
                    continue
                },
            }
        };
        a.push(tz);
    }
    return a
    
}

fn create_surfaces(corners: i32, size: &(u32, u32), revert: bool) -> Vec<(sdl2::rect::Rect, sdl2::surface::Surface<'static>)> {
    let (w,h) = size;
    let bg_color = if revert {
        (0,0,0)
    } else {
        (255,255,255)
    };

    let mut x: Vec<(sdl2::rect::Rect, sdl2::surface::Surface)> = Vec::new();

    let num_rows = (corners as f64).sqrt() as i32;
    let num_cols = corners + num_rows - 1;

    let surface_width = w / (num_cols as u32);
    let surface_height = h / (num_rows as u32);

    for i in 0..num_rows {
        for j in 0..num_cols {
            let index = i * num_cols + j;
            if index < corners {
                let mut surface = sdl2::surface::Surface::new(surface_width, surface_height, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
                surface.fill_rect(None, sdl2::pixels::Color::RGB(bg_color.0, bg_color.1, bg_color.2)).unwrap();
                let surface_rect = sdl2::rect::Rect::new(surface_width as i32 * j, surface_width as i32 * i, surface_width, surface_height);
                x.push((surface_rect, surface))
            }
        }
    }



    return x
}