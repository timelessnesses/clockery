// #![windows_subsystem = "windows"]
use chrono_tz;
use clap::{self, Parser};
use clock::Clock;
use ctrlc;
use sdl2;
use serde_json;
use std::{self, iter::zip, str::FromStr};
mod clock;
mod snow;

#[cfg(target_os = "windows")]
use winapi;

const NOTOSANS: &[u8; 556216] = include_bytes!("assets/NotoSans-Regular.ttf");
const DIGITAL: &[u8; 20984] = include_bytes!("assets/digital.ttf");

#[derive(clap::Parser)]
#[command(author = "timelessnesses", about = "Clockery, a timezone shower.")]
struct Cli {
    /// Frame limiting
    #[arg(short, long)]
    fps: Option<i64>,
    /// List GPU renderers (for the SELECTED_GPU_RENDERER arg)
    #[arg(short, long)]
    list_gpu_renderers: bool,
    /// Select your own renderer if you want to
    #[arg(short, long)]
    selected_gpu_renderer: Option<usize>,
}

#[cfg(target_os = "windows")]
fn hide_console() {
    // use winapi;
    unsafe {
        let cw = winapi::um::wincon::GetConsoleWindow();
        if cw != std::ptr::null_mut() {
            winapi::um::winuser::ShowWindow(cw, winapi::um::winuser::SW_HIDE);
        }
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    hide_console();

    let parsed = Cli::parse();
    if parsed.list_gpu_renderers {
        for (i, item) in sdl2::render::drivers().enumerate() {
            println!(
                "Renderer #{}:\n   Name: {}\n  Flags: {}",
                i + 1,
                item.name,
                item.flags
            )
        }
        return;
    }

    let fl = match parsed.fps {
        Some(f) => f,
        None => -1,
    };

    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let mut window = video
        .window("Clockery", 800, 600)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .metal_view()
        // .opengl()
        .build()
        .unwrap();
    let config = std::fs::read_to_string("./config.json").or({
        match std::fs::File::create("./config.json") {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e),
        }
    });
    let mut config = match config {
        Ok(c) => {
            load_config(c) // being dropped lmao
        }
        Err(_) => {
            panic!("Failed To Open File.")
        }
    };

    let mut num_surfaces = config.clocks.len() as i32;
    let size = window.size();
    let mut snow_enable = config.snow_enable;
    let snow_limit = {
        if snow_enable {
            config.snow_limit
        } else {
            0
        }
    };
    let mut surfaces = create_surfaces(num_surfaces, size, config.revert, snow_limit);

    let mut am_pm = config.am_pm;
    let mut revert = config.revert;

    window.set_minimum_size(800, 600).unwrap();
    let mut canvas = match parsed.selected_gpu_renderer {
        Some(i) => match window.into_canvas().index((i - 1) as u32).build() {
            Ok(c) => c,
            Err(_) => {
                panic!("Failed to initialize with your index driver provided in the argument!")
            }
        },
        None => window.into_canvas().build().unwrap(),
    };
    let mut event_pump = ctx.event_pump().unwrap();

    let mut running = true;
    let clock = ctx.timer().unwrap();

    let font_loader = sdl2::ttf::init().unwrap();

    let date_font = font_loader
        .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(DIGITAL).unwrap(), 40)
        .unwrap();
    let normal_font = font_loader
        .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(NOTOSANS).unwrap(), 20)
        .unwrap();

    let fps_font = font_loader
        .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(NOTOSANS).unwrap(), 15)
        .unwrap();

    let head_font = font_loader
        .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(NOTOSANS).unwrap(), 50)
        .unwrap();

    // let copied = &config;
    let mut ft = std::time::Instant::now();
    let mut fc = 0;
    let mut fps = 0.0;

    let mut mf = 0.0;

    let mut lf = 0.0;
    let mut lpf = 0.0;
    let mut lft = std::time::Instant::now();

    // let _cloned_config = config.clone();
    ctrlc::set_handler(move || {
        running = !running;
    })
    .expect("Failed to set CTRL+C handler for whatever reason");
    'running: while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::R),
                    ..
                } => revert = !revert,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::A),
                    ..
                } => am_pm = !am_pm,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Q),
                    ..
                } => {
                    let content =
                        std::fs::read_to_string("./config.json").expect("Cannot read the file");
                    config = load_config(content);
                    am_pm = config.am_pm;
                    revert = config.revert;
                    num_surfaces = config.clocks.len() as i32;
                    surfaces =
                        create_surfaces(num_surfaces, canvas.output_size().unwrap(), revert, {
                            // snow_enable = !snow_enable;
                            if snow_enable {
                                config.snow_limit
                            } else {
                                0
                            }
                        });
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::S),
                    ..
                } => {
                    surfaces =
                        create_surfaces(num_surfaces, canvas.output_size().unwrap(), revert, {
                            snow_enable = !snow_enable;
                            if snow_enable {
                                config.snow_limit
                            } else {
                                0
                            }
                        });
                }
                sdl2::event::Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(_, _) => {
                        // println!("{:#?}", canvas.output_size().unwrap());
                        surfaces =
                            create_surfaces(num_surfaces, canvas.output_size().unwrap(), revert, {
                                // snow_enable = !snow_enable;
                                if snow_enable {
                                    config.snow_limit
                                } else {
                                    0
                                }
                            });
                        // println!("Created surface!")
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        clear(&mut canvas, revert);
        apply(
            &config.clocks,
            am_pm,
            revert,
            &mut surfaces,
            &mut canvas,
            &date_font,
            &normal_font,
        );
        center("Clockery", &mut canvas, &head_font, Some(100), revert);
        to_screen(
            &render_font(
                &fps_font,
                format_args!("FPS: {}", truncate(fps, 2))
                    .to_string()
                    .as_str(),
                revert,
            ),
            &mut canvas,
            Some((0, 0)),
            None,
        );
        to_screen(
            &render_font(
                &fps_font,
                format_args!("Max FPS: {}", truncate(mf, 2))
                    .to_string()
                    .as_str(),
                revert,
            ),
            &mut canvas,
            Some((0, 15)),
            None,
        );
        to_screen(
            &render_font(
                &fps_font,
                format_args!("Min FPS: {}", truncate(lf, 2))
                    .to_string()
                    .as_str(),
                revert,
            ),
            &mut canvas,
            Some((0, 30)),
            None,
        );
        // apply(config.clone(), &mut surfaces, &mut canvas, &date_font, &normal_font);
        canvas.present();
        // unsafe {canvas.render_flush();}
        fc += 1;
        let elapsed_time = ft.elapsed();
        if elapsed_time.as_secs() >= 1 {
            fps = fc as f64 / elapsed_time.as_secs_f64();
            fc = 0;
            ft = std::time::Instant::now();
            if fps > mf {
                mf = fps
            } else if fps < lpf {
                lpf = fps
            }
        }
        let elapsed_time = lft.elapsed();
        if elapsed_time.as_secs() >= 3 {
            lf = lpf;
            lpf = fps;
            lft = std::time::Instant::now();
        }
        clock.delay(delay_fps(fl as i32))
    }
    let mut converted: Vec<String> = Vec::new();
    for e in config.clone().clocks {
        if e.is_none() {
            converted.push("local".to_owned());
            continue;
        }
        converted.push(e.unwrap().name().to_owned())
    }

    match std::fs::File::options()
        .read(true)
        .write(true)
        .open("./config.json")
    {
        Ok(f) => {
            let v = serde_json::json!(
                {
                    "clocks": converted,
                    "am_pm": am_pm,
                    "revert": revert,
                    "snow_enable": snow_enable,
                    "snow_limit": config.snow_limit
                }
            );
            serde_json::to_writer_pretty(f, &v).expect("Failed to save configuration");
        }
        Err(_) => {
            panic!("Failed to save configuration");
        }
    }
}

