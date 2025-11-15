use dofus_opti_core::model::Id;

use crate::model::Gear;
use std::cmp::Reverse;
use std::collections::HashSet;

pub fn select_top<F>(size: usize, scorer: F) -> impl FnMut(&mut Vec<Gear>)
where
    F: Fn(&Gear) -> i32,
{
    move |items| {
        items.sort_by_key(|item| Reverse(scorer(item)));
        items.truncate(size);
    }
}

pub fn ignore_ids(ids_ignore: HashSet<Id>) -> impl FnMut(&mut Vec<Gear>) {
    move |gears| {
        gears.retain(|gear| !ids_ignore.contains(&gear.id));
    }
}
