
pub use crate::dungeon_crawler::prelude::*;
#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]

pub fn use_item (ecs: &mut SubWorld,
                commands: &mut CommandBuffer,
                #[resource] map: &mut CrawlerMap) {

    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate_item)|{
        let item = ecs.entry_ref(activate_item.item);
        if let Ok(item) = item {
            println!("Try to use Item");
            if let Ok(healing) = item.get_component::<ProvidesHealing>(){
                println!("Try to use health poitiion");
                healing_to_apply.push((activate_item.used_by, healing.amount));
            }
            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>(){
                println!("Try to apply dungeon map");
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }
        }
        commands.remove(activate_item.item);
        commands.remove(*entity);
    });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) { 
            if let Ok(health) = target.get_component_mut::<Health>(){
                health.current = i32::min(health.max, health.current + heal.1)
            }
        }        
    }

}
