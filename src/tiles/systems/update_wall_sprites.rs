use crate::tiles::components::tile::Tile;
use crate::tiles::components::wall::Wall;
use crate::tiles::components::wall_sprite::WallSprite;
use bevy::prelude::*;

pub fn update_wall_sprites(
    mut commands: Commands,
    query: Query<(Entity, &Wall), (With<Tile>, Changed<Wall>)>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Wall might eventually have a sprite texture information
    for (entity, _wall) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/wall.png"),
                    ..Default::default()
                },
                WallSprite,
            ));
        });
    }
}
