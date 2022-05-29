use macroquad::prelude::*;

pub struct Dog {
    pub img: Texture2D,
    pub pos: Vec2,
    pub frame: Rect,
}

impl Dog {
    pub fn update(&mut self) {
        if self.pos.y == 240. {
            self.pos.y = 240.;
        } else {
            self.pos.y -= 1.;
        }

        self.draw();
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            self.img,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    5. * self.img.width() / 6.,
                    self.frame.y,
                    self.img.width() / 6.,
                    self.img.height() / 2.,
                )),
                ..Default::default()
            },
        );
    }
}
