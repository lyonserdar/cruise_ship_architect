use crate::prelude::*;
use crate::tiles::components::floor::Floor;
use crate::tiles::components::floor_sprite::FloorSprite;
use crate::tiles::components::tile::Tile;

pub fn update_floor_sprites(
    mut commands: Commands,
    query: Query<(Entity, &Floor), (With<Tile>, Changed<Floor>)>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Floor might eventually have a sprite texture information
    for (entity, _floor) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("floor.png"),
                    ..Default::default()
                },
                FloorSprite,
            ));
        });
    }
}
