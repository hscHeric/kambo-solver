use kambo_core::Solution;
use rand::Rng;

pub trait AsVectorSolution {
    type Gene: Clone;
    fn genes(&self) -> &[Self::Gene];
    fn genes_mut(&mut self) -> &mut [Self::Gene];
}

pub trait CrossoverOperator<S: Solution> {
    fn crossover<R: Rng>(&self, rng: &mut R, parent1: &S, parent2: &S) -> (S, S);
}

pub struct OnePoint;

impl<S> CrossoverOperator<S> for OnePoint
where
    S: Solution + Clone + AsVectorSolution,
{
    fn crossover<R: Rng>(&self, rng: &mut R, parent1: &S, parent2: &S) -> (S, S) {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        let genes1 = child1.genes_mut();
        let genes2 = child2.genes_mut();

        let len = genes1.len();
        if len < 2 {
            return (child1, child2);
        }

        let crossover_point = rng.random_range(1..len);

        genes1[crossover_point..].swap_with_slice(&mut genes2[crossover_point..]);

        (child1, child2)
    }
}

pub struct TwoPoint;

impl<S> CrossoverOperator<S> for TwoPoint
where
    S: Solution + Clone + AsVectorSolution,
{
    fn crossover<R: Rng>(&self, rng: &mut R, parent1: &S, parent2: &S) -> (S, S) {
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        let genes1 = child1.genes_mut();
        let genes2 = child2.genes_mut();

        let len = genes1.len();
        if len < 3 {
            return (child1, child2);
        }

        let point1 = rng.random_range(1..len - 1);
        let point2 = rng.random_range(point1..len);

        if point1 == point2 {
            return (child1, child2);
        }

        genes1[point1..point2].swap_with_slice(&mut genes2[point1..point2]);

        (child1, child2)
    }
}
