use rand::{self, Rng};
use sdl2;

// const SNOWFLAKE_SIZE: int = 5;

#[derive(Clone, Copy)]
pub struct Snowflake {
    x: i32,
    y: i32,
    size: i32,
    window_width: i32,
    window_height: i32,
    speed: i32,
}

impl Snowflake {
    pub fn new(w: i32, h: i32, size: i32) -> Self {
        let mut rng = rand::thread_rng();
        return Snowflake {
            x: rng.gen_range(0..w+10),
            y: -2,
            speed: rng.gen_range(1..4),
            window_height: h,
            window_width: w + 10,
            size,
        };
    }

    pub fn update(&mut self) {
        self.y += self.speed;
        let mut rng = rand::thread_rng();
        if self.y > self.window_height {
            self.y = -2;
            self.x = rng.gen_range(0..self.window_width);
            self.speed = rng.gen_range(1..4);
        }
    }

    pub fn render(&self, surface: &mut sdl2::surface::Surface, revert: bool) {
        surface
            .fill_rect(
                sdl2::rect::Rect::new(self.x, self.y, self.size as u32, self.size as u32),
                {
                    if !revert {
                        sdl2::pixels::Color::WHITE
                    } else {
                        sdl2::pixels::Color::BLACK
                    }
                },
            )
            .unwrap();
    }
}

#[derive(Clone)]
pub struct SnowParticles {
    snowflakes: Vec<Snowflake>,
}

impl SnowParticles {
    pub fn new(limit: i32, surface: &mut sdl2::surface::Surface) -> Self {
        let mut snowflakes: Vec<Snowflake> = Vec::new();
        for _ in 0..limit {
            snowflakes.push(Snowflake::new(
                surface.width() as i32,
                surface.height() as i32,
                5,
            ))
        }
        return Self { snowflakes };
    }

    pub fn render(&mut self, surface: &mut sdl2::surface::Surface, revert: bool) {
        for snow in self.snowflakes.iter_mut() {
            snow.update();
            snow.render(surface, revert)
        }
    }
}
