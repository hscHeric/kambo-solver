use kambo_core::Solution;

pub trait RepairOperator<S: Solution> {
    fn repair(&self, solution: &mut S) -> bool;
}

impl<S: Solution> RepairOperator<S> for fn(&mut S) -> bool {
    fn repair(&self, _solution: &mut S) -> bool {
        false // Não faz nada.
    }
}
