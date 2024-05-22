use crate::spec::scheduler::State;

pub struct Spawn;

// Pseudo function that corresponding to the procedure when this program is launched
impl super::Formalized for Spawn {
    fn is_invokable(&self, _: &State, _: u32, _: &[u32]) -> bool {
        false
    }

    fn args(&self) -> &[(u32, u32)] {
        &[]
    }

    fn call(&self, state: &State, _: u32, _: &[u32]) -> Vec<State> {
        let num_core = state.cpu.cores.len() as u32;
        State::new(num_core).create_task(1).schedule()
    }
}

pub static FUNCTION: Spawn = Spawn;

#[cfg(test)]
mod tests {
    use crate::spec::{
        cpu::{Core, CPU},
        function::{get_function, Function},
        sched_data::{ReadyQueue, TaskControlBlock, TaskState},
        scheduler::State,
    };

    #[test]
    fn test_spawn() {
        let spawn = get_function(Function::Spawn);
        let states = spawn.call(&State::new(2), 0, &[]);

        assert_eq!(
            states,
            vec![
                State {
                    cpu: CPU {
                        cores: vec![
                            Core {
                                id: 0,
                                task: Some(TaskControlBlock {
                                    tid: 1,
                                    prio: 1,
                                    state: TaskState::Running,
                                },),
                            },
                            Core { id: 1, task: None },
                        ],
                    },
                    ready_queue: ReadyQueue::new(),
                    terminated_tasks: vec![],
                },
                State {
                    cpu: CPU {
                        cores: vec![
                            Core { id: 0, task: None },
                            Core {
                                id: 1,
                                task: Some(TaskControlBlock {
                                    tid: 1,
                                    prio: 1,
                                    state: TaskState::Running,
                                },),
                            },
                        ],
                    },
                    ready_queue: ReadyQueue::new(),
                    terminated_tasks: vec![],
                },
            ]
        );
    }
}