fn _create_instances<'a, 'b, 'c, 'd>(
    config: Configuration,
    date_font: &'c sdl2::ttf::Font<'a, 'b>,
    normal_font: &'c sdl2::ttf::Font<'a, 'b>,
) -> Vec<clock::Clock<'a, 'b, 'c, 'd>> {
    let mut v: Vec<clock::Clock<'a, 'b, 'c, 'd>> = Vec::new();
    for tz in config.clocks {
        v.push(Clock::new(
            tz,
            config.am_pm,
            config.revert,
            date_font,
            normal_font,
        ))
    }
    return v;
}

fn clear(window: &mut sdl2::render::Canvas<sdl2::video::Window>, revert: bool) {
    window.set_draw_color({
        if revert {
            sdl2::pixels::Color::WHITE
        } else {
            sdl2::pixels::Color::BLACK
        }
    });
    window.clear();
}
fn to_screen(
    text: &sdl2::surface::Surface,
    window: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dest: Option<(i32, i32)>,
    rect: Option<&sdl2::rect::Rect>,
) {
    let texture_creator = window.texture_creator();
    let text = match texture_creator.create_texture_from_surface(text) {
        Ok(t) => t,
        Err(_) => return,
    };
    window
        .copy(&text, None, {
            match (dest, rect) {
                (Some(d), None) => {
                    let size = text.query();
                    sdl2::rect::Rect::new(d.0, d.1, size.width, size.height)
                }
                (None, Some(r)) => *r,
                _ => {
                    panic!("Unexpected");
                }
            }
        })
        .unwrap();
}

