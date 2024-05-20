use crate::spec::scheduler;

pub struct PthreadCreate;

impl super::FormalizedFunction for PthreadCreate {
    fn is_invokable(&self, current: &scheduler::State, caller: u32, _args: &[u32]) -> bool {
        for core in current.cpu.cores.iter() {
            if let Some(task) = &core.task {
                if task.tid == caller {
                    return true;
                }
            }
        }

        false
    }

    // TODO check
    // Priority
    fn args(&self) -> &[(u32, u32)] {
        &[(1, 99)]
    }

    fn call(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        assert!(self.is_invokable(current, caller, args));

        let prio = args[0];

        current.create_task(prio).schedule()
    }
}

pub static FUNCTION: PthreadCreate = PthreadCreate;

#[cfg(test)]
mod tests {
    use crate::spec::{
        function::{get_function, FormalizedFunctionType},
        sched_data::{ReadyQueue, TaskControlBlock, TaskState},
        scheduler::State,
    };
    use std::collections::VecDeque;

    #[test]
    fn test_pthread_create() {
        let mut states = State::new(2).create_task(1).schedule();

        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in get_function(FormalizedFunctionType::PthreadCreate)
                .call(&state, 1, &[3])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        new_states = vec![];
        for state in states.iter() {
            for new_state in get_function(FormalizedFunctionType::PthreadCreate)
                .call(&state, 1, &[2])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        new_states = vec![];
        for state in states.iter() {
            for new_state in get_function(FormalizedFunctionType::PthreadCreate)
                .call(&state, 2, &[4])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        for state in states.into_iter() {
            let t0 = state.cpu.cores[0].task.as_ref().unwrap();
            let t1 = state.cpu.cores[1].task.as_ref().unwrap();

            assert!((t0.tid == 2 || t1.tid == 2) && (t0.tid == 4 || t1.tid == 4));
            assert!((t0.prio == 3 || t1.prio == 3) && (t0.prio == 4 || t1.prio == 4));
            assert_eq!(
                state.ready_queue,
                ReadyQueue(VecDeque::from(vec![
                    TaskControlBlock {
                        tid: 3,
                        prio: 2,
                        state: TaskState::Ready,
                    },
                    TaskControlBlock {
                        tid: 1,
                        prio: 1,
                        state: TaskState::Ready,
                    },
                ])),
            );
            assert!(state.terminated_tasks.is_empty());
        }
    }
}
