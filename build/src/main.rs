use anyhow::{Ok, Result};

use clap::Parser;

use dofus_opti_core::file::read_gears;
use dofus_opti_core::model::ALL_GEAR_TYPES;
use dofus_opti_core::model::Gear as CoreGear;

use dofus_opti_dofus_build::gear_selector;
use dofus_opti_dofus_build::model::*;
use dofus_opti_dofus_build::parser::parse_gear;
use dofus_opti_dofus_build::scorer::default_score;

use num_format::{Locale, ToFormattedString};

use std::collections::HashSet;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Select the language of the output
    #[arg(long = "language", default_value = "English")]
    language: Language,

    /// Define one or more build requirements (e.g. "Vitality >= 3000")
    #[arg(short, long = "requirement", num_args(1..), action = clap::ArgAction::Append)]
    requirements: Vec<Requirement>,

    #[arg(short, long = "ignore-gear", num_args(1..), action = clap::ArgAction::Append)]
    ignore_gears: Vec<String>,

    /// prepare the gear catalog but don't run the search
    #[arg(long = "dry-run")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Dofus Build CLI");

    let args = Args::parse();

    let build_requirements = BuildRequirements {
        requirements: args.requirements,
    };

    println!("{:?}", build_requirements);

    let gear_ids_to_ignore: HashSet<String> = args.ignore_gears.into_iter().collect();

    let effect_scorer = |effects: &Effects| default_score(&build_requirements, effects);
    let gear_scorer = |gear: &Gear| effect_scorer(&gear.effects);

    let gears: Vec<Gear> = import_all_gears()?;

    let mut catalog = GearCatalog::new(gears);
    println!("Initial {}", catalog.summarize());

    if !gear_ids_to_ignore.is_empty() {
        catalog.filter(gear_selector::ignore_ids(gear_ids_to_ignore, args.language));
    }

    catalog.filter(gear_selector::select_top(gear_scorer, 30));
    catalog.filter(gear_selector::select_by_stdev(gear_scorer, 0.5));

    println!("After filtering {}", catalog.summarize());

    if args.dry_run {
        return Ok(());
    }

    let mut build = Build::new();

    let mut build_created: i64 = 0;
    let report_every = 1_000_000;
    let start = Instant::now();
    let mut last_report = start;

    for amulet in catalog.get_gears(GearSlotType::Amulet) {
        if let Err(e) = build.set_gear(GearSlot::Amulet, amulet) {
            eprintln!("❌ Skipping amulet {}: {}", amulet.name.localized(args.language), e);
            continue;
        }
        for belt in catalog.get_gears(GearSlotType::Belt) {
            if let Err(e) = build.set_gear(GearSlot::Belt, belt) {
                eprintln!("❌ Skipping belt {}: {}", belt.name.localized(args.language), e);
                continue;
            }
            for boot in catalog.get_gears(GearSlotType::Boots) {
                if let Err(e) = build.set_gear(GearSlot::Boots, boot) {
                    eprintln!("❌ Skipping boots {}: {}", boot.name.localized(args.language), e);
                    continue;
                }
                for cloack in catalog.get_gears(GearSlotType::Cloak) {
                    if let Err(e) = build.set_gear(GearSlot::Cloak, cloack) {
                        eprintln!(
                            "❌ Skipping cloack {}: {}",
                            cloack.name.localized(args.language),
                            e
                        );
                        continue;
                    }
                    for hat in catalog.get_gears(GearSlotType::Hat) {
                        if let Err(e) = build.set_gear(GearSlot::Hat, hat) {
                            eprintln!(
                                "❌ Skipping hat {}: {}",
                                hat.name.localized(args.language),
                                e
                            );
                            continue;
                        }
                        for ring_1 in catalog.get_gears(GearSlotType::Ring) {
                            if let Err(e) = build.set_gear(GearSlot::Ring1, ring_1) {
                                eprintln!(
                                    "❌ Skipping ring 1 {}: {}",
                                    ring_1.name.localized(args.language),
                                    e
                                );
                                continue;
                            }
                            for ring_2 in catalog.get_gears(GearSlotType::Ring) {
                                if let Err(e) = build.set_gear(GearSlot::Ring2, ring_2) {
                                    eprintln!(
                                        "❌ Skipping ring 2 {}: {}",
                                        ring_2.name.localized(args.language),
                                        e
                                    );
                                    continue;
                                }
                                for shield in catalog.get_gears(GearSlotType::Shield) {
                                    if let Err(e) = build.set_gear(GearSlot::Shield, shield) {
                                        eprintln!(
                                            "❌ Skipping shield {}: {}",
                                            shield.name.localized(args.language),
                                            e
                                        );
                                        continue;
                                    }
                                    for weapon in catalog.get_gears(GearSlotType::Weapon) {
                                        if let Err(e) = build.set_gear(GearSlot::Weapon, weapon) {
                                            eprintln!(
                                                "❌ Skipping weapon {}: {}",
                                                weapon.name.localized(args.language),
                                                e
                                            );
                                            continue;
                                        } else {
                                            build_created += 1;
                                            if build_created % report_every == 0 {
                                                let now = Instant::now();
                                                let delta_sec =
                                                    now.duration_since(last_report).as_secs_f64();
                                                let build_rate_sec =
                                                    report_every as f64 / delta_sec;
                                                println!(
                                                    "Iterated over {} builds with a build rate of {} build/sec",
                                                    build_created.to_formatted_string(&Locale::en),
                                                    (build_rate_sec as u64)
                                                        .to_formatted_string(&Locale::en)
                                                );

                                                last_report = now;
                                            }
                                            if build.satisfy_requirements(&build_requirements) {
                                                println!("{}", build.summary(args.language));
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