fn render_font<'a>(font: &sdl2::ttf::Font, text: &str, revert: bool) -> sdl2::surface::Surface<'a> {
    let a = font
        .render(text)
        .blended({
            if revert {
                sdl2::pixels::Color::BLACK
            } else {
                sdl2::pixels::Color::WHITE
            }
        })
        .unwrap();

    return a;
}

#[derive(Clone)]
struct Configuration {
    pub clocks: Vec<Option<chrono_tz::Tz>>,
    pub am_pm: bool,
    pub revert: bool,
    pub snow_limit: i64,
    pub snow_enable: bool,
}

fn load_config(con: String) -> Configuration {
    let config: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&con);

    let config = match config {
        Ok(values) => values,
        Err(_) => {
            // panic!("{:?}", p);
            let write = serde_json::json!({
                "clocks": ["local"],
                "revert": false,
                "am_pm": false,
                "snow_enable": false,
                "snow_limit": 200
            });
            let file = std::fs::File::options()
                .write(true)
                .open("./config.json")
                .unwrap();
            serde_json::to_writer_pretty(&file, &write)
                .expect("Failed to write default configuration to a file.");
            write
        }
    };

    let clocks = parse_timezones(config["clocks"].as_array().unwrap().to_owned());
    let am_pm: bool = config["am_pm"].as_bool().unwrap();
    let revert: bool = config["revert"].as_bool().unwrap();
    let snow_limit = config["snow_limit"].as_i64().unwrap();
    let snow_enable = config["snow_enable"].as_bool().unwrap();

    // drop(file);

    return Configuration {
        clocks,
        am_pm,
        revert,
        snow_enable,
        snow_limit,
    };
}

fn parse_timezones(timezones: Vec<serde_json::Value>) -> Vec<Option<chrono_tz::Tz>> {
    let mut c: Vec<String> = Vec::new();
    for e in timezones {
        c.push(e.as_str().unwrap().to_owned());
    }

    let mut a: Vec<Option<chrono_tz::Tz>> = Vec::new();
    for x in c {
        // println!("{}", x);
        let tz = if x.to_lowercase() == "local" {
            None
        } else {
            match chrono_tz::Tz::from_str(&x) {
                Ok(t) => Some(t),
                Err(_) => {
                    println!("Failed to parse timezone: {}", x);
                    continue;
                }
            }
        };
        a.push(tz);
    }
    return a;
}

