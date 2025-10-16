pub trait First<T> {
    fn first(&mut self) -> Option<T>;
}

impl<T, V: Iterator<Item = T>> First<T> for V {
    fn first(&mut self) -> Option<T> {
        self.next()
    }
}
