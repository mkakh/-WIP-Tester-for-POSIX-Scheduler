use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TaskState {
    New,
    Ready,
    Running,
    Terminated,
    Waiting,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct TaskControlBlock {
    pub(crate) tid: u32,
    pub(crate) prio: u32,
    pub(crate) state: TaskState,
}

impl TaskControlBlock {
    pub(crate) fn new(tid: u32, prio: u32) -> TaskControlBlock {
        TaskControlBlock {
            tid,
            prio,
            state: TaskState::New,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct ReadyQueue(pub(crate) VecDeque<TaskControlBlock>);

impl ReadyQueue {
    pub(crate) const fn new() -> Self {
        ReadyQueue(VecDeque::new())
    }

    pub(crate) fn enqueue(&mut self, new_task: TaskControlBlock) {
        if self.0.is_empty() {
            self.0.push_back(new_task);
            return;
        }

        if let Some(front_task) = self.front() {
            if new_task.prio > front_task.prio {
                self.0.push_front(new_task);
                return;
            }
        }

        for pos in (1..=self.0.len()).rev() {
            if self.0[pos - 1].prio >= new_task.prio {
                self.0.insert(pos, new_task);
                break;
            }
        }
    }

    pub(crate) fn front(&self) -> Option<&TaskControlBlock> {
        self.0.front()
    }

    pub(crate) fn dequeue(&mut self) -> Option<TaskControlBlock> {
        self.0.pop_front()
    }

    pub(crate) fn iter(&self) -> std::collections::vec_deque::Iter<TaskControlBlock> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut inputs = vec![vec![]];
        const MAX_PRIO: usize = 3;

        for i in 1..=MAX_PRIO {
            inputs.push(vec![i]);
            for j in 1..=MAX_PRIO {
                inputs.push(vec![i, j]);
                for k in 1..=MAX_PRIO {
                    inputs.push(vec![i, j, k]);
                    for l in 1..=MAX_PRIO {
                        for m in 1..=MAX_PRIO {
                            inputs.push(vec![i, j, k, l, m]);
                        }
                    }
                }
            }
        }

        for input in inputs.into_iter() {
            let mut queue = ReadyQueue::new();

            for (i, prio) in input.into_iter().enumerate() {
                queue.enqueue(TaskControlBlock::new((i + 1) as u32, prio as u32))
            }

            for (i, _) in queue.0.iter().enumerate() {
                if queue.0.len() > i + 1 {
                    assert!(queue.0[i].prio >= queue.0[i + 1].prio, "{:?}", queue);
                }
            }
        }
    }

    #[test]
    fn test_simple_enqueue() {
        let mut queue = ReadyQueue(VecDeque::new());

        let task1 = TaskControlBlock {
            tid: 1,
            prio: 1,
            state: TaskState::Ready,
        };

        let task2 = TaskControlBlock {
            tid: 2,
            prio: 2,
            state: TaskState::Ready,
        };

        let task3 = TaskControlBlock {
            tid: 3,
            prio: 3,
            state: TaskState::Ready,
        };

        let task4 = TaskControlBlock {
            tid: 4,
            prio: 2,
            state: TaskState::New,
        };

        queue.enqueue(task1.clone());
        queue.enqueue(task2.clone());
        queue.enqueue(task3.clone());
        queue.enqueue(task4.clone());

        let expected_order = vec![task3, task2, task4, task1];

        for (task, expected_task) in queue.0.iter().zip(expected_order.iter()) {
            assert_eq!(task, expected_task);
        }
    }

    #[test]
    fn test_dequeue() {
        let mut queue = ReadyQueue(VecDeque::new());

        let task1 = TaskControlBlock {
            tid: 1,
            prio: 1,
            state: TaskState::Ready,
        };

        let task2 = TaskControlBlock {
            tid: 2,
            prio: 2,
            state: TaskState::Ready,
        };

        let task3 = TaskControlBlock {
            tid: 3,
            prio: 3,
            state: TaskState::Ready,
        };

        let task4 = TaskControlBlock {
            tid: 4,
            prio: 2,
            state: TaskState::New,
        };

        queue.enqueue(task1.clone());
        queue.enqueue(task2.clone());
        queue.enqueue(task3.clone());
        queue.enqueue(task4.clone());

        let task = queue.dequeue().unwrap();
        assert_eq!(task, task3);

        let expected_order = vec![task2, task4, task1];

        for (task, expected_task) in queue.0.iter().zip(expected_order.iter()) {
            assert_eq!(task, expected_task, "queue: {:?}", queue);
        }
    }
}
