use sdl2;
use chrono::{self, TimeZone};

pub struct Clock<'a, 'b, 'c> {
    timezone: Option<chrono_tz::Tz>,
    am_pm: bool,
    name: String,
    revert: bool,
    date_font: &'c sdl2::ttf::Font<'a, 'b>,
    normal_font: &'c sdl2::ttf::Font<'a, 'b>,
}

impl<'a, 'b, 'c> Clock<'a, 'b, 'c> {
    pub fn new(timezone: Option<chrono_tz::Tz>, am_pm: bool, revert: bool, date_font: &'c sdl2::ttf::Font<'a, 'b>, normal_font: &'c sdl2::ttf::Font<'a, 'b>) -> Self {
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
            normal_font
        }
    }

    pub fn current_datetime_in_timezone(self, timezone: Option<chrono_tz::Tz>) -> chrono::DateTime<chrono_tz::Tz> {
        timezone.unwrap().from_local_datetime(&chrono::Local::now().naive_local()).unwrap()
    }
    
    
        
}