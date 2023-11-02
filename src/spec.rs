use crate::search::*;
use crate::*;
use once_cell::sync::Lazy;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TCB(pub Vec<ThreadData>);
impl TCB {
    pub fn get_running_lowest_priority(&self) -> Option<usize> {
        self.0
            .iter()
            .filter(|td| td.state == ThreadState::Running)
            .map(|td| td.prio)
            .min()
    }

    pub fn get(&self, tid: usize) -> Option<&ThreadData> {
        self.0.get(tid)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ThreadData {
    pub tid: usize,
    pub prio: usize,
    pub state: ThreadState,
}
impl ThreadData {
    pub fn new(tid: usize, prio: usize) -> ThreadData {
        ThreadData {
            tid,
            prio,
            state: ThreadState::Running,
        }
    }
}

impl std::fmt::Display for ThreadData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{:?})", self.tid, self.prio, self.state)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadyQueue(Vec<VecDeque<usize>>);

impl Default for ReadyQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ReadyQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = vec![];
        for q in self.0.iter() {
            let mut w = vec![];
            for tid in q.iter() {
                w.push(format!("{}", tid));
            }
            v.push(format!("[{}]", w.join(",")));
        }
        write!(f, "{}", v.join(","))
    }
}

impl ReadyQueue {
    pub fn new() -> ReadyQueue {
        ReadyQueue(vec![VecDeque::new()])
    }

    pub fn enqueue(&mut self, tid: usize, prio: usize) {
        while self.0.len() <= prio {
            self.0.push(VecDeque::new());
        }
        self.0.get_mut(prio).unwrap().push_back(tid);
    }

    pub fn dequeue(&mut self) -> Option<usize> {
        // return highest prio threads from ready queue
        for q in self.0.iter_mut().rev() {
            if let Some(tid) = q.pop_front() {
                return Some(tid);
            }
        }
        None
    }

    pub fn insert(&mut self, tid: usize, prio: usize) {
        while self.0.len() <= prio {
            self.0.push(VecDeque::new());
        }
        self.0.get_mut(prio).unwrap().push_front(tid);
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|q| q.is_empty())
    }

    pub fn get_highest_priority(&self) -> Option<usize> {
        // priority is from 0 to max_prio
        self.0.iter().position(|q| !q.is_empty())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchedState {
    pub tcb: TCB,
    pub ready_queue: ReadyQueue,
}

impl SchedState {
    pub fn get_state(&self) -> String {
        let mut v = vec![];
        for td in self.tcb.0.iter() {
            v.push(format!("{:?}", td.state).to_uppercase());
        }
        format!("{{{}}}", v.join(","))
    }
}
impl std::fmt::Display for SchedState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = vec![];
        for td in self.tcb.0.iter() {
            v.push(format!("{}", td));
        }
        write!(f, "{{{}}}", v.join(","))
    }
}

pub struct InvPthreadCreate;
impl FuncInvoke for InvPthreadCreate {
    fn invoke(&self, invoker: usize, arg: &FuncArg, state: &SchedState) -> Vec<SchedState> {
        // the len of arguments is 1, the priority of the new thread
        assert!(arg.len() == 1);
        let mut v = vec![];
        if let Some(invoker_td) = state.tcb.get(invoker) {
            if invoker_td.state == ThreadState::Running {
                let mut new_state = state.clone();
                let (tid, prio) = (new_state.tcb.0.len(), arg[0]);
                new_state.tcb.0.push(ThreadData::new(tid, prio));
                v.push(new_state);
            }
        }
        v
    }
}

pub struct InvPthreadExit;
impl FuncInvoke for InvPthreadExit {
    fn invoke(&self, invoker: usize, arg: &FuncArg, state: &SchedState) -> Vec<SchedState> {
        assert!(arg.is_empty());
        // we assume that the main thread is always running
        if invoker == 0 {
            return vec![];
        }
        let mut v = vec![];
        if let Some(invoker_td) = state.tcb.get(invoker) {
            if invoker_td.state == ThreadState::Running {
                let mut new_state = state.clone();
                new_state.tcb.0[invoker].state = ThreadState::Terminated;
                v.push(new_state);
            }
        }
        v
    }
}

