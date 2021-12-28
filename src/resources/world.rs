use bevy::{reflect::TypeUuid, prelude::Handle};

use super::map::TiledMap;

#[derive(TypeUuid)]
#[uuid = "183e015f-567d-48a9-9857-0d46901c6caa"]
pub struct TiledWorld {
    pub maps: Vec<(usize, usize, Handle<TiledMap>)>
}
