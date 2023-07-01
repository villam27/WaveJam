use bevy::prelude::*;

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
		.run();
}
