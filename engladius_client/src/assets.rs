use std::path::PathBuf;

use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Assets<'a> {
    pub font_title: Font<'a, 'a>,
    pub font_normal: Font<'a, 'a>,
}

impl<'a> Assets<'a> {
    pub fn load(ttf_context: &Sdl2TtfContext, assets_path: PathBuf) -> Result<Assets<'_>, String> {

        let font_title = ttf_context.load_font(assets_path.join("m3x6.ttf"), 32)?;
        let font_normal = ttf_context.load_font(assets_path.join("m3x6.ttf"), 16)?;

        Ok(Assets {
            font_normal,
            font_title

        })
    }
}