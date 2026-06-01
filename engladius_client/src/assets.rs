use std::path::PathBuf;

use sdl2::{image::{LoadSurface, Sdl2ImageContext}, surface::Surface, ttf::{Font, Sdl2TtfContext}};

pub struct Assets<'a> {
    pub font_normal: Font<'a, 'a>,
    pub player_image: Surface<'a>
}

impl<'a> Assets<'a> {
    pub fn load(ttf_context: &'a Sdl2TtfContext, image_context: &Sdl2ImageContext, assets_path: PathBuf) -> Result<Assets<'a>, String> {
        let font_normal = ttf_context.load_font(assets_path.join("m3x6.ttf"), 32)?;
        let player_image = Surface::from_file(assets_path.join("player.png"))?;

        Ok(Assets {
            font_normal,
            player_image
        })
    }
}