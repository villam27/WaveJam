use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct AData {
	start : usize,
	end : usize,
	frame_time : f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FrameTime(f32);

#[derive(Component)]
pub struct CurrAnimId(usize);

#[derive(Component)]
pub struct SpriteAnimationData {
	data: Vec<AData>,
}

/*
	sprite corps
	sprite tete
	animation
 */
#[derive(Bundle)]
struct PlayerBodyBundle {
	player: Player,
	frame_time: FrameTime,
	current_anim: CurrAnimId,
	animation_data: SpriteAnimationData,
	
	#[bundle]
	sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerHeadBundle {
	player: Player,
	frame_time: FrameTime,
	current_anim: CurrAnimId,
	animation_data: SpriteAnimationData,
	
	#[bundle]
	sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerBundle
{
	player: Player,
	transform: SpatialBundle,
}

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(init_player)
			.add_system(animate);
	}
}

fn init_player(mut command : Commands, asset_server: Res<AssetServer>, mut atlases: ResMut<Assets<TextureAtlas>>) {
	let	body_texture: Handle<Image> =  asset_server.load("player/body.png");
	let	body_texture_atlas: TextureAtlas = TextureAtlas::from_grid(body_texture, Vec2::new(48.0, 48.0), 4, 1, None, None);
	let body_texture_handle: Handle<TextureAtlas> = atlases.add(body_texture_atlas);
	let mut body_animation_data: Vec<AData> = Vec::new();
	let	head_texture: Handle<Image> =  asset_server.load("player/head.png");
	let	head_texture_atlas: TextureAtlas = TextureAtlas::from_grid(head_texture, Vec2::new(72.0, 72.0), 4, 1, None, None);
	let head_texture_handle: Handle<TextureAtlas> = atlases.add(head_texture_atlas);
	let mut head_animation_data: Vec<AData> = Vec::new();
	
	body_animation_data.push(AData {start: 0, end: 3, frame_time: 0.15});
	head_animation_data.push(AData {start: 0, end: 3, frame_time: 0.25});

	command.spawn(PlayerBundle 
	{
		player: Player,
		transform: SpatialBundle::default(),
	}).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
	parent.spawn(PlayerBodyBundle {
		player: Player,
		frame_time: FrameTime(0.0),
		current_anim: CurrAnimId(0),
		sprite: SpriteSheetBundle {
			sprite: (TextureAtlasSprite {index: body_animation_data[0].start, ..default()}),
			texture_atlas: (body_texture_handle),
			..default()
		},
		animation_data: SpriteAnimationData { data: body_animation_data },
	});
	}).with_children(|parent| {
	parent.spawn(PlayerHeadBundle {
		player: Player,
		frame_time: FrameTime(0.0),
		current_anim: CurrAnimId(0),
		sprite: SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, 40.0, 0.0),
			sprite: (TextureAtlasSprite {index: head_animation_data[0].start, ..default()}),
			texture_atlas: (head_texture_handle),
			..default()
		},
		animation_data: SpriteAnimationData { data: head_animation_data },
	});
	});
}

pub fn animate(mut query: Query<(&mut TextureAtlasSprite, &mut SpriteAnimationData, &mut FrameTime, &CurrAnimId)>, time: Res<Time>) {
	for (mut sprite, animation, mut frame_time, id) in query.iter_mut() {
		frame_time.0 += time.delta_seconds();
		if frame_time.0 > animation.data[id.0].frame_time
		{
			let frames_passed: usize = (frame_time.0 / animation.data[id.0].frame_time) as usize;
			sprite.index += frames_passed;
			if sprite.index > animation.data[id.0].end {sprite.index = animation.data[id.0].start;}
			frame_time.0 -= animation.data[id.0].frame_time * frames_passed as f32;
		}
	}
}