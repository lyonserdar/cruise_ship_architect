use crate::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ComponentA>()
            .register_type::<ComponentB>()
            .register_type::<ResourceA>()
            .add_systems(Startup, load_scene_system)
            .add_systems(Update, log_system);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        Self {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ResourceA {
    pub score: u32,
}

const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}

fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}
