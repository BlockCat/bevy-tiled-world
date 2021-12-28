use bevy::{
    core::Name,
    prelude::{AddAsset, Bundle, IntoSystem, Plugin, Query, Transform, GlobalTransform},
};
use bevy_tilemap::{prelude::TilemapDefaultPlugins, Tilemap};
use components::Tileworld;
use resources::{TiledMap, TiledWorld};

pub mod components;
pub mod loader;
pub mod resources;

#[derive(Default)]
pub struct TiledWorldPlugin;

impl Plugin for TiledWorldPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_asset::<TiledWorld>()
            .add_plugins(TilemapDefaultPlugins)
            .add_asset::<TiledMap>()
            .add_event::<WorldReadyEvent>()
            .add_event::<MapReadyEvent>()
            .add_asset_loader(loader::TiledMapAssetLoader)
            .add_asset_loader(loader::WorldAssetLoader)
            .add_system(loader::process_loaded_world.system())
            .add_system(loader::process_loaded_map.system())
            .add_system(loader::spawn_worlds.system())
            .add_system(test.system())
            .add_system(loader::spawn_tilemaps.system());
    }
}

fn test(mut query: Query<&mut Tilemap>) {
    for mut tilemap in query.iter_mut() {
        for x in 0..=0 {
            for y in 0..=0 {
                tilemap.spawn_chunk((x, y)).unwrap();
            }
        }
        
    }
}


pub struct WorldReadyEvent;
pub struct MapReadyEvent;

#[derive(Bundle)]
pub struct WorldBundle {
    // Name of the world
    pub name: Name,
    pub world: Tileworld,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

