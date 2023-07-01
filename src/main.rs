use bevy::prelude::*;
use player::PlayerPlugin;
mod player;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
