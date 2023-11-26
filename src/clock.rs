use std::rc::Rc;
use sdl2::ttf::{Font, Sdl2TtfContext};
use chrono::{self, TimeZone};

struct Clock {
    timezone: Option<chrono_tz::Tz>,
    am_pm: bool,
    name: String,
    revert: bool,
    date_font: sdl2::ttf::Font<'static, 'static>,
    normal_font: sdl2::ttf::Font<'static, 'static>,
}

impl Clock {
    pub fn new(timezone: Option<chrono_tz::Tz>, am_pm: bool, revert: bool, date_font: sdl2::ttf::Font<'static, 'static>, normal_font: sdl2::ttf::Font<'static, 'static>) -> Self {
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

    fn current_datetime_in_timezone(self, timezone: Option<chrono_tz::Tz>) -> chrono::DateTime<chrono::FixedOffset> {
        match timezone {
            Some(tz) => {
                // Get the current time in UTC
                let utc_now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    
                // Convert the UTC time to the target timezone
                let tz_offset: chrono_tz::Tz = tz.offset_from_utc_datetime(&utc_now.naive_utc());
                let fixed_offset: chrono::FixedOffset = chrono::FixedOffset::east(tz_offset.fix().local_minus_utc());
    
                let datetime_in_target_timezone: chrono::DateTime<chrono::FixedOffset> =
                    utc_now.with_timezone(&fixed_offset);
    
                datetime_in_target_timezone
            }
            None => {
                // Get the current time in the local timezone
                chrono::Local::now().into()
            }
        }
    }
    
    
        
}
