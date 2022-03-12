pub trait Stackable<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, v: T);
    fn peek(&self) -> Option<&T>;
    fn top(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}
