use bevy::prelude::*;

use crate::bullet::{BulletBundle};

pub struct PlayerPlugin;

pub struct AData {
    start: usize,
    end: usize,
    frame_time: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerHead;

#[derive(Component)]
pub struct FrameTime(f32);

#[derive(Component)]
pub struct CurrAnimId(usize);

#[derive(Component)]
pub struct PlayerSpeed(Vec3);

#[derive(Component)]
pub struct SpriteAnimationData {
    data: Vec<AData>,
}

#[derive(Component)]
pub struct PlayerShoot {
    time: Timer,
}

/*
   sprite corps
   sprite tete
   animation
*/
#[derive(Bundle)]
struct PlayerBodyBundle {
    body: PlayerBody,
    frame_time: FrameTime,
    current_anim: CurrAnimId,
    animation_data: SpriteAnimationData,

    #[bundle]
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerHeadBundle {
    head: PlayerHead,
    frame_time: FrameTime,
    current_anim: CurrAnimId,
    animation_data: SpriteAnimationData,

    #[bundle]
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: SpatialBundle,
    speed: PlayerSpeed,
    shooting_time: PlayerShoot,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_player)
            .add_system(player_input)
            .add_system(change_head)
            .add_system(change_body)
            .add_system(shoot)
            .add_system(animate);
    }
}

fn init_player(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let body_texture: Handle<Image> = asset_server.load("player/body.png");
    let body_texture_atlas: TextureAtlas =
        TextureAtlas::from_grid(body_texture, Vec2::new(48.0, 48.0), 4, 1, None, None);
    let body_texture_handle: Handle<TextureAtlas> = atlases.add(body_texture_atlas);
    let mut body_animation_data: Vec<AData> = Vec::new();
    let head_texture: Handle<Image> = asset_server.load("player/head.png");
    let head_texture_atlas: TextureAtlas =
        TextureAtlas::from_grid(head_texture, Vec2::new(72.0, 72.0), 4, 1, None, None);
    let head_texture_handle: Handle<TextureAtlas> = atlases.add(head_texture_atlas);
    let mut head_animation_data: Vec<AData> = Vec::new();

    body_animation_data.push(AData {
        start: 0,
        end: 0,
        frame_time: 0.15,
    });
    body_animation_data.push(AData {
        start: 0,
        end: 3,
        frame_time: 0.15,
    });
    head_animation_data.push(AData {
        start: 0,
        end: 0,
        frame_time: 0.25,
    });
    head_animation_data.push(AData {
        start: 1,
        end: 1,
        frame_time: 0.25,
    });
    head_animation_data.push(AData {
        start: 2,
        end: 2,
        frame_time: 0.25,
    });
    head_animation_data.push(AData {
        start: 3,
        end: 3,
        frame_time: 0.25,
    });
    command
        .spawn(PlayerBundle {
            player: Player,
            transform: SpatialBundle::default(),
            speed: PlayerSpeed(Vec3::new(220., 220., 0.)),
            shooting_time: PlayerShoot {
                time: Timer::from_seconds(0.2, TimerMode::Once),
            },
        })
        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
            parent.spawn(PlayerBodyBundle {
                body: PlayerBody,
                frame_time: FrameTime(0.0),
                current_anim: CurrAnimId(0),
                sprite: SpriteSheetBundle {
                    sprite: (TextureAtlasSprite {
                        index: body_animation_data[0].start,
                        ..default()
                    }),
                    texture_atlas: (body_texture_handle),
                    ..default()
                },
                animation_data: SpriteAnimationData {
                    data: body_animation_data,
                },
            });
        })
        .with_children(|parent| {
            parent.spawn(PlayerHeadBundle {
                head: PlayerHead,
                frame_time: FrameTime(0.0),
                current_anim: CurrAnimId(0),
                sprite: SpriteSheetBundle {
                    transform: Transform::from_xyz(0.0, 40.0, 0.0),
                    sprite: (TextureAtlasSprite {
                        index: head_animation_data[0].start,
                        ..default()
                    }),
                    texture_atlas: (head_texture_handle),
                    ..default()
                },
                animation_data: SpriteAnimationData {
                    data: head_animation_data,
                },
            });
        });
}

