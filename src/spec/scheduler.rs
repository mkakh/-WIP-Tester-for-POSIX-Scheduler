use super::sched_data::ReadyQueue;
use crate::spec::{cpu::CPU, sched_data};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub(crate) cpu: CPU,
    pub(crate) ready_queue: sched_data::ReadyQueue,
    pub(crate) terminated_tasks: Vec<sched_data::TcbPtr>,
}

impl State {
    pub fn new(num_core: u32) -> State {
        State {
            cpu: CPU::new(num_core),
            ready_queue: ReadyQueue::new(),
            terminated_tasks: Vec::new(),
        }
    }

    // Takes a task from a specified CPU and returns it to the ready queue
    pub(crate) fn interrupt(&self, cpu_id: u32) -> State {
        let mut next = self.clone();
        for (i, core) in self.cpu.cores.iter().enumerate() {
            if core.id == cpu_id {
                if let Some(mut task) = next.cpu.cores[i].task.take() {
                    task.state = sched_data::TaskState::Ready;
                    next.ready_queue.enqueue(task);
                }
                break;
            }
        }

        next
    }

    // Dispatches a task to an idle random CPU core.
    pub(crate) fn dispatch(&self, task: sched_data::TcbPtr) -> Vec<State> {
        let mut nexts = vec![];

        for idle_core in self.cpu.get_idle_cores().iter() {
            let mut next = self.clone();
            for (i, core) in self.cpu.cores.iter().enumerate() {
                if core.id == idle_core.id {
                    let mut task = task.clone();
                    task.state = sched_data::TaskState::Running;
                    next.cpu.cores[i].task = Some(task);
                    nexts.push(next);
                    break;
                }
            }
        }
        nexts
    }

    // Create a new task and enqueue it to the ready queue
    pub(crate) fn create_task(&self, prio: u32) -> State {
        let tid = {
            let mut tid = 0;

            for core in self.cpu.cores.iter() {
                if let Some(task) = &core.task {
                    tid = std::cmp::max(tid, task.tid);
                }
            }
            for tcb in self.ready_queue.iter() {
                tid = std::cmp::max(tid, tcb.tid);
            }

            tid + 1
        };

        let mut new_task = Box::new(sched_data::TaskControlBlock::new(tid, prio));
        new_task.state = sched_data::TaskState::Ready;
        let mut next = self.clone();
        next.ready_queue.enqueue(new_task);
        next
    }

    pub(crate) fn dispatch_to_all_idle_cores(&self) -> Vec<State> {
        let mut states = vec![self.clone()];
        let mut made_progress = false;

        while {
            let mut new_states = vec![];
            for state in states.into_iter() {
                if !state.cpu.get_idle_cores().is_empty() && state.ready_queue.front().is_some() {
                    made_progress = true;
                    let mut current_state = state;
                    if let Some(task) = current_state.ready_queue.dequeue() {
                        for st in current_state.dispatch(task).into_iter() {
                            if !new_states.contains(&st) {
                                new_states.push(st);
                            }
                        }
                    }
                } else if !new_states.contains(&state) {
                    new_states.push(state);
                }
            }
            states = new_states;
            made_progress
        } {
            made_progress = false;
        }

        states
    }

    pub(crate) fn preempt_to_lower_priority_tasks(&self) -> Vec<State> {
        let mut new_states = vec![];
        if let Some(front_task) = self.ready_queue.front() {
            for core in self.cpu.cores.iter() {
                if let Some(task) = &core.task {
                    if front_task.prio > task.prio {
                        new_states.push(self.interrupt(core.id));
                    }
                }
            }
        }
        new_states
    }

    pub(crate) fn schedule(&self) -> Vec<State> {
        let mut prev_states = vec![self.clone()];
        let mut new_states = vec![];

        while {
            for prev_state in prev_states.iter() {
                for dispatched_state in prev_state.dispatch_to_all_idle_cores().into_iter() {
                    for preempted_state in dispatched_state
                        .preempt_to_lower_priority_tasks()
                        .into_iter()
                    {
                        for new_state in preempted_state.dispatch_to_all_idle_cores().into_iter() {
                            if !new_states.contains(&new_state) {
                                new_states.push(new_state);
                            }
                        }
                    }
                }
            }
            new_states != prev_states
        } {
            prev_states = new_states;
            new_states = vec![];
        }

        new_states
    }
}
