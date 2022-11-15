use rand::Rng;

pub fn pick<'a, T: ?Sized>(items: &[&'a T]) -> &'a T {
    let random_index: usize = rand::thread_rng().gen_range(0..items.len());
    items.get(random_index).unwrap()
}
