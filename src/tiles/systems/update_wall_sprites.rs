use crate::prelude::*;
use crate::tiles::components::tile::Tile;
use crate::tiles::components::wall::Wall;

pub fn update_wall_sprites(
    mut commands: Commands,
    query: Query<(Entity, &Wall), (With<Tile>, Changed<Wall>)>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Wall might eventually have a sprite texture information
    for (entity, wall) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("wall.png"),
                ..Default::default()
            });
        });
    }
}
