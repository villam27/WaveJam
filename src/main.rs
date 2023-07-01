use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_ldtk::prelude::*;

mod game;
mod map;
mod player;

struct MainPlugin;

impl PluginGroup for MainPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(game::GamePlugin)
            .add(player::PlayerPlugin)
    }
}

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 360.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: ((WIDTH, HEIGHT).into()),
                        title: ("Wave Jam".into()),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<map::WallBundle>(1)
        .add_plugins(MainPlugin)
        // .add_system(map::print_walls)
        .run();
}
