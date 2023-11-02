use posix_sched_tester::*;
#[test]
fn td_new() {
    let td = spec::ThreadData::new(1, 3);
    assert_eq!(td.tid, 1);
    assert_eq!(td.prio, 3);
    assert_eq!(td.state, spec::ThreadState::Ready);
}
