// pub mod gui;

use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{from_str};

pub struct SaveLoadPlugin;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitJson {
    pub name: String,
    pub sprite: String,
    pub max_hp: u32,
    pub hp: u32,
    pub dmg: u32,
    pub dmg_type: char,
    pub weaknesses: Vec<char>,
    pub resistances: Vec<char>,
}

fn save_game() {
    /* save player team to disk */
}

pub fn load_units(asset_path: &str) -> Vec<UnitJson> {
    let mut file = File::open(asset_path).expect("File und");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    from_str(&data).expect("Error fitting schema")
}
fn setup() {
    /* check for saved data */
}

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
