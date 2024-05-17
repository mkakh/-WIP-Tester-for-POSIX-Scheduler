use crate::spec::{cpu, sched_data};

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) cpu: cpu::CPU,
    pub(crate) ready_queue: sched_data::ReadyQueue,
    pub(crate) terminated_tasks : Vec<sched_data::TcbPtr>,
}

impl State {
    // Takes a task from a specified CPU and returns it to the ready queue
    pub fn interrupt(&self, cpu_id: u32) -> State {
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

    // Dispatches a task to an available random CPU core.
    pub fn dispatch(&self, task: sched_data::TcbPtr) -> Vec<State> {
        let mut nexts = vec![];

        let available_cores = self.cpu.cores.iter().filter(|core| core.task.is_none());
        for available_core in available_cores {
            let mut next = self.clone();
            for (i, core) in self.cpu.cores.iter().enumerate() {
                if core.id == available_core.id {
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

    pub(crate) fn schedule(&self) -> Vec<State> {
        let mut next = self.clone();

        // check if there is a task that has a lower priority comparing with the task in the ready queue
        // TODO
        // if there is, interrupt

        if let Some(task) = next.ready_queue.dequeue() {
            next.dispatch(task)
        } else {
            vec![next]
        }
    }
}
