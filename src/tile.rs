use crate::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Walkable;

#[derive(Component)]
pub enum TileType {
    Floor,
    Wall,
}

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>, mut grid: ResMut<Grid>) {
    println!("Spawning tiles");
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            grid.nodes.insert(GridNode::new(x as i32, y as i32));
        }
    }

    for &GridNode(IVec2 { x, y }) in grid.nodes.clone().iter() {
        // let &GridNode { x, y } = node;
        let entity = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("floor.png"),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * TILE_SIZE as f32,
                        y as f32 * TILE_SIZE as f32,
                        0.0,
                    )),
                    ..Default::default()
                },
                Tile,
                Walkable,
                TileType::Floor,
            ))
            .id();

        grid.floors.hashmap.insert(GridNode::new(x, y), entity);
    }

    // commands.insert_resource(Grid { tiles: hashmap });
}

pub fn spawn_wall(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Vec2 { x, y } = cursor_world_position.position;
        // let node = GridNode::new(x as i32, y as i32);
        let node = GridNode::new(x as i32, y as i32);

        if !grid.nodes.contains(&node) {
            return;
        }

        let entity = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("wall.png"),
                    transform: Transform::from_xyz(x * TILE_SIZE as f32, y * TILE_SIZE as f32, 0.0),
                    ..Default::default()
                },
                TileType::Wall,
            ))
            .id();

        grid.walls.hashmap.insert(node, entity);
    }
}
