use crate::{
    components::{SpawnMap, SpawnWorld, Tileworld},
    resources::{TiledMap, TiledWorld},
    MapReadyEvent, WorldBundle, WorldReadyEvent,
};
use bevy::{
    core::Name,
    math::Vec2,
    prelude::{
        AssetEvent, Assets, BuildChildren, Color, Commands, Entity, EventReader, EventWriter,
        Handle, Query, Res, ResMut, Texture, Visible,
    },
    sprite::{Rect, TextureAtlas, TextureAtlasBuilder, TextureAtlasBuilderError},
};
use bevy_tilemap::{
    prelude::{TilemapBuilder, TilemapBundle},
    tilemap::TilemapError,
    Tile, Tilemap, TilemapLayer,
};

mod map;
mod world;

pub use map::TiledMapAssetLoader;
pub use world::WorldAssetLoader;

/// Handle events: do loading of tiledmap resource.
/// Based on events
///
pub fn process_loaded_map(
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    mut ready_event: EventWriter<MapReadyEvent>,
) {
    // Load textureatlas
    // Change active/inactive tilemaps
    //
}

pub fn process_loaded_world(
    mut map_events: EventReader<AssetEvent<TiledWorld>>,
    mut ready_event: EventWriter<WorldReadyEvent>,
) {
    for i in map_events.iter() {
        println!("I: {:?}", i);
    }
    // let mut changed_maps = HashSet::<Handle<TiledWorld>>::default();

    // for event in map_events.iter() {
    //     match event {
    //         AssetEvent::Created { handle } => {
    //             changed_maps.insert(handle.clone());
    //         }
    //         AssetEvent::Modified { handle } => {
    //             changed_maps.insert(handle.clone());
    //         }
    //         AssetEvent::Removed { handle } => {
    //             // if mesh was modified and removed in the same update, ignore the modification
    //             // events are ordered so future modification events are ok
    //             changed_maps.remove(handle);
    //         }
    //     }
    // }

    // ready_event.send(WorldReadyEvent);
}

pub fn spawn_worlds(
    mut commands: Commands,
    query: Query<(Entity, &SpawnWorld)>,
    world_res: Res<Assets<TiledWorld>>,
) {
    for (entity, spawn) in query.iter() {
        println!("Found spawn world request");

        if let Some(world) = world_res.get(spawn.world.clone_weak()) {
            commands
                .entity(entity)
                .remove::<SpawnWorld>()
                .insert_bundle(WorldBundle {
                    name: Name::from(spawn.name.as_str()),
                    world: Tileworld {
                        world: spawn.world.clone(),
                    },
                    transform: Default::default(),
                    global_transform: Default::default(),
                })
                .with_children(|parent| {
                    for (x, y, map) in &world.maps {
                        parent.spawn().insert(SpawnMap {
                            name: Some(format!("{} [{}, {}]", spawn.name, x, y)),
                            map: map.clone_weak(),
                            x: *x,
                            y: *y,
                        });
                    }
                });
            println!("Spawn request has loaded world");
        }
    }
}

pub fn spawn_tilemaps(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &SpawnMap)>,
    map_res: Res<Assets<TiledMap>>,
) {
    // println!("L: {}", textures.len());

    for (entity, spawn) in query.iter() {
        println!("Found spawn tilemap request");
        if let Some(map) = map_res.get(spawn.map.clone_weak()) {
            println!("Spawn request has loaded tilemap");
            let atlas = match create_texture_atlas(map, &mut textures) {
                Ok(atlas) => atlas,
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            };

            let atlas = atlases.add(atlas);
            let tilemap = create_tilemap(map, atlas).unwrap();

            commands
                .entity(entity)
                .remove::<SpawnMap>()
                .insert_bundle(TilemapBundle {
                    tilemap,
                    visible: Visible {
                        is_visible: true,
                        is_transparent: true,
                    },
                    transform: Default::default(),
                    global_transform: Default::default(),
                });
        }
    }
}

fn create_tilemap(
    map: &TiledMap,
    texture_atlas: Handle<TextureAtlas>,
) -> Result<Tilemap, TilemapError> {
    let mut tilemap = TilemapBuilder::default()
        .auto_chunk()
        .auto_spawn(6, 6)
        .texture_atlas(texture_atlas)
        .texture_dimensions(32, 32);

    let mut e = Vec::new();

    for layer in &map.map.layers {
        match &layer.tiles {
            tiled::layers::LayerData::Finite(l) => {
                for (y, l) in l.into_iter().enumerate() {
                    for (x, tile) in l.into_iter().enumerate() {
                        if tile.gid > 0 {
                            e.push(Tile {
                                point: (x as i32, -(y as i32)),
                                sprite_order: layer.layer_index as usize,
                                sprite_index: tile.gid as usize - 1, //tile.gid as usize,
                                tint: Color::WHITE,
                            });
                        }
                    }
                }
            }
            tiled::layers::LayerData::Infinite(_) => todo!(),
        }

        tilemap = tilemap.add_layer(TilemapLayer::default(), layer.layer_index as usize);
    }

    let mut tilemap = tilemap.finish()?;
    tilemap.insert_tiles(e)?;

    Ok(tilemap)
}

fn create_texture_atlas(
    map: &TiledMap,
    textures: &mut Assets<Texture>,
) -> Result<TextureAtlas, LoaderError> {
    let mut builder = TextureAtlasBuilder::default();

    for (handle, _) in &map.tilesets {
        let texture = textures
            .get(handle.clone_weak())
            .ok_or(LoaderError::TextureNotLoaded)?;

        builder.add_texture(handle.clone_weak(), texture);
    }

    // hack to combine
    let a = builder
        .finish(textures)
        .map_err(|e| LoaderError::AtlasBuildError(e))?;

    // Ok(a)

    let image_recs = a.textures.clone();

    let mut new_rects = Vec::new();

    for (image_rec, (_, tileset)) in image_recs.iter().zip(&map.tilesets) {
        let cols = image_rec.width() as u32 / tileset.tile_width;
        let rows = image_rec.height() as u32 / tileset.tile_height;

        let min = image_rec.min;

        for row in 0..rows {
            for col in 0..cols {
                let offset = Vec2::from((
                    (col * tileset.tile_width) as f32,
                    (row * tileset.tile_height) as f32,
                ));
                let min = min + offset;
                let max = min + Vec2::from((tileset.tile_width as f32, tileset.tile_height as f32));
                new_rects.push(Rect { min, max });
            }
        }
    }

    Ok(TextureAtlas {
        texture: a.texture,
        size: a.size,
        textures: new_rects,
        texture_handles: Default::default(),
    })
}

#[derive(Debug)]
enum LoaderError {
    TextureNotLoaded,
    AtlasBuildError(TextureAtlasBuilderError),
}
