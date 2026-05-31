use std::path::{Path, PathBuf};
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext, init};
use sdl2::video::{DisplayMode, Window, WindowContext, WindowSurfaceRef};
use sdl2::{EventPump, Sdl, VideoSubsystem, gfx};

use crate::assets::Assets;

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
}

impl<'a> Game<'a> {
    pub fn new() -> Result<Game<'a>, String> {
        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let monitor_display_mode = video_subsystem.desktop_display_mode(0).unwrap();

        let mut window = video_subsystem
            .window(
                "Hello world",
                monitor_display_mode.w as u32,
                monitor_display_mode.h as u32,
            )
            .position(20, 20)
            .build()
            .unwrap();

        window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
        let event_pump = sdl_context.event_pump().unwrap();

        let game_surf = Surface::new(384, 216, sdl2::pixels::PixelFormatEnum::RGB24)?;

        // INIT FONT

        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;

        let assets_path = cwd.join("assets/");

        

        let game = Game {
            sdl_context: sdl_context,
            window: window,
            video_subsystem: video_subsystem,
            monitor_display_mode: monitor_display_mode,
            game_surf: game_surf,
            event_pump: event_pump,
            done: false,
            delta: 0.0,
            assets_directory_path: assets_path,
        };

        Ok(game)
    }

    pub fn draw(&mut self, assets: &Assets) -> Result<(), String> {

        
        self.game_surf.fill_rect(None, Color::RGB(80, 150, 80))?;
        

        let title_text_surface = assets
            .font_title
            .render("Engladius_")
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;

        let title_text_surface_black: Surface<'_> = assets
            .font_title
            .render("Engladius_")
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;

        title_text_surface_black.blit(None, &mut self.game_surf, Some(Rect::new(384 / 2 - title_text_surface_black.width() as i32 / 2 + 1, 15 - title_text_surface_black.height() as i32 / 2 + 1, 2, 2)))?;
        title_text_surface.blit(None, &mut self.game_surf, Some(Rect::new(384 / 2 - title_text_surface.width() as i32 / 2, 15 - title_text_surface.height() as i32 / 2, 2, 2)))?;
        
        
        // let title_text_surface = assets
        //     .font1
        //     .render("Hello world!")
        //     .blended(Color::RGB(255, 255, 255))
        //     .map_err(|e| e.to_string())?;

        // // self.game_surf.fill_rect()

        // title_text_surface.blit(None, &mut self.game_surf, Some(Rect::new(384 / 2 - text_surface.width() as i32 / 2, 216 / 2 - text_surface.height() as i32 / 2, 2, 2)))?;
        

        Ok(())
    }

    pub fn run(&mut self, assets: &Assets) -> Result<(), String> {
        let mut last_time = Instant::now();

        while !self.done {
            let now: Instant = Instant::now();

            self.delta = now.duration_since(last_time).as_secs_f32();

            last_time = Instant::now();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => self.done = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.done = true;
                    }
                    _ => {}
                }
            }

            


            self.draw(assets)?;

            let mut winsurf = self.window.surface(&self.event_pump)?;

            let window_rect = winsurf.rect();

            let window_center = window_rect.center();

            let scale: i32 = (window_rect.w / 384).min((window_rect.h / 216));

            let game_rect = Rect::new(window_center.x - (384 * scale) / 2 , window_center.y - (216 * scale) / 2, 384 * scale as u32, 216 * scale as u32);

            self.game_surf.blit_scaled(None, &mut winsurf, Some(game_rect))?;

            winsurf.update_window()?;

            

            
        }

        Ok(())
    }
}

