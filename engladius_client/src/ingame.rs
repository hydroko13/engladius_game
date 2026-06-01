use sdl2::{EventPump, rect::Rect, surface::Surface};


use crate::assets::Assets;

pub struct InGameState {

}

impl InGameState {
    pub fn new() -> InGameState {



        InGameState {

        }
    }

    pub fn draw(&self, game_surf: &mut Surface<'_>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) {

    }

    pub fn update(&mut self, dt: f32) {
        
    }

}