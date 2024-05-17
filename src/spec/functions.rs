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

pub(crate) struct PthreadCreate;

impl FormalizedFunction for PthreadCreate {
    fn name(&self) -> &str {
        "pthread_create"
    }

    // Priority
    fn args(&self) -> &[(usize, usize)] {
        &[(1, 99)]
    }

    fn call(&self, current: &scheduler::State, args: &[usize]) -> Vec<scheduler::State> {
        assert!(check_args(self, args));
        let prio = args[0];

        current.create_task(prio as u32).schedule()
    }
}

pub(crate) static PTHREAD_CREATE: PthreadCreate = PthreadCreate;

// TODO change the name
pub(crate) enum FormalizedFunctionType {
    PthreadCreate,
}

pub(crate) fn get_function(fn_type: FormalizedFunctionType) -> &'static dyn FormalizedFunction {
    match fn_type {
        FormalizedFunctionType::PthreadCreate => &PTHREAD_CREATE,
    }
}
