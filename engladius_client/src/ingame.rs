use sdl2::{EventPump, keyboard::Scancode, rect::Rect, surface::Surface};


use crate::assets::Assets;

pub struct InGameState {
    player: Player,

}

impl InGameState {
    pub fn new() -> InGameState {



        InGameState {
            player: Player::new(0.0, 0.0)
        }
    }

    pub fn draw(&self, assets: &Assets, game_surf: &mut Surface<'_>, event_pump: &EventPump, view_rect: &Rect, view_scale: i32) -> Result<(), String>{
        assets.player_image.blit(None, game_surf, Rect::new((self.player.x - 8.0) as i32, (self.player.y - 8.0) as i32, 0, 0))?;
        Ok(())
    }

    pub fn update(&mut self, dt: f32, event_pump: &EventPump) {
        let keys = event_pump.keyboard_state();
        if keys.is_scancode_pressed(Scancode::W) {
            self.player.y -= dt * 112.0;
        }
        if keys.is_scancode_pressed(Scancode::S) {
            self.player.y += dt * 112.0;
        }
        if keys.is_scancode_pressed(Scancode::D) {
            self.player.x += dt * 112.0;
        }
        if keys.is_scancode_pressed(Scancode::A) {
            self.player.x -= dt * 112.0;
        }
    }

    pub fn process_left_click(&mut self, x: i32, y: i32, view_rect: &Rect, view_scale: i32) {
        
    }

}

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            x: x,
            y: y
        }
    }
    
}

