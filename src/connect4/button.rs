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

///Constant dimmesions for padding between button text and outline
pub const BUTTON_PADDING: (f32, f32) =  (10.0, 10.0);
///Constant dimmesions for spacing between distinct buttons
pub const BUTTON_SPACING: (f32, f32) = (50.0, 50.0);

///
/// A struct representing a button object on a menu or a game
///
/// # Fields
/// * text              = Text object representing text for the button
/// * outline           = Rect object representing background shape of button. Should be at least same dimesnsions as text
/// * background_color  = MyColor object representing background color of button
/// * active            = Boolean indicating if button is visible
/// * selected          = Boolean indicating if button has been clicked     
/// * highlighted       = Boolean indicating if mouse is hovering over the button      
/// * highlighted_color = MyColor object representing color the background is changed to if the button is highlighted or selected     
///
pub struct Button {
    pub text: graphics::Text,
    pub outline: graphics::Rect,
    background_color: MyColor,
    pub active: bool,
    pub selected: bool,
    pub highlighted: bool,
    highlighted_color: MyColor
}

/// Struct used for creating buttons used in the main menu and connect 4 game
impl Button {
    pub fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text, 
                 outline: dim, 
                 background_color: MyColor::Red,
                 active: true, 
                 selected: false, 
                 highlighted: false,
                 highlighted_color: MyColor::Green
                }
    }

    ///Draw method for rendering button
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
        }
        Ok(())
    }

    ///Method to set the background color of button normally and when highlighted
    pub fn set_colors(&mut self, bg_color: MyColor, hl_color: MyColor) {
        self.background_color = bg_color;
        self.highlighted_color = hl_color;
    }

    ///Method to determine if mouse if hovering over button, updates highlighted state accordingly
    pub fn is_button_under_mouse(&mut self, ctx: &mut Context) -> bool {
        let mouse_loc = mouse::position(ctx);
        if self.active && self.outline.contains(mouse_loc)  {
            self.highlighted = true;
        } else {
            self.highlighted = false;
        }
        self.highlighted
    }
}