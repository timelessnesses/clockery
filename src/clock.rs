use crate::snow;
use chrono::{self, Datelike, TimeZone};
use sdl2;
use slicestring::Slice;
// use humantime;
// use crate::snow;

pub struct Clock<'a, 'b, 'c, 'd> {
    timezone: Option<chrono_tz::Tz>,
    am_pm: bool,
    name: String,
    revert: bool,
    date_font: &'c sdl2::ttf::Font<'a, 'b>,
    normal_font: &'c sdl2::ttf::Font<'a, 'b>,
    markery: std::marker::PhantomData<&'d str>, // used for 'd for the surface
                                                // snow: snow::SnowParticles,
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
            markery: std::marker::PhantomData,
        };
    }

    pub fn current_datetime_in_timezone(&self) -> chrono::DateTime<chrono::FixedOffset> {
        match self.timezone {
            Some(tz) => {
                // println!("It has timezone");
                tz.from_utc_datetime(&chrono::Utc::now().naive_utc())
                    // .unwrap()
                    .fixed_offset()
            }
            None => {
                // println!("It's none");
                chrono::Local::now().fixed_offset()
            }
        }
    }

    pub fn render(
        &self,
        time: chrono::DateTime<chrono::FixedOffset>,
        surface: &mut sdl2::surface::Surface,
        snow_particle: &mut snow::SnowParticles,
    ) {
        let utcized = time.naive_local();
        let new_year = chrono::NaiveDate::from_ymd_opt(utcized.year() + 1, 1, 1).unwrap();
        let time_new_year = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let built = chrono::NaiveDateTime::new(new_year, time_new_year);

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
        self.center(&timer, surface, self.date_font, Some(middle_y - 100));
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
                format!(
                    "{} ({})",
                    time.format("%A %d/%B/%Y UTC%Z").to_string(),
                    zone
                )
            }
            Err(_) => {
                format!("{} {}", time.format("%A %d/%B/%Y %Z").to_string(), zone)
            }
        };
        self.center(
            format!("Currently {}", offset).as_str(),
            surface,
            self.normal_font,
            Some(middle_y + 20),
        );
        self.center(
            format!("The date is {}", d_thing).as_str(),
            surface,
            self.normal_font,
            Some(middle_y + 70),
        );
        let left = built - utcized;

        // let formatted = humantime::format_duration(left.to_std().unwrap()).to_string();
        let formatted = self.formatter(left);

        self.center(
            formatted.as_str(),
            surface,
            self.normal_font,
            Some(middle_y + 120),
        );

        snow_particle.render(surface, self.revert);
    }

    fn formatter(&self, left: chrono::Duration) -> String {
        let stded = left.to_std().unwrap();
        let seconds = stded.as_secs() as i64;
        let nanoseconds = stded.subsec_millis();

        let years = seconds / (365 * 24 * 60 * 60);
        let remaining_seconds = seconds % (365 * 24 * 60 * 60);

        let months = remaining_seconds / (30 * 24 * 60 * 60);
        let remaining_seconds = remaining_seconds % (30 * 24 * 60 * 60);

        let weeks = remaining_seconds / (7 * 24 * 60 * 60);
        let remaining_seconds = remaining_seconds % (7 * 24 * 60 * 60);

        let days = remaining_seconds / (24 * 60 * 60);
        let remaining_seconds = remaining_seconds % (24 * 60 * 60);

        let hours = remaining_seconds / 3600;
        let remaining_seconds = remaining_seconds % 3600;

        let minutes = remaining_seconds / 60;
        let remaining_seconds = remaining_seconds % 60;

        let mut build = String::new();
        build += "New years: ";

        if left.num_hours() <= 12 {
            // new year 12 hours left
            if years != 0 {
                build += format!("{} years, ", years).as_str();
            }
            if months != 0 {
                build += format!("{} months, ", months).as_str();
            }
            if weeks != 0 {
                build += format!("{} weeks, ", weeks).as_str();
            }
            if days != 0 {
                build += format!("{} days, ", days).as_str();
            }
            build += format!(
                "{} hours, {} minutes, {} seconds, {} miliseconds",
                hours, minutes, remaining_seconds, nanoseconds
            )
            .as_str();
            return build;
        }
        return "".to_string();
    }

    fn center(
        &self,
        text: &str,
        surface: &mut sdl2::surface::Surface,
        font: &'c sdl2::ttf::Font<'a, 'b>,
        y: Option<i32>,
    ) {
        let y = match y {
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
                Err(_) => continue,
            };
            // let rect = sdl2::rect::Rect::new(rendered.width() as i32, rendered.height() as i32, (surface.width() - rendered.width()) / 2, y_centered as u32);
            let rect = sdl2::rect::Rect::new(
                ((surface.width() - rendered.width()) / 2) as i32,
                y_centered as i32,
                rendered.width(),
                rendered.height(),
            );
            self.to_screen(&rendered, surface, None, Some(rect));
            y_centered += font.height();
        }
    }
    fn render_font(
        &self,
        font: &'c sdl2::ttf::Font<'a, 'b>,
        text: &str,
    ) -> sdl2::ttf::FontResult<sdl2::surface::Surface<'d>> {
        font.render(text).blended({
            if self.revert {
                sdl2::pixels::Color::BLACK
            } else {
                sdl2::pixels::Color::WHITE
            }
        })
    }

    fn to_screen(
        &self,
        surface: &sdl2::surface::Surface,
        mut target: &mut sdl2::surface::Surface,
        dest: Option<(i32, i32)>,
        rect: Option<sdl2::rect::Rect>,
    ) {
        surface
            .blit(None, &mut target, {
                match (dest, rect) {
                    (Some(_), None) => {
                        let (w, h) = surface.size();
                        sdl2::rect::Rect::new(0, 0, w, h)
                    }
                    (None, Some(r)) => r,
                    _ => {
                        panic!("Unexpected!");
                    }
                }
            })
            .unwrap()
            .unwrap();
    }

    fn word_wrap(
        &self,
        text: &str,
        max_width: u32,
        font: &'c sdl2::ttf::Font<'a, 'b>,
    ) -> Vec<String> {
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
