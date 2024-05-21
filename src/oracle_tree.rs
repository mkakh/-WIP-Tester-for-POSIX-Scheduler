use crate::spec::{
    function::{get_function, FormalizedFunctionType},
    scheduler::State,
};
use spin::mutex::SpinMutex;

type StateGroup = Vec<State>;

#[derive(Debug, PartialEq, Eq)]
pub struct OracleTree {
    edges: Vec<Edge>,
}

impl OracleTree {
    const fn new() -> Self {
        OracleTree { edges: vec![] }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn init(num_core: u32) {
        let spawn = get_function(FormalizedFunctionType::Spawn);
        let states = spawn.call(&State::new(num_core), 0, &[]);
        let mut new_tree = OracleTree::new();
        new_tree.add_edge(Edge {
            fn_type: FormalizedFunctionType::Spawn,
            args: vec![],
            children: states,
        });
        let mut tree = ORACLE_TREE.lock();
        *tree = new_tree;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    fn_type: FormalizedFunctionType,
    args: Vec<u32>,
    children: StateGroup,
}

pub static ORACLE_TREE: SpinMutex<OracleTree> = SpinMutex::new(OracleTree::new());

#[cfg(test)]
mod tests {
    use super::{Edge, OracleTree, ORACLE_TREE};
    use crate::spec::{
        cpu::{Core, CPU},
        function::FormalizedFunctionType::Spawn,
        sched_data::{ReadyQueue, TaskControlBlock, TaskState::Running},
        scheduler::State,
    };

    #[test]
    fn test_oracle_tree_init() {
        OracleTree::init(2);
        let tree = ORACLE_TREE.lock();
        println!("{:#?}", *tree);
        assert_eq!(
            *tree,
            OracleTree {
                edges: vec![Edge {
                    fn_type: Spawn,
                    args: vec![],
                    children: vec![
                        State {
                            cpu: CPU {
                                cores: vec![
                                    Core {
                                        id: 0,
                                        task: Some(TaskControlBlock {
                                            tid: 1,
                                            prio: 1,
                                            state: Running,
                                        }),
                                    },
                                    Core { id: 1, task: None },
                                ],
                            },
                            ready_queue: ReadyQueue::new(),
                            terminated_tasks: vec![],
                        },
                        State {
                            cpu: CPU {
                                cores: vec![
                                    Core { id: 0, task: None },
                                    Core {
                                        id: 1,
                                        task: Some(TaskControlBlock {
                                            tid: 1,
                                            prio: 1,
                                            state: Running,
                                        }),
                                    },
                                ],
                            },
                            ready_queue: ReadyQueue::new(),
                            terminated_tasks: vec![],
                        },
                    ],
                }],
            }
        );
    }
}
