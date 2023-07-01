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

impl BulletBundle {
    pub fn create_from_position(translation: Vec3, direction:Vec2, img: Handle<Image>) -> BulletBundle {
        BulletBundle {
            bullet: Bullet,
            direction: BulletDirection(Vec3::new(direction.x, direction.y, 0.)),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(translation.x, translation.y, 0.0),
                texture: img,
                ..default()
            },
        }
    }
}

pub fn move_bullet(mut query: Query<(&mut Transform, &BulletDirection), With<Bullet>>, time: Res<Time>,)
{
    for (mut sprite, direction) in query.iter_mut()
    {
        sprite.translation.x += direction.0.x * time.delta_seconds();
        sprite.translation.y += direction.0.y * time.delta_seconds();
    }
}