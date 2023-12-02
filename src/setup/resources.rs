use bevy::prelude::*;
use bevy::gltf::Gltf;

#[derive(Resource)]
pub struct MyAssetPack(Handle<Gltf>);