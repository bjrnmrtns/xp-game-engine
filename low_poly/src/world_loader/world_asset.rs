use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "9fcc8bee-3547-11eb-adc1-0242ac120002"]
pub struct WorldAsset {
    pub objects: Vec<(i32, i32, i32)>,
}
