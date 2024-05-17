use crate::spec::scheduler;

pub(crate) struct PthreadCreate;

impl super::FormalizedFunction for PthreadCreate {
    fn name(&self) -> &str {
        "pthread_create"
    }

    // TODO check
    // Priority
    fn args(&self) -> &[(usize, usize)] {
        &[(1, 99)]
    }

    fn call(&self, current: &scheduler::State, args: &[usize]) -> Vec<scheduler::State> {
        assert!(super::check_args(self, args));
        let prio = args[0];

        current.create_task(prio as u32).schedule()
    }
}

pub(crate) static FUNCTION: PthreadCreate = PthreadCreate;
