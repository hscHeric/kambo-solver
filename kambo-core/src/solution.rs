pub trait Solution: Clone + Send + Sync {
    type ObjectiveValue: PartialOrd + Clone + Send + Sync;

    fn objective_value(&self) -> Self::ObjectiveValue;

    fn set_objective_value(&mut self, value: Self::ObjectiveValue);
}
