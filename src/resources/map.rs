use bevy::{prelude::*, reflect::TypeUuid};
use tiled::{map::Map, tileset::Tileset};

#[derive(TypeUuid)]
#[uuid = "ae929dbf-ee9e-4d0d-9f49-a34dd63ab155"]
pub struct TiledMap {
    pub map: Map,
    pub tilesets: Vec<(Handle<Texture>, Tileset)>,
}
