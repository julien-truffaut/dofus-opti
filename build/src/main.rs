use anyhow::{Ok, Result};

use dofus_opti_core::model::{ALL_GEAR_TYPES, CharacteristicRange, CharacteristicType};
use dofus_opti_core::model::Gear as CoreGear;
use dofus_opti_core::file::read_gears;

use dofus_opti_dofus_build::model::*;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {

    println!("Dofus Build CLI");

    let all_gears= import_all_gears()?;
    let filtered_gears = all_gears.into_iter().filter(|g| g.level > 198).collect();
    let all_gears_by_slot_type = group_gears_by_slot_type(filtered_gears);

    let mut build = Build::empty();

    let amulets = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Amulet);
    let belts   = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Belt);
    let boots   = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Boots);
    let cloacks = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Cloak);
    let hats    = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Hat);
    let rings   = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Ring);
    let shields = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Shield);
    let weapons = get_gears_for_slot(&all_gears_by_slot_type, GearSlotType::Weapon);

    let mut build_possible: u128 = 1;

    for gear_slot_type in ALL_GEAR_SLOT_TYPES {
        let size = all_gears_by_slot_type.get(gear_slot_type).map(|gears| gears.len()).unwrap_or(0);
        println!("Number of {gear_slot_type} considered: {size}");
        build_possible = build_possible * size as u128;
    }

    println!("Number of possible build {build_possible}");

    let mut full_builder_counter = 0;

    for amulet in amulets {
        if let Err(e) = build.set_gear(amulet, &GearSlot::Amulet) {
            eprintln!("❌ Skipping amulet {}: {}", amulet.name, e);
            continue;
        }
        for belt in belts {
            if let Err(e) = build.set_gear(belt, &GearSlot::Belt) {
                eprintln!("❌ Skipping belt {}: {}", belt.name, e);
                continue;
            }
            for boot in boots {
                if let Err(e) = build.set_gear(boot, &GearSlot::Boots) {
                    eprintln!("❌ Skipping boots {}: {}", boot.name, e);
                    continue;
                }
                for cloack in cloacks {
                    if let Err(e) = build.set_gear(cloack, &GearSlot::Cloak) {
                        eprintln!("❌ Skipping cloack {}: {}", cloack.name, e);
                        continue;
                    }
                    for hat in hats {
                        if let Err(e) = build.set_gear(hat, &GearSlot::Hat) {
                            eprintln!("❌ Skipping hat {}: {}", hat.name, e);
                            continue;
                        }
                        for ring_1 in rings {
                            if let Err(e) = build.set_gear(ring_1, &GearSlot::Ring1) {
                                eprintln!("❌ Skipping ring 1 {}: {}", ring_1.name, e);
                                continue;
                            }
                            for ring_2 in rings {
                                if let Err(e) = build.set_gear(ring_2, &GearSlot::Ring2) {
                                    eprintln!("❌ Skipping ring 2 {}: {}", ring_2.name, e);
                                    continue;
                                }
                                for shield in shields {
                                    if let Err(e) = build.set_gear(shield, &GearSlot::Shield) {
                                        eprintln!("❌ Skipping shield {}: {}", shield.name, e);
                                        continue;
                                    }
                                    for weapon in weapons {
                                        if let Err(e) = build.set_gear(weapon, &GearSlot::Weapon) {
                                            eprintln!("❌ Skipping weapon {}: {}", weapon.name, e);
                                            continue;
                                        } else {
                                            full_builder_counter += 1;
                                            if full_builder_counter % 10_000_000 == 0 {
                                                println!("Completed {full_builder_counter} builds");
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



fn get_gears_for_slot<'a>(
    map: &'a HashMap<GearSlotType, Vec<Gear>>,
    slot: GearSlotType,
) -> &'a [Gear] {
    map.get(&slot)
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

fn group_gears_by_slot_type(gears: Vec<Gear>) -> HashMap<GearSlotType, Vec<Gear>> {
    let mut map: HashMap<GearSlotType, Vec<Gear>> = HashMap::new();

    for gear in gears {
        let slot_type = GearSlotType::from(&gear.gear_type);
        map.entry(slot_type)
            .or_insert_with(Vec::new)
            .push(gear);
        
    }

    map
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

fn parse_gear(gear: CoreGear) -> Gear {
    Gear { 
        id: gear.id, 
        name: gear.name,
        gear_type: gear.gear_type, 
        level: gear.level, 
        effects: parse_effects(gear.characteristics) 
    }
}

fn parse_effects(characteristics: Vec<CharacteristicRange>) -> Effects {
    let mut effects = Effects::empty();

    characteristics.iter().for_each(|characteristic_range| match characteristic_range.kind {
        CharacteristicType::AbilityPoint             => effects.ability_point = Some(characteristic_range.max),
        CharacteristicType::AbilityPointParry        => effects.ability_point_parry = Some(characteristic_range.max),
        CharacteristicType::AbilityPointReduction    => effects.ability_point_reduction = Some(characteristic_range.max),
        CharacteristicType::Agility                  => effects.agility = Some(characteristic_range.max),
        CharacteristicType::AirDamage                => effects.air_damage = Some(characteristic_range.max),
        CharacteristicType::AirResistance            => effects.air_resistance = Some(characteristic_range.max),
        CharacteristicType::AirResistancePercent     => effects.air_resistance_percent = Some(characteristic_range.max),
        CharacteristicType::Chance                   => effects.chance = Some(characteristic_range.max),
        CharacteristicType::Critical                 => effects.critical = Some(characteristic_range.max),
        CharacteristicType::CriticalDamage           => effects.critical_damage = Some(characteristic_range.max),
        CharacteristicType::CriticalResistance       => effects.critical_resistance = Some(characteristic_range.max),
        CharacteristicType::Damage                   => effects.damage = Some(characteristic_range.max),
        CharacteristicType::Dodge                    => effects.dodge = Some(characteristic_range.max),
        CharacteristicType::EarthDamage              => effects.earth_damage = Some(characteristic_range.max),
        CharacteristicType::EarthResistance          => effects.earth_resistance = Some(characteristic_range.max),
        CharacteristicType::EarthResistancePercent   => effects.earth_resistance_percent = Some(characteristic_range.max),
        CharacteristicType::FireDamage               => effects.fire_damage = Some(characteristic_range.max),
        CharacteristicType::FireResistance           => effects.fire_resistance = Some(characteristic_range.max),
        CharacteristicType::FireResistancePercent    => effects.fire_resistance_percent = Some(characteristic_range.max),
        CharacteristicType::Heals                    => effects.heals = Some(characteristic_range.max),
        CharacteristicType::Initiative               => effects.initiative = Some(characteristic_range.max),
        CharacteristicType::Intelligence             => effects.intelligence = Some(characteristic_range.max),
        CharacteristicType::Lock                     => effects.lock = Some(characteristic_range.max),
        CharacteristicType::MeleeDamage              => effects.melee_damage = Some(characteristic_range.max),
        CharacteristicType::MeleeResistance          => effects.melee_resistance = Some(characteristic_range.max),
        CharacteristicType::MovementPoint            => effects.movement_point = Some(characteristic_range.max),
        CharacteristicType::MovementPointParry       => effects.movement_point_parry = Some(characteristic_range.max),
        CharacteristicType::MovementPointReduction   => effects.movement_point_reduction = Some(characteristic_range.max),
        CharacteristicType::NeutralDamage            => effects.neutral_damage = Some(characteristic_range.max),
        CharacteristicType::NeutralResistance        => effects.neutral_resistance = Some(characteristic_range.max),
        CharacteristicType::NeutralResistancePercent => effects.neutral_resistance_percent = Some(characteristic_range.max),
        CharacteristicType::Pods                     => effects.pods = Some(characteristic_range.max),
        CharacteristicType::Power                    => effects.power = Some(characteristic_range.max),
        CharacteristicType::Prospecting              => effects.prospecting = Some(characteristic_range.max),
        CharacteristicType::PushBackDamage           => effects.push_back_damage = Some(characteristic_range.max),
        CharacteristicType::PushBackResistance       => effects.push_back_resistance = Some(characteristic_range.max),
        CharacteristicType::Range                    => effects.range = Some(characteristic_range.max),
        CharacteristicType::RangeDamage              => effects.range_damage = Some(characteristic_range.max),
        CharacteristicType::RangeResistance          => effects.range_resistance = Some(characteristic_range.max),
        CharacteristicType::ReflectedDamage          => effects.reflected_damage = Some(characteristic_range.max),
        CharacteristicType::SpellDamage              => effects.spell_damage = Some(characteristic_range.max),
        CharacteristicType::Strength                 => effects.strength = Some(characteristic_range.max),
        CharacteristicType::Summon                   => effects.summon = Some(characteristic_range.max),
        CharacteristicType::TrapDamage               => effects.trap_damage = Some(characteristic_range.max),
        CharacteristicType::TrapPower                => effects.trap_power = Some(characteristic_range.max),
        CharacteristicType::Vitality                 => effects.vitality = Some(characteristic_range.max),
        CharacteristicType::WaterDamage              => effects.water_damage = Some(characteristic_range.max),
        CharacteristicType::WaterResistance          => effects.water_resistance = Some(characteristic_range.max),
        CharacteristicType::WaterResistancePercent   => effects.water_resistance_percent = Some(characteristic_range.max),
        CharacteristicType::WeaponDamage             => effects.weapon_damage = Some(characteristic_range.max),
        CharacteristicType::Wisdom                   => effects.wisdom = Some(characteristic_range.max),
    });

    effects
}