pub fn animate(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut SpriteAnimationData,
        &mut FrameTime,
        &CurrAnimId,
    )>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time, id) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.data[id.0].frame_time {
            let frames_passed: usize = (frame_time.0 / animation.data[id.0].frame_time) as usize;
            sprite.index += frames_passed;
            if sprite.index > animation.data[id.0].end {
                sprite.index = animation.data[id.0].start;
            }
            frame_time.0 -= animation.data[id.0].frame_time * frames_passed as f32;
        }
    }
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    speed_query: Query<&mut PlayerSpeed, With<Player>>,
) {
    let mut transform = query.single_mut();
    let speed = speed_query.single();

    if keys.pressed(KeyCode::D) {
        transform.translation.x += speed.0.x * time.delta_seconds();
    } else if keys.pressed(KeyCode::Q) {
        transform.translation.x -= speed.0.x * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Z) {
        transform.translation.y += speed.0.x * time.delta_seconds();
    } else if keys.pressed(KeyCode::S) {
        transform.translation.y -= speed.0.x * time.delta_seconds();
    }
}

pub fn change_head(
    keys: Res<Input<KeyCode>>,
    mut head: Query<&mut CurrAnimId, With<PlayerHead>>,
    mut tex: Query<(&mut TextureAtlasSprite, &SpriteAnimationData), With<PlayerHead>>,
) {
    let mut head_id = head.single_mut();
    let (mut texture, data) = tex.single_mut();

    if keys.pressed(KeyCode::Right) {
        head_id.0 = 1;
    }
    if keys.pressed(KeyCode::Left) {
        head_id.0 = 2;
    }
    if keys.pressed(KeyCode::Up) {
        head_id.0 = 3;
    }
    if keys.pressed(KeyCode::Down) {
        head_id.0 = 0;
    }
    texture.index = data.data[head_id.0].start;
}

pub fn shoot(
    mut command: Commands,
    keys: Res<Input<KeyCode>>,
    pos: Query<&Transform, With<Player>>,
    mut timer_query: Query<&mut PlayerShoot, With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let pos = pos.single();
    let mut timer = timer_query.single_mut();

    timer.time.tick(time.delta());
    if keys.pressed(KeyCode::Right) {
        if timer.time.finished() {
            command.spawn(BulletBundle::create_from_position(
                pos.translation,
				Vec2 {x: 450.0, y:0.0},
                asset_server.load("player/bullet.png"),
            ));
            timer.time.reset();
        }
    }
	if keys.pressed(KeyCode::Left) {
        if timer.time.finished() {
            command.spawn(BulletBundle::create_from_position(
                pos.translation,
				Vec2 {x: -450.0, y:0.0},
                asset_server.load("player/bullet.png"),
            ));
            timer.time.reset();
        }
    }
	if keys.pressed(KeyCode::Up) {
        if timer.time.finished() {
            command.spawn(BulletBundle::create_from_position(
                pos.translation,
				Vec2 {y: 450.0, x:0.0},
                asset_server.load("player/bullet.png"),
            ));
            timer.time.reset();
        }
    }
	if keys.pressed(KeyCode::Down) {
        if timer.time.finished() {
            command.spawn(BulletBundle::create_from_position(
                pos.translation,
				Vec2 {y: -450.0, x:0.0},
                asset_server.load("player/bullet.png"),
            ));
            timer.time.reset();
        }
    }
}

pub fn change_body(keys: Res<Input<KeyCode>>, mut body: Query<&mut CurrAnimId, With<PlayerBody>>) {
    let mut body_id = body.single_mut();

    if keys.any_pressed([KeyCode::Q, KeyCode::Z, KeyCode::D, KeyCode::S]) {
        body_id.0 = 1;
    } else {
        body_id.0 = 0;
    }
}
