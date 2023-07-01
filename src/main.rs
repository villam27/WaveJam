use bevy::{prelude::*, app::PluginGroupBuilder};
use game::GamePlugin;
use player::PlayerPlugin;

mod player;
mod game;

struct MainPlugin;

impl PluginGroup for MainPlugin {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(GamePlugin)
			.add(PlayerPlugin)
	}
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainPlugin)
        .run();
}
