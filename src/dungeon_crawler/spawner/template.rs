pub use crate::dungeon_crawler::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Clone, Debug, Deserialize)]
pub struct Template{
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>, 
    pub base_damage: Option<i32>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum EntityType {
    Item, Enemy
}

#[derive(Clone, Debug, Deserialize)]
pub struct Templates{
    pub entities:  Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed to open file");
        from_reader(file).expect("Unable to load file")
    }

    pub fn spawn_entities(&self, ecs: &mut World, rng: &mut RandomNumberGenerator, level: usize, spawn_points: &[Point]) {
        let mut availible_entities = Vec::new();
        self.entities.iter().filter(|e| e.levels.contains(&level))
        .for_each(|t| {
            for _ in 0 .. t.frequency {
                availible_entities.push(t);
            }
        });
        let mut cb = legion::systems::CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&availible_entities) {
                self.spawn_entity(pt, entity, &mut cb)
            }
        });
        cb.flush(ecs);
    }

    pub fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut legion::systems::CommandBuffer) {
        let entity = commands.push((
            pt.clone(),
            Render{
                color: ColorPair::new( WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone())
        ));
        match template.entity_type {
            EntityType::Item => {
                commands.add_component(entity, Item{})
            },
            EntityType::Enemy => {
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer{});
                commands.add_component(entity, Enemy{});
                commands.add_component(entity, Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap()
                });
            }
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(name, count)| {
                match name.as_str() {
                    "Healing" => {
                        commands.add_component(entity, ProvidesHealing {amount: *count});
                    },
                    "Magic map" => {
                        commands.add_component(entity, ProvidesDungeonMap{});
                    },
                    _ => {println!("Some Unknown Effect !: {}", name);}
                }
            });
        }
    }
}
