use std::fs::File;
use std::io::Write;

use bevy::tasks::IoTaskPool;

use crate::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<Position>()
            .register_type::<Walkable>()
            .register_type::<Floor>()
            .register_type::<Wall>()
            .register_type::<Object>()
            .register_type::<Item>()
            .add_systems(
                Update,
                save_scene_system.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(OnEnter(GamePlayState::Loading), load_scene_system);
    }
}

const SCENE_FILE_PATH: &str = "scenes/scene.scn.ron";

pub fn load_scene_system(world: &mut World) {
    println!("Loading Game");
    // (Temporary) Remove the tile_lookup components
    let query = world
        .query_filtered::<Entity, With<TileLookup>>()
        .get_single(world);

    if let Ok(entity) = query {
        world.despawn(entity);
    }

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
}

pub fn save_scene_system(world: &mut World) {
    if world.resource::<Input<KeyCode>>().just_pressed(KeyCode::P) {
        let mut query = world.query_filtered::<Entity, With<Tile>>();

        let mut builder = DynamicSceneBuilder::from_world(world);
        // builder.deny_all();
        builder.allow::<Tile>();
        builder.allow::<Position>();
        builder.allow::<Walkable>();
        builder.allow::<Floor>();
        builder.allow::<Wall>();
        builder.allow::<Object>();
        builder.allow::<Item>();
        builder.extract_entities(query.iter(world));
        let scene = builder.build();
        let type_registry = world.resource::<AppTypeRegistry>();
        let serialized_scene = match scene.serialize_ron(&type_registry) {
            Ok(serialized_scene) => serialized_scene,
            Err(error) => {
                error!("{}", error);
                return;
            }
        };

        info!("{}", serialized_scene);

        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/{SCENE_FILE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}
