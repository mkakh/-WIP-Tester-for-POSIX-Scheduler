use crate::spec::{sched_data::TaskState, scheduler};

pub struct PthreadExit;

impl super::Formalized for PthreadExit {
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

    fn args(&self) -> &[(u32, u32)] {
        &[]
    }

    fn call(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        assert!(self.is_invokable(current, caller, args));

        let mut next = current.clone();
        for (i, core) in current.cpu.cores.iter().enumerate() {
            if let Some(task) = &core.task {
                if task.tid == caller {
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

#[cfg(test)]
mod tests {
    use crate::spec::{function, scheduler};

    #[test]
    fn test_pthread_exit() {
        let mut states = scheduler::State::new(2).create_task(1).schedule();

        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in function::get_function(function::Function::PthreadCreate)
                .call(&state, 1, &[3])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in function::get_function(function::Function::PthreadCreate)
                .call(&state, 1, &[3])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in function::get_function(function::Function::PthreadExit)
                .call(&state, 3, &[])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;
        println!("{:#?}", states);
    }
}
