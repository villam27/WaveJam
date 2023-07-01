use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.055, 0.957, 0.988);
pub struct GamePlugin;

#[derive(Component)]
struct MainCamera;

fn start_game(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn((Camera2dBundle { ..default() }, MainCamera));
    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("top.ldtk"),
        ..Default::default()
    });
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(start_game);
    }
}
