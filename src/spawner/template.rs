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

impl Templates {
    pub fn load() -> Self {

        // File::open returns a Result. Since the file may not exist, or you
        // may not have access to it, you need to handle the error.
        let file = File::open("resources/template.ron")
            .expect("Failed opening file");
        
        // from_reader calls Serde to open the file. If Serde cannot read the contents
        // of the file, the function will return an error, so we use expect to unwrap 
        // the returned option.
        from_reader(file)
            .expect("Unable to load templates")
    }
}