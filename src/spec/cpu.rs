#![allow(clippy::upper_case_acronyms)]
use crate::spec::sched_data;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct CPU {
    pub(crate) cores: Vec<Core>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Core {
    pub(crate) id: u32,
    pub(crate) task: Option<sched_data::TaskControlBlock>,
}

impl CPU {
    pub(crate) fn new(n: u32) -> Self {
        let mut cores = vec![];
        for id in 0..n {
            cores.push(Core { id, task: None });
        }
        CPU { cores }
    }

    pub(crate) fn get_idle_cores(&self) -> Vec<&Core> {
        self.cores
            .iter()
            .filter(|core| core.task.is_none())
            .collect()
    }
}
