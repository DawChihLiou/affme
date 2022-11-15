use rand::Rng;

pub fn pick<T: Clone>(vec: &Vec<T>) -> Option<T> {
    let random_index: usize = rand::thread_rng().gen_range(0..vec.len());
    vec.get(random_index).cloned()
}
