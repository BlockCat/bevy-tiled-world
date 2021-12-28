use crate::resources::TiledMap;
use bevy::asset::{AssetLoader, AssetPath, LoadedAsset};
use std::io::{BufReader, Cursor, Read};
use tiled::error::TiledError;

pub struct TiledMapAssetLoader;

impl AssetLoader for TiledMapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path = load_context.path().parent().unwrap();

            let map = tiled::parse_with_factory(BufReader::new(bytes), |first_gid, source| {
                let tileset_path = load_context.path().with_file_name(source);
                let bytes =
                    futures::executor::block_on(load_context.read_asset_bytes(tileset_path))
                        .map_err(|e| TiledError::Other(e.to_string()))?;

                tiled::parse_tileset(Cursor::new(bytes), first_gid)
            })?;

            let mut asset_paths = Vec::with_capacity(map.tilesets.len());
            let mut asset_handles = Vec::with_capacity(map.tilesets.len());

            for tileset in &map.tilesets {
                for image in &tileset.images {
                    let path = path.join(&image.source);
                    let asset_path = AssetPath::new(path, None);

                    let texture_handle = load_context.get_handle(asset_path.clone());

                    asset_handles.push((texture_handle, tileset.clone()));
                    asset_paths.push(asset_path);
                }
            }

            let loaded_asset = LoadedAsset::new(TiledMap {
                map,
                tilesets: asset_handles,
            });

            load_context.set_default_asset(loaded_asset.with_dependencies(asset_paths));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}
