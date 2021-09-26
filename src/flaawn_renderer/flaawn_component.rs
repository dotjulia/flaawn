pub trait FlaawnComponent: Send + Sync {
    fn build(&self) -> String;
}
