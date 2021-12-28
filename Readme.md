# Bevy and tiled world

Use at your own risk, clone, extend.

```rust
use bevy::{asset::AssetPath, prelude::*, DefaultPlugins};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tiled_world::{components::SpawnWorld, resources::TiledWorld, TiledWorldPlugin};

fn main() {
    println!("Starting game");
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TiledWorldPlugin)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_world.system()))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("Setup");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let world: Handle<TiledWorld> = asset_server.load("tiled/world/main.world");

    commands.spawn().insert(SpawnWorld {
        name: String::from("test"),
        world,
    });
}
```