use crate::prelude::*;
use bevy::render::render_resource::Texture;

// #[derive(Debug, Default)]
// pub struct Neighbors<T> {
//     pub east: Option<T>,
//     pub north_east: Option<T>,
//     pub north: Option<T>,
//     pub north_west: Option<T>,
//     pub west: Option<T>,
//     pub south_west: Option<T>,
//     pub south: Option<T>,
//     pub south_east: Option<T>,
// }

// This is just a tag component for now but it might have more use later
// like neighbors
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tile;

// This will be loaded when the scene is created
#[derive(Component)]
pub struct TileLookup {
    pub tiles: HashMap<Position, Entity>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InTileLookup;

impl TileLookup {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.tiles.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.tiles.values_mut()
    }
}

impl Default for TileLookup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Walkable;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
    Object,
    Item,
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub walkable: Walkable,
    pub tile_type: TileType,
    pub position: Position,
}

impl Default for TileBundle {
    fn default() -> Self {
        Self {
            tile: Tile,
            walkable: Walkable,
            tile_type: TileType::Floor,
            position: Position::new(0, 0),
        }
    }
}

// pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {

// fn countdown(
//     mut game_state: ResMut<NextState<GameState>>,
//     time: Res<Time>,
//     mut timer: ResMut<SplashTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::MainMenu);
//     }
// }
pub fn spawn_tiles(world: &mut World) {
    println!("Spawning tiles");
    // for x in 0..GRID_WIDTH {
    //     for y in 0..GRID_HEIGHT {
    //         let position = Position::new(x as i32, y as i32);
    //         let entity = commands
    //             .spawn((
    //                 SpriteBundle {
    //                     texture: asset_server.load("floor.png"),
    //                     transform: Transform::from_translation(Vec3::new(
    //                         x as f32 * TILE_SIZE as f32,
    //                         y as f32 * TILE_SIZE as f32,
    //                         0.0,
    //                     )),
    //                     ..Default::default()
    //                 },
    //                 TileBundle {
    //                     position,
    //                     ..default()
    //                 },
    //             ))
    //             .id();
    //
    //         tile_storage.tiles.insert(position, entity);
    //     }
    // }

    // let scene_bundle = DynamicSceneBundle {
    //     scene: asset_server.load("scenes/scene.scn.ron"),
    //     ..default()
    // };
    // let mut query = world.query::<Entity>();

    // for entity in world.query::<Entity>().iter(world) {
    //     println!("Entity Found");
    // }
    //
    let asset_server = world.resource::<AssetServer>();

    world.spawn(DynamicSceneBundle {
        scene: asset_server.load("scenes/scene.scn.ron"),
        ..default()
    });

    world
        .resource_mut::<NextState<GamePlayState>>()
        .set(GamePlayState::Playing);

    let tile_lookup = TileLookup::new();
    world.spawn(tile_lookup);

    // for entity in world.query::<(Entity, &SceneInstance)>().iter(world) {
    //     println!("Entity Found After Scene Loaded");
    // }

    // world.spawn(scene_bundle);
}

pub fn update_tile_lookup(
    mut commands: Commands,
    // query: Query<(Entity, &Tile)>,
    // q: Query<Entity>,
    mut tile_lookup_query: Query<&mut TileLookup>,
    tile_query: Query<(Entity, &Position), Without<InTileLookup>>,
) {
    // if tile_lookup_query.iter().count() > 0 {
    //     return;
    // }

    // if tile_query.iter().count() == 0 {
    //     return;
    // }

    // println!("Creating tile lookup");

    let mut tile_lookup = tile_lookup_query.single_mut();

    for (entity, position) in tile_query.iter() {
        // println!("Inserting tile at {:?}", position);
        tile_lookup.tiles.insert(*position, entity);
        commands.entity(entity).insert(InTileLookup);
        println!("Found Tile");
    }

    println!("Tile lookup count: {}", tile_lookup.tiles.len());
    // for entity in q.iter() {
    //     info!("Entity({})", entity.index());
    // }
}

pub fn update_sprites(
    mut commands: Commands,
    // TODO add second system to create the sprites, this one should just updates the texture
    query: Query<(Entity, &TileType, &Position), (With<Tile>, Without<Sprite>)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, tile_type, position) in query.iter() {
        // println!("Adding sprite to entity({})", entity.index());
        let texture = match tile_type {
            TileType::Floor => asset_server.load("floor.png"),
            TileType::Wall => asset_server.load("wall.png"),
            // TODO: change this
            _ => asset_server.load("floor.png"),
        };
        commands.entity(entity).insert(SpriteBundle {
            texture,
            transform: Transform::from_translation(position.to_world()),
            ..Default::default()
        });
    }
}

pub fn spawn_wall(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_world_position: Res<CursorWorldPosition>,
    tile_storage: Query<&TileLookup>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Vec2 { x, y } = cursor_world_position.position;

        if let Ok(tile_storage) = tile_storage.get_single() {
            let position = Position::new(x as i32, y as i32);
            if tile_storage.tiles.contains_key(&position) {
                commands
                    .entity(*tile_storage.tiles.get(&position).unwrap())
                    .remove::<Walkable>()
                    .remove::<TileType>()
                    // SpriteBundle
                    // TODO: Not remove the whole thing but just the texture
                    .remove::<Sprite>()
                    .remove::<Transform>()
                    .remove::<GlobalTransform>()
                    .remove::<Handle<Image>>()
                    .remove::<Visibility>()
                    .remove::<ComputedVisibility>()
                    // End SpriteBundle
                    .insert(TileType::Wall);
            } else {
                return;
            }
        }

        // let _entity = commands
        //     .spawn((
        //         SpriteBundle {
        //             texture: asset_server.load("wall.png"),
        //             transform: Transform::from_xyz(x * TILE_SIZE as f32, y * TILE_SIZE as f32, 0.0),
        //             ..Default::default()
        //         },
        //         TileType::Wall,
        //     ))
        //     .id();
    }
}
