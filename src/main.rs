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

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 360.;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins
			.set(ImagePlugin::default_nearest())
			.set(WindowPlugin {
				primary_window : Some(Window {
					resolution: ((WIDTH, HEIGHT).into()), title: ("Wave Jam".into()), resizable: (false.into()), ..default()}), ..default()
				}))
		.add_plugins(MainPlugin)
		.run();
}
