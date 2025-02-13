use specs::{prelude::*, rayon::string};
use specs_derive::*;
use rltk::RGB;

/// Non-NPC Player component
#[derive(Component, Debug)]
pub struct Player {}

/// Defines an entities location in space
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Entities can be rendered with a glyph
/// foreground and background
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

/// Viewshed means "what can I see from here?"
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub dirty: bool,
    pub range: i32,
}

/// NPC Mob
#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

/// Indicates that this component can block other entities
/// Used by the map_indexing_system
#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// Indicates that a component can have
/// melee intent.
#[derive(Component, Debug)]
pub struct WantsToMelee {
    pub target: Entity
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        // If the entity already has a SufferDamage component
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            // Add a new SufferDamage component
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Potion {
    pub heal_amount: i32
}

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity
}