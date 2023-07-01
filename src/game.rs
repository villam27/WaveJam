use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.055, 0.957, 0.988);
pub struct GamePlugin;

#[derive(Component)]
struct MainCamera;

fn start_game(mut command: Commands) {
    command.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(start_game);
    }
}
