use crate::spec::scheduler;

pub(crate) struct PthreadCreate;

impl super::FormalizedFunction for PthreadCreate {
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

    // TODO check
    // Priority
    fn args(&self) -> &[(usize, usize)] {
        &[(1, 99)]
    }

    fn call(
        &self,
        current: &scheduler::State,
        caller: u32,
        args: &[usize],
    ) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        assert!(self.is_invokable(current, caller, args));

        let prio = args[0];

        current.create_task(prio as u32).schedule()
    }
}

pub static FUNCTION: PthreadCreate = PthreadCreate;

#[cfg(test)]
mod tests {
    use crate::spec::{
        cpu::{Core, CPU},
        function::{get_function, FormalizedFunctionType},
        sched_data::{ReadyQueue, TaskControlBlock, TaskState},
        scheduler::State,
    };
    use std::collections::VecDeque;

    #[test]
    fn test_pthread_create() {
        let state = State::new(2);
        assert_eq!(
            state,
            State {
                cpu: CPU {
                    cores: vec![Core { id: 0, task: }, Core { id: 1, task: None }]
                },
                ready_queue: ReadyQueue::new(),
                terminated_tasks: vec![]
            }
        );

        let new_states = get_function(FormalizedFunctionType::PthreadCreate).call(&state, &[3]);

        assert_eq!(
            new_states,
            vec![
                State {
                    cpu: CPU {
                        cores: vec![
                            Core {
                                id: 0,
                                task: Some(Box::new(TaskControlBlock {
                                    tid: 1,
                                    prio: 3,
                                    state: TaskState::Running
                                }))
                            },
                            Core { id: 1, task: None }
                        ]
                    },
                    ready_queue: ReadyQueue::new(),
                    terminated_tasks: vec![]
                },
                State {
                    cpu: CPU {
                        cores: vec![
                            Core { id: 0, task: None },
                            Core {
                                id: 1,
                                task: Some(Box::new(TaskControlBlock {
                                    tid: 1,
                                    prio: 3,
                                    state: TaskState::Running
                                }))
                            }
                        ]
                    },
                    ready_queue: ReadyQueue::new(),
                    terminated_tasks: vec![]
                }
            ]
        );
    }

    #[test]
    fn test_multiple_pthread_create() {
        let state = State::new(2);

        let mut states = get_function(FormalizedFunctionType::PthreadCreate).call(&state, &[3]);

        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in get_function(FormalizedFunctionType::PthreadCreate)
                .call(&state, &[3])
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
                .call(&state, &[1])
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
                .call(&state, &[4])
                .into_iter()
            {
                if !new_states.contains(&new_state) {
                    new_states.push(new_state);
                }
            }
        }
        states = new_states;

        let expected_result = vec![
            State {
                cpu: CPU {
                    cores: vec![
                        Core {
                            id: 0,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 4,
                                prio: 4,
                                state: TaskState::Running,
                            })),
                        },
                        Core {
                            id: 1,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 2,
                                prio: 3,
                                state: TaskState::Running,
                            })),
                        },
                    ],
                },
                ready_queue: ReadyQueue(VecDeque::from(vec![
                    Box::new(TaskControlBlock {
                        tid: 1,
                        prio: 3,
                        state: TaskState::Ready,
                    }),
                    Box::new(TaskControlBlock {
                        tid: 3,
                        prio: 1,
                        state: TaskState::Ready,
                    }),
                ])),
                terminated_tasks: vec![],
            },
            State {
                cpu: CPU {
                    cores: vec![
                        Core {
                            id: 0,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 1,
                                prio: 3,
                                state: TaskState::Running,
                            })),
                        },
                        Core {
                            id: 1,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 4,
                                prio: 4,
                                state: TaskState::Running,
                            })),
                        },
                    ],
                },
                ready_queue: ReadyQueue(VecDeque::from(vec![
                    Box::new(TaskControlBlock {
                        tid: 2,
                        prio: 3,
                        state: TaskState::Ready,
                    }),
                    Box::new(TaskControlBlock {
                        tid: 3,
                        prio: 1,
                        state: TaskState::Ready,
                    }),
                ])),
                terminated_tasks: vec![],
            },
            State {
                cpu: CPU {
                    cores: vec![
                        Core {
                            id: 0,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 4,
                                prio: 4,
                                state: TaskState::Running,
                            })),
                        },
                        Core {
                            id: 1,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 1,
                                prio: 3,
                                state: TaskState::Running,
                            })),
                        },
                    ],
                },
                ready_queue: ReadyQueue(VecDeque::from(vec![
                    Box::new(TaskControlBlock {
                        tid: 2,
                        prio: 3,
                        state: TaskState::Ready,
                    }),
                    Box::new(TaskControlBlock {
                        tid: 3,
                        prio: 1,
                        state: TaskState::Ready,
                    }),
                ])),
                terminated_tasks: vec![],
            },
            State {
                cpu: CPU {
                    cores: vec![
                        Core {
                            id: 0,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 2,
                                prio: 3,
                                state: TaskState::Running,
                            })),
                        },
                        Core {
                            id: 1,
                            task: Some(Box::new(TaskControlBlock {
                                tid: 4,
                                prio: 4,
                                state: TaskState::Running,
                            })),
                        },
                    ],
                },
                ready_queue: ReadyQueue(VecDeque::from(vec![
                    Box::new(TaskControlBlock {
                        tid: 1,
                        prio: 3,
                        state: TaskState::Ready,
                    }),
                    Box::new(TaskControlBlock {
                        tid: 3,
                        prio: 1,
                        state: TaskState::Ready,
                    }),
                ])),
                terminated_tasks: vec![],
            },
        ];
        assert_eq!(expected_result, states);
    }
}
