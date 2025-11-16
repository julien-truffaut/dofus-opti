use dofus_opti_core::model::Id;

use crate::model::Gear;
use std::cmp::Reverse;
use std::collections::HashSet;

pub fn ignore_ids(ids_ignore: HashSet<Id>) -> impl FnMut(&mut Vec<Gear>) {
    move |gears| {
        gears.retain(|gear| !ids_ignore.contains(&gear.id));
    }
}

pub fn select_top<F>(scorer: F, size: usize) -> impl FnMut(&mut Vec<Gear>)
where
    F: Fn(&Gear) -> i32,
{
    move |gears| {
        gears.sort_by_key(|gear| Reverse(scorer(gear)));
        gears.truncate(size);
    }
}

pub fn select_by_stdev<F>(scorer: F, threshold: f64) -> impl FnMut(&mut Vec<Gear>)
where
    F: Fn(&Gear) -> i32,
{
    move |gears| {
        if !gears.is_empty() {
            gears.sort_by_key(|gear| Reverse(scorer(gear)));
            let scores: Vec<f64> = gears.iter().map(|g| scorer(g) as f64).collect();
            let size = gears.len() as f64;
            let mean = scores.iter().sum::<f64>() / size;
            let variance = scores.iter().map(|score| (score - mean).powi(2)).sum::<f64>() / size;
            let stdev = variance.sqrt();

            let cutoff = mean + threshold * stdev;

            gears.retain(|g| scorer(g) as f64 >= cutoff);
        }
    }
}
