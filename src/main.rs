mod oracle_tree;
mod spec;

use spec::{function, scheduler};

fn main() {
    let mut states = scheduler::State::new(2).create_task(1).schedule();

    let mut new_states = vec![];
    for state in states.into_iter() {
        for new_state in function::get_function(function::FormalizedFunctionType::PthreadCreate)
            .call(&state, 1, &[3])
            .into_iter()
        {
            if !new_states.contains(&new_state) {
                new_states.push(new_state);
            }
        }
    }
    states = new_states;

    let mut new_states = vec![];
    for state in states.into_iter() {
        for new_state in function::get_function(function::FormalizedFunctionType::PthreadCreate)
            .call(&state, 1, &[3])
            .into_iter()
        {
            if !new_states.contains(&new_state) {
                new_states.push(new_state);
            }
        }
    }
    states = new_states;

    let mut new_states = vec![];
    for state in states.into_iter() {
        for new_state in function::get_function(function::FormalizedFunctionType::PthreadExit)
            .call(&state, 3, &[])
            .into_iter()
        {
            if !new_states.contains(&new_state) {
                new_states.push(new_state);
            }
        }
    }
    states = new_states;
    println!("{:#?}", states);
}
