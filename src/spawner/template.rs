use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type : EntityType,
    pub levels : HashSet<usize>,
    pub frequency : i32,
    pub name : String,
    pub glyph : char,
    pub provides : Option<Vec<(String, i32)>>,
    pub hp : Option<i32>
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy, Item
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities : Vec<Template>,
}
