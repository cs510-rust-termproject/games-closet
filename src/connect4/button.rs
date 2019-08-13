// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
extern crate ggez;

use ggez::graphics;
use ggez::input::mouse;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use super::core::MyColor;

pub const BUTTON_PADDING: (f32, f32) =  (10.0, 10.0);
pub const BUTTON_SPACING: (f32, f32) = (50.0, 50.0);

pub struct Button {
    pub text: graphics::Text,
    pub outline: graphics::Rect,
    background_color: MyColor,
    pub active: bool,
    pub selected: bool,
    pub highlighted: bool,
    highlighted_color: MyColor
}

impl Button {
    pub fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text: text, 
                 outline: dim, 
                 background_color: MyColor::Red,
                 active: true, 
                 selected: false, 
                 highlighted: false,
                 highlighted_color: MyColor::Green
                }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let draw_color = if self.selected || self.highlighted { self.highlighted_color.get_draw_color() } else { self.background_color.get_draw_color() };
            let textbox = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(),             
                self.outline,
                draw_color,
            )?;
            let text_offset = ((self.outline.w - self.text.width(ctx) as f32)/2.0, (self.outline.h - self.text.height(ctx) as f32)/2.0);
            graphics::draw(ctx, &textbox, (Point2 {x: 0.0, y: 0.0},))?;
            graphics::draw(ctx, &self.text, (Point2 {x: self.outline.x + text_offset.0, y: self.outline.y + text_offset.1},))?;
            //println!("{},{}  {},{}", self.outline.x, self.outline.y, self.outline.x - text_offset.0, self.outline.y - text_offset.1);
        }
        Ok(())
    }

    pub fn set_colors(&mut self, bg_color: MyColor, hl_color: MyColor) {
        self.background_color = bg_color;
        self.highlighted_color = hl_color;
    }

    pub fn as_button_under_mouse(&mut self, ctx: &mut Context) -> bool {
        let mouse_loc = mouse::position(ctx);
        if self.active && self.outline.contains(mouse_loc)  {
            self.highlighted = true;
        } else {
            self.highlighted = false;
        }
        self.highlighted
    }
}