pub fn repeat<E>(n: usize, e: E) -> Vec<E>
where
    E: Clone,
{
    (0..n).map(|_| e.clone()).collect()
}

const DEBUG_MODE: bool = false;
pub fn debug(s: String) {
    if DEBUG_MODE {
        println!("{}", s);
    }
}