pub fn schedule(st: &SchedState, param: &TestParameter) -> Vec<SchedState> {
    // make threads running up to n_core
    fn ready_to_running(st: &SchedState, param: &TestParameter) -> SchedState {
        let mut st = st.clone();
        while !check_n_running(&st, param) {
            if let Some(tid) = st.ready_queue.dequeue() {
                st.tcb.0.iter_mut().find(|x| x.tid == tid).unwrap().state = ThreadState::Running;
            }
        }
        st
    }

    // the running threads could be back to ready anytime
    fn running_to_ready(st: &SchedState, param: &TestParameter) -> Vec<SchedState> {
        let mut v = vec![st.clone()];
        let mut ch_flag: bool = true;
        while ch_flag {
            ch_flag = false;
            let mut w = v.clone();
            for st in v.iter() {
                for td in st.tcb.0.iter() {
                    if td.state == ThreadState::Running {
                        let mut new_st = st.clone();
                        new_st.tcb.0[td.tid].state = ThreadState::Ready;
                        new_st.ready_queue.insert(td.tid, td.prio);
                        if !w.contains(&new_st) {
                            ch_flag = true;
                            w.push(new_st.to_owned());
                        }
                    }
                }
            }
            v = w;
        }

        v.into_iter()
            .filter(|st| n_running(st) <= param.n_core)
            .collect()
    }

    fn n_running(st: &SchedState) -> usize {
        st.tcb
            .0
            .iter()
            .filter(|x| x.state == ThreadState::Running)
            .count()
    }

    fn check_n_running(st: &SchedState, param: &TestParameter) -> bool {
        let num_of_running = n_running(st);
        num_of_running == param.n_core
            || (num_of_running < param.n_core && st.ready_queue.is_empty())
    }

    fn check_prio(st: &SchedState) -> bool {
        // there is no thread running which has lower prio than ready threads
        st.tcb.get_running_lowest_priority() >= st.ready_queue.get_highest_priority()
    }

    let mut v: VecDeque<SchedState> = {
        let v = if n_running(st) <= param.n_core {
            vec![st.clone()]
        } else {
            running_to_ready(st, param)
        };

        let mut w = vec![];
        for s in v.iter() {
            let s = ready_to_running(s, param);
            if !w.contains(&s) {
                w.push(s);
            }
        }
        assert!(!w.is_empty());
        VecDeque::from(w)
    };

    // swap threads if there is a thread which has lower prio than ready threads
    while v.iter().all(|st| !check_prio(st)) {
        let mut st = v.pop_front().unwrap();
        // get running threads which has lowest prio
        let lowest_prio = st.tcb.get_running_lowest_priority().unwrap();
        let lowest_prio_tid = st
            .tcb
            .0
            .iter()
            .filter(|x| x.state == ThreadState::Running && x.prio == lowest_prio)
            .map(|x| x.tid)
            .collect::<Vec<_>>();

        // get ready threads which has higher prio than running threads
        let ready_tid = st.ready_queue.dequeue().unwrap();
        for tid in lowest_prio_tid {
            let mut new_st = st.clone();
            new_st
                .tcb
                .0
                .iter_mut()
                .find(|x| x.tid == tid)
                .unwrap()
                .state = ThreadState::Ready;
            new_st
                .tcb
                .0
                .iter_mut()
                .find(|x| x.tid == ready_tid)
                .unwrap()
                .state = ThreadState::Running;
            new_st.ready_queue.insert(tid, lowest_prio);
            v.push_back(new_st);
        }
    }

    assert!(v.iter().all(|st| check_n_running(st, param)));
    v.into_iter().collect()
}

pub static PTHREAD_CREATE: Lazy<Func> = Lazy::new(|| {
    Func::new(
        "PthreadCreate".to_string(),
        vec![(1, 99)],
        Box::new(InvPthreadCreate),
    )
});

pub static PTHREAD_EXIT: Lazy<Func> =
    Lazy::new(|| Func::new("PthreadExit".to_string(), vec![], Box::new(InvPthreadExit)));
