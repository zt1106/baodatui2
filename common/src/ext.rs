pub trait IntoBoxExt<T> {
    fn to_box(self) -> Box<T>;
}

impl<T> IntoBoxExt<T> for T {
    fn to_box(self) -> Box<T> {
        Box::new(self)
    }
}
