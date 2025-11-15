use anyhow::{Ok, Result};

use dofus_opti_core::file::read_gears;
use dofus_opti_core::model::Gear as CoreGear;
use dofus_opti_core::model::{ALL_GEAR_TYPES, Id};

use dofus_opti_dofus_build::gear_selector;
use dofus_opti_dofus_build::model::*;
use dofus_opti_dofus_build::parser::parse_gear;
use dofus_opti_dofus_build::scorer::default_score;

use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Dofus Build CLI");

    let build_requirements = BuildRequirements {
        requirements: vec![
            Requirement {
                id: RequirementId::Strength,
                desired_value: 800,
            },
            Requirement {
                id: RequirementId::Vitality,
                desired_value: 3000,
            },
        ],
    };

    let gear_ids_to_ignore = HashSet::from([Id::from("gore_master_ring_(gms_only)")]);

    let effect_scorer = |effects: &Effects| default_score(&build_requirements, effects);
    let gear_scorer = |gear: &Gear| effect_scorer(&gear.effects);

    let gears: Vec<Gear> = import_all_gears()?;

    let mut catalog = GearCatalog::new(gears);
    catalog.filter(gear_selector::ignore_ids(gear_ids_to_ignore));
    catalog.filter(gear_selector::select_top(10, gear_scorer));

    let mut build = Build::empty();

    let mut build_possible: u128 = 1;

    for gear_slot_type in ALL_GEAR_SLOT_TYPES {
        let size = catalog.get_gears(*gear_slot_type).len();
        println!("Number of {gear_slot_type} considered: {size}");
        build_possible = build_possible * size as u128;
    }

    println!("Number of possible build {build_possible}");

    let mut full_builder_counter: i64 = 0;

    for amulet in catalog.get_gears(GearSlotType::Amulet) {
        if let Err(e) = build.set_gear(amulet, &GearSlot::Amulet) {
            eprintln!("❌ Skipping amulet {}: {}", amulet.name, e);
            continue;
        }
        for belt in catalog.get_gears(GearSlotType::Belt) {
            if let Err(e) = build.set_gear(belt, &GearSlot::Belt) {
                eprintln!("❌ Skipping belt {}: {}", belt.name, e);
                continue;
            }
            for boot in catalog.get_gears(GearSlotType::Boots) {
                if let Err(e) = build.set_gear(boot, &GearSlot::Boots) {
                    eprintln!("❌ Skipping boots {}: {}", boot.name, e);
                    continue;
                }
                for cloack in catalog.get_gears(GearSlotType::Cloak) {
                    if let Err(e) = build.set_gear(cloack, &GearSlot::Cloak) {
                        eprintln!("❌ Skipping cloack {}: {}", cloack.name, e);
                        continue;
                    }
                    for hat in catalog.get_gears(GearSlotType::Hat) {
                        if let Err(e) = build.set_gear(hat, &GearSlot::Hat) {
                            eprintln!("❌ Skipping hat {}: {}", hat.name, e);
                            continue;
                        }
                        for ring_1 in catalog.get_gears(GearSlotType::Ring) {
                            if let Err(e) = build.set_gear(ring_1, &GearSlot::Ring1) {
                                eprintln!("❌ Skipping ring 1 {}: {}", ring_1.name, e);
                                continue;
                            }
                            for ring_2 in catalog.get_gears(GearSlotType::Ring) {
                                if let Err(e) = build.set_gear(ring_2, &GearSlot::Ring2) {
                                    eprintln!("❌ Skipping ring 2 {}: {}", ring_2.name, e);
                                    continue;
                                }
                                for shield in catalog.get_gears(GearSlotType::Shield) {
                                    if let Err(e) = build.set_gear(shield, &GearSlot::Shield) {
                                        eprintln!("❌ Skipping shield {}: {}", shield.name, e);
                                        continue;
                                    }
                                    for weapon in catalog.get_gears(GearSlotType::Weapon) {
                                        if let Err(e) = build.set_gear(weapon, &GearSlot::Weapon) {
                                            eprintln!("❌ Skipping weapon {}: {}", weapon.name, e);
                                            continue;
                                        } else {
                                            full_builder_counter += 1;
                                            if full_builder_counter % 10_000_000 == 0 {
                                                println!(
                                                    "Iterated over {} builds",
                                                    full_builder_counter
                                                );
                                            }
                                            if build.satisfy_requirements(&build_requirements) {
                                                build.print_short_build();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn import_all_gears() -> Result<Vec<Gear>> {
    let mut all_core_gears: Vec<CoreGear> = Vec::new();

    for gear_type in ALL_GEAR_TYPES {
        let mut gears = read_gears("core/data", gear_type)?;
        all_core_gears.append(&mut gears);
    }

    let all_gears = all_core_gears.into_iter().map(parse_gear).collect();

    Ok(all_gears)
}
