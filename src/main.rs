use posix_sched_tester::spec::{function, scheduler};
fn main() {
    let state = scheduler::State::new(2);
    let new_states =
        function::get_function(function::FormalizedFunctionType::PthreadCreate).call(&state, &[3]);
    println!("{:?}", new_states);
}
