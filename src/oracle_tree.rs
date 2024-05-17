use crate::spec;

struct State {
    ready_queue: spec::sched_data::ReadyQueue,
    tcb: spec::sched_data::TaskControlBlock,
}
//struct OracleTree {
//    expected_state: State,
//    children: Vec<Spec, Arg, OracleTree>,
//}
//
//impl OracleTree {
//    fn add_child(&mut self, child: State) {
//        self.children.push(child);
//    }
//}
