mod pthread_create;
mod pthread_exit;
use crate::spec::scheduler;

pub(crate) trait FormalizedFunction {
    fn name(&self) -> &str;
    fn args(&self) -> &[(usize, usize)];
    fn call(&self, current: &scheduler::State, args: &[usize]) -> Vec<scheduler::State>;
}

fn check_args(f: &dyn FormalizedFunction, args: &[usize]) -> bool {
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
pub(crate) enum FormalizedFunctionType {
    PthreadCreate,
    PthreadExit,
}

pub(crate) fn get_function(fn_type: FormalizedFunctionType) -> &'static dyn FormalizedFunction {
    match fn_type {
        FormalizedFunctionType::PthreadCreate => &pthread_create::FUNCTION,
        FormalizedFunctionType::PthreadExit => &pthread_exit::FUNCTION,
    }
}