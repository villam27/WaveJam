use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub fn print_walls(walls: Query<&Wall>)
{
    let mut i = 0;

    walls.for_each(|_| i += 1);
    println!("Wall count: {i}");
}
