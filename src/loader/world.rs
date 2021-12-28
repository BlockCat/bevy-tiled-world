use crate::resources::TiledWorld;
use bevy::asset::{AssetLoader, AssetPath, LoadedAsset};
use path_dedot::ParseDot;
use serde::Deserialize;
use std::io::BufReader;

pub struct WorldAssetLoader;

impl AssetLoader for WorldAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        println!("Loading world: {:?}", load_context.path());
        Box::pin(async move {
            let root = load_context.path().parent().unwrap();

            let world: JsonTiledWorld = serde_json::from_reader(BufReader::new(bytes))?;

            let dependencies = world
                .maps
                .iter()
                .map(|m| {
                    let path = root.join(&m.file_name).parse_dot().unwrap().to_path_buf();

                    println!("Tileset: {:?}", path);

                    AssetPath::new(path, None)
                })
                .collect::<Vec<_>>();

            let maps = dependencies
                .iter()
                .map(|x| load_context.get_handle(x.clone()))
                .zip(&world.maps)
                .map(|(handle, map)| (map.x, map.y, handle))
                .collect();

            let loaded_asset = LoadedAsset::new(TiledWorld { maps });

            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));

            println!("Loaded world: {:?}", load_context.path());

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["world"]
    }
}

#[derive(Deserialize)]
struct JsonTiledWorld {
    maps: Vec<JsonTiledMap>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonTiledMap {
    file_name: String,
    x: usize,
    y: usize,
}
