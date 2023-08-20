use std::fs::File;
use std::io::Write;

use bevy::tasks::IoTaskPool;

use crate::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<TileType>()
            .register_type::<Position>()
            .register_type::<Walkable>()
            // .add_systems(OnEnter(GameState::Game), scene_setup)
            .add_systems(Startup, scene_setup)
            .add_systems(Update, save_scene_system.run_if(in_state(GameState::Game)));
        // .add_systems(Startup, load_scene_system)
        // .add_systems(Update, log_system);
    }
}

const SCENE_FILE_PATH: &str = "scenes/scene.scn.ron";

fn scene_setup(mut commands: Commands) {
    // commands.insert_resource(AppTypeRegistry::default());
}

pub fn save_scene_system(world: &mut World) {
    if world.resource::<Input<KeyCode>>().just_pressed(KeyCode::P) {
        let mut query = world.query_filtered::<Entity, With<Tile>>();

        let mut builder = DynamicSceneBuilder::from_world(world);
        // builder.deny_all();
        builder.allow::<Tile>();
        builder.allow::<TileType>();
        builder.allow::<Position>();
        builder.allow::<Walkable>();
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

// fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(DynamicSceneBundle {
//         scene: asset_server.load(SCENE_FILE_PATH),
//         ..default()
//     });
// }
//
// fn log_system(
//     query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
//     res: Option<Res<ResourceA>>,
// ) {
//     for (entity, component_a) in &query {
//         info!("  Entity({})", entity.index());
//         info!(
//             "    ComponentA: {{ x: {} y: {} }}\n",
//             component_a.x, component_a.y
//         );
//     }
//     if let Some(res) = res {
//         if res.is_added() {
//             info!("  New ResourceA: {{ score: {} }}\n", res.score);
//         }
//     }
// }
