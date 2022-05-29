use macroquad::prelude::*;

pub struct Bird {
    pub img: Texture2D,
    pub fall_img: Texture2D,
    pub die: bool,
    pub pos: Vec2,
    pub frame: Rect,
    pub fall_frame: Rect,
    pub last_update: f64,
    pub last_fall: f64,
    pub turn: bool,
    pub speed: Vec2,
    pub rect: Rect,
    pub fall: bool,
    pub count: i32,
}

impl Bird {
    pub fn animation(&mut self) {
        if get_time() - self.last_update > 0.2 {
            self.last_update = get_time();

            self.frame.x += 1.;

            if self.frame.x > 2. {
                self.frame.x = 0.;
            }
        }
    }

    pub fn update(&mut self) {
        if !self.fall {
            self.animation();

            if !self.turn {
                self.speed.x = 1.;
            } else {
                self.speed.x = -1.;
            }

            if self.pos.x > screen_width() - self.img.width() / 3. || self.pos.x < 0. {
                self.speed.x *= -1.;
                self.turn = !self.turn;
            }
        } else {
            self.speed.x = 0.;
            self.speed.y = -2.;

            if get_time() - self.last_fall > 0.2 && self.count > 0 {
                self.last_fall = get_time();
                self.count -= 1;
            }

            if self.count == 0 {
                self.fall_frame.x = 1.;
            }
        }

        if self.pos.y > 380. {
            self.die = true;
        }

        self.pos.x += self.speed.x;
        self.pos.y += -self.speed.y;

        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;
        self.rect.w = self.img.width() / 3.;
        self.rect.h = self.img.height();

        self.draw();
    }

    pub fn draw(&mut self) {
        // draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 5.,YELLOW);
        if !self.fall {
            draw_texture_ex(
                self.img,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        self.frame.x * self.img.width() / 3.,
                        self.frame.y,
                        self.img.width() / 3.,
                        self.img.height(),
                    )),
                    flip_x: self.turn,
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                self.fall_img,
                self.pos.x,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        self.fall_frame.x * self.fall_img.width() / 2.,
                        self.fall_frame.y,
                        self.fall_img.width() / 2.,
                        self.fall_img.height(),
                    )),
                    ..Default::default()
                },
            );
        }
    }
}
