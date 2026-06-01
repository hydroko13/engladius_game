use sdl2::{EventPump, keyboard::Scancode::Menu, pixels::Color, rect::{Point, Rect}, surface::Surface};

use crate::{assets::Assets, main};

pub struct Button<'a> {
    x: i32,
    y: i32,
    text: String,
    rendered_text: Surface<'a>,
    rendererd_text_shadow: Surface<'a>
}

impl<'a> Button<'a> {
    pub fn new(x: i32, y: i32, text: String, assets: &Assets) -> Result<Button<'a>, String> {

        let rendered_text_surf = assets.font_normal.render(text.as_str()).solid(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;

        let rendered_text_shadow = assets.font_normal.render(text.as_str()).solid(Color::RGB(0, 0, 0)).map_err(|e| e.to_string())?;

        




        let button = Button {
            x,
            y,
            text,
            rendered_text: rendered_text_surf,
            rendererd_text_shadow: rendered_text_shadow

        };

        Ok(button)
    }

    pub fn draw(&self, mut game_surf: &mut Surface<'a>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) -> Result<(), String> {


        let window_mouse_pos = event_pump.mouse_state();

        let game_mouse_pos = ((window_mouse_pos.x() - view_rect.x) / view_scale, (window_mouse_pos.y() - view_rect.y) / view_scale);
        
        let mut hover_rect = self.rendered_text.rect();  

        hover_rect.center_on(Point::new(self.x, self.y));      

        if hover_rect.contains_point(game_mouse_pos) {
            self.rendererd_text_shadow.blit(None, &mut game_surf, Some(Rect::new(self.x - self.rendered_text.width() as i32 / 2 + 2, self.y - self.rendered_text.height() as i32 / 2 + 3, 0, 0)))?;
            self.rendered_text.blit(None, &mut game_surf, Some(Rect::new(self.x - self.rendered_text.width() as i32 / 2 - 1, self.y - self.rendered_text.height() as i32 / 2 - 1, 0, 0)))?;  
        } else {
            self.rendererd_text_shadow.blit(None, &mut game_surf, Some(Rect::new(self.x - self.rendered_text.width() as i32 / 2 + 1, self.y - self.rendered_text.height() as i32 / 2 + 2, 0, 0)))?;
            self.rendered_text.blit(None, &mut game_surf, Some(Rect::new(self.x - self.rendered_text.width() as i32 / 2, self.y - self.rendered_text.height() as i32 / 2, 0, 0)))?;  
        }

        
        

        Ok(())
    }
}

pub struct MainMenuGui<'a> {
    play_button: Button<'a>,
}

impl<'a> MainMenuGui<'a> {
    pub fn new(assets: &Assets) -> Result<MainMenuGui<'a>, String> {

        let play_button = Button::new(384 / 2, 216 / 2 + 60, String::from("Play"), assets)?;

        let main_menu = MainMenuGui {
            play_button
        };

        Ok(main_menu)
    }

    pub fn draw(&self, game_surf: &mut Surface<'a>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) -> Result<(), String> {

        self.play_button.draw(game_surf, event_pump, view_rect, view_scale)?;

        Ok(())
    }
}