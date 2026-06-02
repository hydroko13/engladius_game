use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use sdl2::controller::Button::Paddle4;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::sys::SDL_Renderer;
use sdl2::ttf::{Font, Sdl2TtfContext, init};
use sdl2::video::{DisplayMode, Window, WindowContext, WindowSurfaceRef};
use sdl2::{EventPump, Sdl, VideoSubsystem, gfx};

use crate::assets::Assets;
use crate::game;
use crate::ingame::InGameState;
use crate::main_menu::MainMenuEvent::PressedPlay;
use crate::main_menu::MainMenuState;


pub enum GameState<'a> {
    LoadingGame,
    MainMenu(MainMenuState<'a>),
    InGame(InGameState)
}

pub struct Game<'a> {
    pub sdl_context: Sdl,
    monitor_display_mode: DisplayMode,
    video_subsystem: VideoSubsystem,
    window: Window,
    pub game_surf: Surface<'a>,
    event_pump: EventPump,
    done: bool,
    delta: f32,
    pub assets_directory_path: PathBuf,
    pub game_state: GameState<'a>,
    pub game_version: String
}


impl<'a> Game<'a> {
    pub fn new(game_version: String) -> Result<Game<'a>, String> {
        let sdl_context = sdl2::init()?;

        let start_fullscreen = false;

        let video_subsystem = sdl_context.video()?;

        let monitor_display_mode = video_subsystem.desktop_display_mode(0).unwrap();
        let mut window = if start_fullscreen {
            video_subsystem
            .window(
                "Engladius",
                monitor_display_mode.w as u32,
                monitor_display_mode.h as u32,
            )
            .position(20, 20)
            .build()
            .unwrap()

        } else {
            video_subsystem
            .window(
                "Engladius",
                960,
                600,
            )
            .position(20, 20)
            .resizable()

            .build()
            .unwrap()

        };
        window.set_minimum_size(384, 216);
        if start_fullscreen {
            window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
        }
        
        let event_pump = sdl_context.event_pump().unwrap();

        let game_surf = Surface::new(384, 216, sdl2::pixels::PixelFormatEnum::RGB24)?;

        // INIT FONT

        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;

        let assets_path = cwd.join("assets/");



        let game = Game {
            sdl_context: sdl_context,
            window: window,
            game_version,
            video_subsystem: video_subsystem,
            monitor_display_mode: monitor_display_mode,
            game_surf: game_surf,
            event_pump: event_pump,
            done: false,
            delta: 0.0,
            assets_directory_path: assets_path,
            game_state: GameState::LoadingGame
        };

        Ok(game)
    }



    pub fn draw(&mut self, assets: &Assets, view_rect: &Rect, view_scale: i32) -> Result<(), String> {

        
        self.game_surf.fill_rect(None, Color::RGB(80, 150, 80))?;

        let game_title = &("Engladius ".to_owned() + &self.game_version);
        

    
        match &self.game_state {
            GameState::MainMenu(main_menu) => {
                main_menu.draw(&mut self.game_surf, &self.event_pump, view_rect, view_scale)?;
            },
            GameState::InGame(in_game) => {
                in_game.draw(assets, &mut self.game_surf, &self.event_pump, view_rect, view_scale)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn run(&mut self, assets: &Assets) -> Result<(), String> {
        let mut last_time: Instant = Instant::now();
        let target_fps: f64 = 100.0;
        let target_frame_dur = (1.0 / target_fps);

        while !self.done {
            let now: Instant = Instant::now();

            self.delta = now.duration_since(last_time).as_secs_f32();

            last_time = Instant::now();

            let window_size = self.window.size();

            let window_center = (window_size.0 / 2, window_size.1 / 2);

            let scale: i32 = (window_size.0 as i32 / 384).min((window_size.1 as i32 / 216));

            let game_rect = Rect::new(window_center.0 as i32 - (384 * scale) / 2 , window_center.1 as i32 - (216 * scale) / 2, 384 * scale as u32, 216 * scale as u32);
            

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => self.done = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.done = true;
                    }
                    Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                        match &mut self.game_state {
                            GameState::MainMenu(main_menu) => {
                                main_menu.process_left_click(x, y, &game_rect, scale);
                            },
                            GameState::InGame(in_game_state) => {
                                in_game_state.process_left_click(x, y, &game_rect, scale);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            

            match &mut self.game_state {
                GameState::InGame(in_game_state) => {
                    in_game_state.update(self.delta, &self.event_pump);
                }

                GameState::MainMenu(main_menu) => {
                    match main_menu.update(self.delta) {
                        PressedPlay => {
                            let in_game_state = InGameState::new();
                            self.game_state = GameState::InGame(in_game_state);
                        },
                        _ => {}
                    }
                }

                _ => {}
            }



            self.draw(assets, &game_rect, scale)?;

            let mut winsurf = self.window.surface(&self.event_pump)?;

            
            winsurf.fill_rect(None,Color::BLACK)?;

            self.game_surf.blit_scaled(None, &mut winsurf, Some(game_rect))?;

            winsurf.update_window()?;

            println!("{}", 1.0/self.delta);

            let now: Instant = Instant::now();

            let mut remaining_time = target_frame_dur - now.duration_since(last_time).as_secs_f64();

            if remaining_time > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(remaining_time));
            }

           

        

            

            
        }

        Ok(())
    }
}

