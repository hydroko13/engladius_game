use crate::{assets::Assets, gui::MainMenuGui};

mod game;
mod assets;
mod gui;

fn main() -> Result<(), String> {


    let mut game_instance = game::Game::new()?;

    let ttf_context = sdl2::ttf::init()?;


    

    let mut assets_instance = Assets::load(&ttf_context, game_instance.assets_directory_path.clone())?;

    game_instance.main_menu_gui = Some(MainMenuGui::new(&assets_instance)?);

    game_instance.run(&assets_instance)?;

    Ok(())
}
