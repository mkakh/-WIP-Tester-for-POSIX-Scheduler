#![allow(clippy::upper_case_acronyms)]
use crate::search::*;
use crate::spec::*;
use crate::test_suite::*;
use once_cell::sync::Lazy;
use posix_sched_tester::*;

fn main() {
    //let init_fn = ("PthreadCreate".to_string(), vec![1], 0);
    let init_fn = ("_".to_string(), vec![], 0);
    let init_state: SchedState = SchedState {
        tcb: TCB(vec![ThreadData {
            tid: 0,
            prio: 1,
            state: ThreadState::Running,
        }]),
        ready_queue: ReadyQueue::new(),
    };
    let root_node: Node = Node::new(NodeType::AND, vec![(init_fn.clone(), init_state.clone())]);

    let mut test_suite = TestSuite::new(root_node);

    let api: Vec<&Func> = [&PTHREAD_CREATE, &PTHREAD_EXIT]
        .iter()
        .map(|x| Lazy::force(x))
        .collect();
    search(
        &init_fn,
        &init_state,
        &[],
        &mut SearchContext {
            api: &api,
            test_param: &TestParameter {
                max_tid: 3,
                max_prio: 2,
                n_core: 2,
            },
        },
        &mut test_suite,
    );
    println!("{}", test_suite);
    let result = test_suite.eval();
    println!("{}", result);
}
