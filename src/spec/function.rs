mod pthread_create;
mod pthread_exit;
mod spawn;
use crate::spec::scheduler;

pub trait FormalizedFunction {
    fn is_invokable(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> bool;

    fn args(&self) -> &[(u32, u32)];

    fn call(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> Vec<scheduler::State>;
}

fn check_args(f: &dyn FormalizedFunction, args: &[u32]) -> bool {
    assert_eq!(args.len(), f.args().len());

    for (i, &arg) in args.iter().enumerate() {
        let (min, max) = f.args()[i];
        if !(min <= arg && arg <= max) {
            return false;
        }
    }
    true
}

// TODO change the name
pub enum FormalizedFunctionType {
    PthreadCreate,
    PthreadExit,
    Spawn,
}

pub fn get_function(fn_type: FormalizedFunctionType) -> &'static dyn FormalizedFunction {
    match fn_type {
        FormalizedFunctionType::Spawn => &spawn::FUNCTION,
        FormalizedFunctionType::PthreadCreate => &pthread_create::FUNCTION,
        FormalizedFunctionType::PthreadExit => &pthread_exit::FUNCTION,
    }
}
