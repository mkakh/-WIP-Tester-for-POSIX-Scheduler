mod pthread_create;
mod pthread_exit;
mod spawn;
use crate::spec::scheduler;
use strum_macros::EnumIter;

pub trait Formalized {
    fn is_invokable(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> bool;

    fn args(&self) -> &[(u32, u32)];

    fn call(&self, current: &scheduler::State, caller: u32, args: &[u32]) -> Vec<scheduler::State>;
}

fn check_args(f: &dyn Formalized, args: &[u32]) -> bool {
    assert_eq!(args.len(), f.args().len());

    for (i, &arg) in args.iter().enumerate() {
        let (min, max) = f.args()[i];
        if !(min <= arg && arg <= max) {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Function {
    PthreadCreate,
    PthreadExit,
    Spawn,
}

pub fn get_function(fn_type: Function) -> &'static dyn Formalized {
    match fn_type {
        Function::Spawn => &spawn::FUNCTION,
        Function::PthreadCreate => &pthread_create::FUNCTION,
        Function::PthreadExit => &pthread_exit::FUNCTION,
    }
}
