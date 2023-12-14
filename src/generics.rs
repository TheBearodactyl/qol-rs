use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn debug_info<T: std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}

pub fn execute_on_each<T, F>(vec: Vec<T>, f: F)
where
    F: Fn(T),
{
    vec.into_iter().for_each(f);
}

pub fn vec_to_hashmap<T, U>(vec: Vec<(T, U)>) -> HashMap<T, U>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    vec.into_iter().collect()
}

pub fn extend_with_defaults<T: Default>(vec: &mut Vec<T>, count: usize) {
    vec.extend(std::iter::repeat_with(T::default).take(count));
}

pub fn deduplicate<T: Eq + Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

pub fn sort_desc<T: Ord>(vec: &mut Vec<T>) {
    vec.sort_by(|a, b| b.cmp(a));
}

pub fn find_index<T, F>(vec: Vec<T>, predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    vec.iter().position(|x| predicate(x))
}

pub fn zip_vecs<T, U>(vec1: Vec<T>, vec2: Vec<U>) -> Vec<(T, U)> {
    vec1.into_iter().zip(vec2.into_iter()).collect()
}

pub fn map_option<T, U, F>(value: Option<T>, f: F) -> Option<U>
where
    F: FnOnce(T) -> U,
{
    value.map(f)
}

pub fn calculate_average<T>(values: &[T]) -> Option<f64>
where
    T: std::ops::Add<Output = T> + Clone + Default + From<usize>,
    f64: From<T>,
{
    let sum: T = values
        .iter()
        .cloned()
        .fold(Default::default(), |acc, x| acc + x);

    if values.is_empty() {
        None
    } else {
        Some(f64::from(sum) / values.len() as f64)
    }
}

pub fn merge_vecs<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T>
where
    T: Clone,
{
    [vec1, vec2].concat()
}

pub fn filter_by<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    vec.into_iter().filter(|x| predicate(x)).collect()
}

pub fn clone_and_double<T: Clone + std::ops::Mul<Output = T>>(vec: &Vec<T>) -> Vec<T>
where
    T: Copy,
{
    vec.iter().cloned().map(|x| x * x).collect()
}

pub fn find_max<T: Ord>(slice: &[T]) -> Option<&T> {
    slice.iter().max()
}

pub fn swap_elements<T>(vec: &mut Vec<T>, index1: usize, index2: usize) {
    vec.swap(index1, index2);
}
