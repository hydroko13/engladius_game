use sdl2::{EventPump, pixels::Color, rect::Rect, surface::Surface};

use crate::{assets::Assets, gui::{Gui, GuiEvent}};

pub struct MainMenuState<'a> {
    pub main_menu_gui: Gui<'a>,
    play_button_id: u32,


}

pub enum MainMenuEvent {
    PressedPlay,
    PressedQuit,
    Nothing
}

impl<'a> MainMenuState<'a> {
    pub fn new(assets: &Assets) -> Result<MainMenuState<'a>, String> {

        let mut gui = Gui::new();

        let play_button_id = gui.add_button(100, 100, "play".to_string(), assets)?;

        Ok(MainMenuState {
            main_menu_gui: gui,
            play_button_id
        })
    }

    pub fn draw(&self, game_surf: &mut Surface<'a>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) -> Result<(), String>{
        
        self.main_menu_gui.draw(game_surf, event_pump, view_rect, view_scale)?;
        Ok(())
    }

    pub fn update(&mut self, dt: f32) -> MainMenuEvent {
        for event in self.main_menu_gui.gui_events.drain(..) {
            match event {
                GuiEvent::ButtonClicked { element_id } => {
                    if element_id == self.play_button_id {
                        return MainMenuEvent::PressedPlay
                    }
                }
            }
        }

        MainMenuEvent::Nothing
    }

    pub fn process_left_click(&mut self, x: i32, y: i32, view_rect: &Rect, view_scale: i32) {
        self.main_menu_gui.process_left_click(x, y, view_rect, view_scale);
    }

}