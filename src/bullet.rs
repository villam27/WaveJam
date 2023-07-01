use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletDirection(pub Vec3);

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub direction: BulletDirection,

    #[bundle]
    pub sprite: SpriteBundle,
}
