use std::fmt::Debug;

pub trait Solution: Clone + Debug + Send + Sync {
    fn fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
}
