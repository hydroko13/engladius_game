use sdl2::{EventPump, rect::Rect, surface::Surface};

use crate::{assets::Assets, gui::Gui};

pub struct MainMenuState<'a> {
    pub main_menu_gui: Gui<'a>,

}

impl<'a> MainMenuState<'a> {
    pub fn new() -> MainMenuState<'a> {



        MainMenuState {
            main_menu_gui: Gui::new()
        }
    }

    pub fn draw(&self, game_surf: &mut Surface<'a>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) -> Result<(), String>{
        self.main_menu_gui.draw(game_surf, event_pump, view_rect, view_scale)?;
        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        
    }

}