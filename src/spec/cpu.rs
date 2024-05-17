#![allow(clippy::upper_case_acronyms)]
use crate::spec::sched_data;

#[derive(Clone)]
pub(crate) struct CPU {
    pub(crate) cores: Vec<Core>,
}

#[derive(Clone)]
pub(crate) struct Core {
    pub(crate) id: u32,
    pub(crate) task: Option<sched_data::TcbPtr>,
}

impl CPU {
    pub(crate) fn new(n: u32) -> CPU {
        let mut cores = vec![];
        for id in 0..n {
            cores.push(Core { id, task: None });
        }
        CPU { cores }
    }
}
