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

//pub(crate) type TcbPtr = Rc<RefCell<TaskControlBlock>>;
pub(crate) type TcbPtr = Box<TaskControlBlock>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct ReadyQueue(VecDeque<TcbPtr>);

impl ReadyQueue {
    pub(crate) fn new() -> Self {
        ReadyQueue(VecDeque::new())
    }

    pub(crate) fn enqueue(&mut self, new_task: TcbPtr) {
        let mut pos = 0;
        if !self.0.is_empty() {
            for (i, tcb) in self.0.iter().enumerate().rev() {
                if tcb.prio == new_task.prio {
                    pos = i + 1;
                    break;
                }
            }
        }
        self.0.insert(pos, new_task);
    }

    pub(crate) fn front(&self) -> Option<&TcbPtr> {
        self.0.front()
    }

    pub(crate) fn dequeue(&mut self) -> Option<TcbPtr> {
        self.0.pop_front()
    }

    pub(crate) fn iter(&self) -> std::collections::vec_deque::Iter<TcbPtr> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut queue = ReadyQueue(VecDeque::new());

        let task1 = Box::new(TaskControlBlock {
            tid: 1,
            prio: 1,
            state: TaskState::Ready,
        });

        let task2 = Box::new(TaskControlBlock {
            tid: 2,
            prio: 2,
            state: TaskState::Ready,
        });

        let task3 = Box::new(TaskControlBlock {
            tid: 3,
            prio: 3,
            state: TaskState::Ready,
        });

        let task4 = Box::new(TaskControlBlock {
            tid: 4,
            prio: 2,
            state: TaskState::New,
        });

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

        let task1 = Box::new(TaskControlBlock {
            tid: 1,
            prio: 1,
            state: TaskState::Ready,
        });

        let task2 = Box::new(TaskControlBlock {
            tid: 2,
            prio: 2,
            state: TaskState::Ready,
        });

        let task3 = Box::new(TaskControlBlock {
            tid: 3,
            prio: 3,
            state: TaskState::Ready,
        });

        let task4 = Box::new(TaskControlBlock {
            tid: 4,
            prio: 2,
            state: TaskState::New,
        });

        queue.enqueue(task1.clone());
        queue.enqueue(task2.clone());
        queue.enqueue(task3.clone());
        queue.enqueue(task4.clone());

        let task = queue.dequeue().unwrap();
        assert_eq!(task, task3);

        let expected_order = vec![task2, task4, task1];

        for (task, expected_task) in queue.0.iter().zip(expected_order.iter()) {
            assert_eq!(task, expected_task);
        }
    }
}
