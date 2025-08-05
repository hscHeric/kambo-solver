use std::time::Duration;

#[derive(Debug, Clone)]
pub enum TerminationCriteria {
    MaxGenerations(u32),
    MaxDuration(Duration),
    NoImprovementFor(u32),
}
