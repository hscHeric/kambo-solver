use kambo_core::Solution;

pub trait RepairOperator<S>
where
    S: Solution,
{
    fn repair(&self, solution: &mut S) -> bool;
}

#[derive(Default, Clone, Copy)]
pub struct NoOpRepair;

impl<S> RepairOperator<S> for NoOpRepair
where
    S: Solution,
{
    fn repair(&self, _solution: &mut S) -> bool {
        // Não faz nada e retorna false.
        false
    }
}
