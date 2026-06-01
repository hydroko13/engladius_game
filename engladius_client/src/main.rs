use crate::{assets::Assets, main_menu::MainMenuState};


mod game;
mod assets;
mod gui;
mod ingame;
mod main_menu;

fn main() -> Result<(), String> {

    let engladius_client_version = "0.1-beta".to_string();


    let mut game_instance = game::Game::new(engladius_client_version.clone())?;
    

    let ttf_context = sdl2::ttf::init()?;
    let image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    

    

    let mut assets_instance = Assets::load(&ttf_context, &image_context, game_instance.assets_directory_path.clone())?;

    let mut main_menu = MainMenuState::new(&assets_instance)?;

    

    game_instance.game_state = game::GameState::MainMenu(main_menu);

    game_instance.run(&assets_instance)?;

    Ok(())
}
