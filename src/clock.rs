use std::thread::current;

use chrono::{self, TimeZone};
use sdl2;
use slicestring::Slice;

pub struct Clock<'a, 'b, 'c, 'd> {
    timezone: Option<chrono_tz::Tz>,
    am_pm: bool,
    name: String,
    revert: bool,
    date_font: &'c sdl2::ttf::Font<'a, 'b>,
    normal_font: &'c sdl2::ttf::Font<'a, 'b>,
    markery: &'d str, // used for 'd for the surface
}

impl<'a, 'b, 'c, 'd> Clock<'a, 'b, 'c, 'd> {
    pub fn new(
        timezone: Option<chrono_tz::Tz>,
        am_pm: bool,
        revert: bool,
        date_font: &'c sdl2::ttf::Font<'a, 'b>,
        normal_font: &'c sdl2::ttf::Font<'a, 'b>,
    ) -> Self {
        let name: String;
        if timezone.is_none() {
            name = "Local".to_owned();
        } else {
            name = timezone.unwrap().name().to_owned();
        }

        return Self {
            timezone,
            am_pm,
            name,
            revert,
            date_font,
            normal_font,
            markery: "s",
        };
    }

    pub fn current_datetime_in_timezone(
        &self,
    ) -> chrono::DateTime<chrono::FixedOffset> {
        match self.timezone {
            Some(tz) => {
                tz
                    .from_local_datetime(&chrono::Local::now().naive_local())
                    .unwrap().fixed_offset()
            },
            None => {
                chrono::Local::now().fixed_offset()
            }
        }
    }

    pub fn render(&self, time: chrono::DateTime<chrono::FixedOffset>, surface: &mut sdl2::surface::Surface) {
        let timer = {
            if !self.am_pm {
                time.format("%H:%M:%S").to_string()
            } else {
                time.format("%I:%M:%S %p").to_string()
            }
        };
        let middle_y = {
            let (_, h) = surface.size();
            (h / 2) as i32
        };
        self.center(&timer, surface, self.date_font, Some(middle_y));
        let offset: &str;
        if self.timezone.is_some() {
            offset = &self.name
        } else {
            offset = "Local time"
        }
        let zt = &time.format("%z").to_string();
        let z_n_or_p = zt.slice(..1);
        let zh = zt.slice(1..3);
        let zm = zt.slice(3..5);
        let zone = {
            if z_n_or_p == "-" {
                format!("{}:{} hours behind", zh, zm)
            } else if z_n_or_p == "+" {
                format!("{}:{} hours forward", zh, zm)
            } else {
                "Local time".to_owned()
        }
        };
        let d_thing = match time.format("%Z").to_string().parse::<i32>() {
                Ok(_) => {
                    format!("{} ({})", time.format("%A %d/%B/%Y UTC%Z").to_string(), zone)
                },
                Err(_) => {
                    format!("{} {}", time.format("%A %d/%B/%Y %Z").to_string(), zone)
                }
            };
        self.center(format!("Currently {}", offset).as_str(), surface, self.normal_font, Some(middle_y + 90));
        self.center(format!("The date is {}", d_thing).as_str(), surface, self.normal_font, Some(middle_y + 130));
    }
    fn center(&self, text: &str, surface: &mut sdl2::surface::Surface, font: &'c sdl2::ttf::Font<'a, 'b>, y: Option<i32>) {
        let y = match y{
            Some(s) => s,
            None => 0,
        };
        let (mw, _) = surface.size();
        let wrapped = self.word_wrap(text, mw, font);

        let h = wrapped.len() as i32 * font.height();
        let mut y_centered = (y - h / 2) as i32;

        for line in wrapped {
            let rendered = match self.render_font(font, &line) {
                Ok(s) => s,
                Err(_) => continue
            };
            // let rect = sdl2::rect::Rect::new(rendered.width() as i32, rendered.height() as i32, (surface.width() - rendered.width()) / 2, y_centered as u32);
            let rect = sdl2::rect::Rect::new(((surface.width() - rendered.width()) / 2) as i32, y_centered as i32, rendered.width(), rendered.height());
            self.to_screen(&rendered, surface, None, Some(rect));
            y_centered += font.height();
        }
        
    }
    fn render_font(&self, font: &'c sdl2::ttf::Font<'a, 'b>, text: &str) -> sdl2::ttf::FontResult<sdl2::surface::Surface<'d>> {
        font.render(text).blended({
            if self.revert {
                sdl2::pixels::Color::BLACK
            } else {
                sdl2::pixels::Color::WHITE
            }
        })
    }

    fn to_screen(&self, surface: &sdl2::surface::Surface, mut target: &mut sdl2::surface::Surface, dest: Option<(i32, i32)>, rect: Option<sdl2::rect::Rect>) {
        surface.blit(None,&mut target, {
            match (dest, rect) {
                (Some(d), None) => {
                    let (w,h) = surface.size();
                    sdl2::rect::Rect::new(0, 0, w,h)
                },
                (None, Some(r)) => r,
                _ => {
                    panic!("Unexpected!");
                },
            }
        }).unwrap().unwrap();
    }

    fn word_wrap(&self, text: &str, max_width: u32, font: &'c sdl2::ttf::Font<'a, 'b>) -> Vec<String> {
        let words = text.split_whitespace();
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
    
        for word in words {
            let test_line = current_line.clone() + word + " ";
            let (test_width, _) = font.size_of(&test_line).unwrap();
    
            if test_width <= max_width {
                current_line = test_line;
            } else {
                lines.push(current_line.trim_end().to_string());
                current_line = word.to_owned() + " ";
            }
        }
    
        if !current_line.trim().is_empty() {
            lines.push(current_line.trim_end().to_string());
        }
    
        // println!("{:#?}", lines);
        lines
    }
}