fn create_surfaces<'a>(
    corners: i32,
    size: (u32, u32),
    revert: bool,
    snow_limit: i64,
) -> Vec<(
    sdl2::rect::Rect,
    sdl2::surface::Surface<'a>,
    snow::SnowParticles,
)> {
    let (w, h) = size;
    let bg_color = if revert { (0, 0, 0) } else { (255, 255, 255) };

    let mut x: Vec<(
        sdl2::rect::Rect,
        sdl2::surface::Surface,
        snow::SnowParticles,
    )> = Vec::new();

    let num_rows = (corners as f64).sqrt() as i32;
    let num_cols = (corners + num_rows - 1) / num_rows;
    // println!("{} {} {}", num_cols, corners, num_rows);
    let surface_width = w / (num_cols as u32);
    let surface_height = h / (num_rows as u32);

    for i in 0..num_rows {
        for j in 0..num_cols {
            let index = i * num_cols + j;
            if index < corners {
                let mut surface = sdl2::surface::Surface::new(
                    surface_width,
                    surface_height,
                    sdl2::pixels::PixelFormatEnum::RGB24,
                )
                .unwrap();
                surface
                    .fill_rect(
                        None,
                        sdl2::pixels::Color::RGB(bg_color.0, bg_color.1, bg_color.2),
                    )
                    .unwrap();
                let surface_rect = sdl2::rect::Rect::new(
                    surface_width as i32 * j,
                    surface_height as i32 * i,
                    surface_width,
                    surface_height,
                );
                let s =
                    snow::SnowParticles::new((snow_limit / corners as i64) as i32, &mut surface);
                x.push((surface_rect, surface, s));
                // println!("{:?}", surface_rect)
            }
        }
    }

    return x;
}

fn delay_fps(fps: i32) -> u32 {
    if fps <= 0 {
        0
    } else {
        1000 / fps as u32
    }
}

fn center(
    text: &str,
    renderer: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
    y: Option<u32>,
    revert: bool,
) {
    let rendered = render_font(font, text, revert);
    let middle = get_middle_surface(&rendered, renderer, y);
    to_screen(&rendered, renderer, None, Some(&middle));
}

fn get_middle_surface(
    surface: &sdl2::surface::Surface,
    window: &sdl2::render::Canvas<sdl2::video::Window>,
    y: Option<u32>,
) -> sdl2::rect::Rect {
    let (w, h) = window.output_size().unwrap();
    let r: sdl2::rect::Rect;

    match y {
        Some(pos) => {
            r = sdl2::rect::Rect::new(
                ((w - surface.width()) / 2) as i32,
                ((pos as u32 - surface.height()) / 2) as i32,
                surface.width(),
                surface.height(),
            );
        }
        None => {
            r = sdl2::rect::Rect::new(
                ((w - surface.width()) / 2) as i32,
                ((h - surface.height()) / 2) as i32,
                surface.width(),
                surface.height(),
            );
        }
    }

    return r;
}

fn truncate(b: f64, precision: usize) -> f64 {
    f64::trunc(b * ((10 * precision) as f64)) / ((10 * precision) as f64)
}

fn apply<'a, 'b, 'c>(
    clock: &Vec<Option<chrono_tz::Tz>>,
    am_pm: bool,
    revert: bool,
    surfaces: &mut Vec<(
        sdl2::rect::Rect,
        sdl2::surface::Surface,
        snow::SnowParticles,
    )>,
    renderer: &mut sdl2::render::Canvas<sdl2::video::Window>,
    date_font: &'c sdl2::ttf::Font<'a, 'b>,
    normal_font: &'c sdl2::ttf::Font<'a, 'b>,
) {
    let mut j: Vec<(sdl2::rect::Rect, &sdl2::surface::Surface)> = Vec::new();
    for (c, i) in zip(clock, surfaces) {
        let clocker = clock::Clock::new(*c, am_pm, revert, date_font, normal_font);
        clear_surface(&mut i.1, revert);
        clocker.render(clocker.current_datetime_in_timezone(), &mut i.1, &mut i.2);
        j.push((i.0, &i.1))
    }
    for (rect, surface) in j {
        to_screen(&surface, renderer, None, Some(&rect));
        // println!("{:?}", rect);
    }
}

fn clear_surface(surface: &mut sdl2::surface::Surface, revert: bool) {
    surface
        .fill_rect(None, {
            if revert {
                sdl2::pixels::Color::WHITE
            } else {
                sdl2::pixels::Color::BLACK
            }
        })
        .unwrap()
}
