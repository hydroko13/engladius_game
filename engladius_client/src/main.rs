use crate::assets::Assets;

mod game;
mod assets;

fn main() -> Result<(), String> {


    let mut game_instance = game::Game::new()?;

    let ttf_context = sdl2::ttf::init()?;


    

    let mut assets_instance = Assets::load(&ttf_context, game_instance.assets_directory_path.clone())?;

    game_instance.run(&assets_instance)?;

    Ok(())
}
