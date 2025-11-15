use std::cmp::Reverse;

pub fn select_top<A, F>(size: usize, scorer: F, items: &mut Vec<A>)
where
    F: Fn(&A) -> i32,
{
    items.sort_by_key(|item| Reverse(scorer(item)));
    items.truncate(size);
}
