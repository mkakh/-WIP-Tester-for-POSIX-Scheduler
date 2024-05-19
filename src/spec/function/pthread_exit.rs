use crate::spec::{sched_data::TaskState, scheduler};

pub(crate) struct PthreadExit;

impl super::FormalizedFunction for PthreadExit {
    fn is_invokable(&self, current: &scheduler::State, caller: u32, _args: &[usize]) -> bool {
        for core in current.cpu.cores.iter() {
            if let Some(task) = &core.task {
                if task.tid == caller {
                    return true;
                }
            }
        }

        false
    }

    fn args(&self) -> &[(usize, usize)] {
        &[]
    }

    fn call(
        &self,
        current: &scheduler::State,
        caller: u32,
        args: &[usize],
    ) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        assert!(self.is_invokable(current, caller, args));

        let mut next = current.clone();
        for (i, core) in current.cpu.cores.iter().enumerate() {
            if let Some(task) = &core.task {
                if task.tid == caller as u32 {
                    let mut task = {
                        let t = next.cpu.cores[i].task.take();
                        assert!(t.is_some());
                        t.unwrap()
                    };

                    task.state = TaskState::Terminated;
                    next.terminated_tasks.push(task);
                    return next.schedule();
                }
            }
        }

        unreachable!();
    }
}

pub(crate) static FUNCTION: PthreadExit = PthreadExit;
