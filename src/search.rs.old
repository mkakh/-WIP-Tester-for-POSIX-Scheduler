use crate::spec::*;
use crate::test_suite::*;
use crate::*;

// e.g.) [(0,2),(1,1),(3,4)] |-> [[0, 1, 3], [0, 1, 4], [1, 1, 3], [1, 1, 4], [2, 1, 3], [2, 1, 4]]
fn cartesian_product(ak: &[(usize, usize)]) -> Vec<Vec<usize>> {
    if ak.is_empty() {
        vec![vec![]]
    } else if ak.len() == 1 {
        (ak[0].0..=ak[0].1).map(|x| vec![x]).collect()
    } else {
        let mut nxt_ak: Vec<(usize, usize)> = ak.to_vec();
        let (fst, snd) = nxt_ak.pop().unwrap();
        iproduct!(cartesian_product(&nxt_ak), fst..=snd)
            .map(|(mut a, b)| {
                a.append(&mut vec![b]);
                a
            })
            .collect()
    }
}

pub struct Func {
    name: FuncName,
    range_of_args: Vec<(usize, usize)>,
    tr: Box<dyn FuncInvoke + Send + Sync>,
}

impl Func {
    pub fn new(
        name: FuncName,
        range_of_args: Vec<(usize, usize)>,
        tr: Box<dyn FuncInvoke + Send + Sync>,
    ) -> Self {
        Self {
            name,
            range_of_args,
            tr,
        }
    }
}

pub trait FuncInvoke {
    fn invoke(&self, invoker: usize, arg: &FuncArg, st: &SchedState) -> Vec<SchedState>;
}

pub struct TestParameter {
    pub max_tid: usize,
    pub max_prio: usize,
    pub n_core: usize,
}

type FuncName = String;
pub type FuncArg = Vec<usize>;
type FuncInvoker = usize;
pub type FnAndArg = (FuncName, FuncArg, FuncInvoker);

pub struct SearchContext<'a> {
    pub api: &'a [&'a Func],
    pub test_param: &'a TestParameter,
    //pub or_id: &'a mut usize,
}

pub fn search(
    func: &FnAndArg,
    state: &SchedState,
    history: &[(FnAndArg, SchedState)],
    context: &mut SearchContext,
    test_suite: &mut TestSuite,
) -> Node {
    let mut history = history.to_owned();
    history.push((func.clone(), state.clone()));

    // list of transitions from the current state to the adjacent states
    let (fns, transitions) = get_transitions(state, &history, context);

    let n = if fns.is_empty() {
        Node::new(NodeType::TC, history.clone())
    } else {
        let n = Node::new(NodeType::AND, history.clone());
        for f in &fns {
            let next_states = get_next_states(f, &transitions);
            let nn = if next_states.len() > 1 {
                handle_uncertainty(f, &history, &transitions, context, test_suite)
            } else {
                search(f, next_states[0], &history, context, test_suite)
            };
            test_suite.add_edge(n.clone(), nn);
        }
        n
    };

    test_suite.add_node(n.clone());
    n
}

fn get_next_states<'a>(
    func: &'a FnAndArg,
    transitions: &'a [(FnAndArg, SchedState)],
) -> Vec<&'a SchedState> {
    transitions
        .iter()
        .filter(|(x, _)| x == func)
        .map(|(_, y)| y)
        .collect_vec()
}

// Helper function to get a list of transitions from the current state to the adjacent states
fn get_transitions(
    state: &SchedState,
    history: &[(FnAndArg, SchedState)],
    context: &mut SearchContext,
) -> (Vec<FnAndArg>, Vec<(FnAndArg, SchedState)>) {
    let mut invokable_fn = vec![];
    let mut transitions = vec![];
    for f in context.api.iter() {
        for invoker in 0..=context.test_param.max_tid {
            for arg in cartesian_product(&f.range_of_args).iter() {
                // the states after invoking the function and scheduling and omitting visited states
                let next_states =
                    f.tr.invoke(invoker, arg, state)
                        .iter()
                        .flat_map(|x| schedule(x, context.test_param))
                        .filter(|x| history.iter().all(|y| y.1 != *x))
                        .collect_vec();
                let next_states = next_states
                    .into_iter()
                    .filter(|x| check_constraints(x, context))
                    .collect_vec();
                if !next_states.is_empty() {
                    invokable_fn.push((f.name.clone(), arg.clone(), invoker));
                }
                transitions.append(
                    &mut next_states
                        .iter()
                        .map(|st| ((f.name.clone(), arg.clone(), invoker), st.clone()))
                        .collect(),
                );
            }
        }
    }
    (invokable_fn, transitions)
}

// Constraints for the search
fn check_constraints(state: &SchedState, context: &SearchContext) -> bool {
    state.tcb.0.len() <= context.test_param.max_tid
        && state.tcb.0.iter().max_by_key(|x| x.prio).unwrap().prio <= context.test_param.max_prio
}

// Helper function for search function to generate a OR node
fn handle_uncertainty(
    func: &FnAndArg,
    history: &[(FnAndArg, SchedState)],
    transitions: &[(FnAndArg, SchedState)],
    context: &mut SearchContext,
    test_suite: &mut TestSuite,
) -> Node {
    let n = Node::new(NodeType::OR, history.to_vec());
    test_suite.add_node(n.clone());
    let next_states = get_next_states(func, transitions);
    for st in next_states.iter() {
        let nn = search(func, st, history, context, test_suite);
        test_suite.add_edge(n.clone(), nn);
    }
    n
}
