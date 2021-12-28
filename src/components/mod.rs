use bevy::prelude::Handle;
use crate::resources::{TiledWorld, TiledMap};

pub struct Tileworld {
    pub world: Handle<TiledWorld>
}

pub struct SpawnWorld {
    pub name: String,
    pub world: Handle<TiledWorld>
}

pub struct SpawnMap {
    pub name: Option<String>,
    pub map: Handle<TiledMap>,
    pub x: usize,
    pub y: usize,
}