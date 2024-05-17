use crate::spec::{sched_data::TaskState, scheduler};

pub(crate) struct PthreadExit;

impl super::FormalizedFunction for PthreadExit {
    fn name(&self) -> &str {
        "pthread_exit"
    }

    // TODO check
    // TID
    fn args(&self) -> &[(usize, usize)] {
        &[(1, 99)]
    }

    fn call(&self, current: &scheduler::State, args: &[usize]) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        let tid = args[0];

        let mut next = current.clone();
        for (i, core) in current.cpu.cores.iter().enumerate() {
            if let Some(task) = &core.task {
                if task.tid == tid as u32 {
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
