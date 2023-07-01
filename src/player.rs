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
struct PlayerBundle {
	player: Player,
	frame_time: FrameTime,
	current_anim: CurrAnimId,
	animation_data: SpriteAnimationData,
	
	#[bundle]
	sprite: SpriteSheetBundle,
}

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(init_player);
	}
}

fn init_player(mut command : Commands, asset_server: Res<AssetServer>, mut atlases: ResMut<Assets<TextureAtlas>>) {
	let	body_texture: Handle<Image> =  asset_server.load("player/body.png");
	let	body_texture_atlas: TextureAtlas = TextureAtlas::from_grid(body_texture, Vec2::new(48.0, 48.0), 4, 1, None, None);
	let body_texture_handle: Handle<TextureAtlas> = atlases.add(body_texture_atlas);
	let body_curr_anim: usize = 0;
	let mut body_animation_data: Vec<AData> = Vec::new();
	
	body_animation_data.push(AData {start: 0, end: 3, frame_time: 0.25});
	command.spawn(PlayerBundle {
		player: Player,
		frame_time: FrameTime(0.0),
		current_anim: CurrAnimId(body_curr_anim),
		sprite: SpriteSheetBundle {
			sprite: (TextureAtlasSprite {index: body_animation_data[body_curr_anim].start, ..default()}),
			texture_atlas: (body_texture_handle),
			..default()
		},
		animation_data: SpriteAnimationData { data: body_animation_data },
	});
}