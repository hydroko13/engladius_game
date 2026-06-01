use crate::{assets::Assets, gui::MainMenuGui};

mod game;
mod assets;
mod gui;

fn main() -> Result<(), String> {

    let engladius_client_version = "0.1-beta".to_string();


    let mut game_instance = game::Game::new(engladius_client_version.clone())?;

    let ttf_context = sdl2::ttf::init()?;


    

    let mut assets_instance = Assets::load(&ttf_context, game_instance.assets_directory_path.clone())?;

    game_instance.game_state = game::GameState::MainMenu(MainMenuGui::new(&assets_instance)?);

    game_instance.run(&assets_instance)?;

    Ok(())
}
