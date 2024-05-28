use itertools::iproduct;
use strum::IntoEnumIterator;

use crate::function::{get_function, Function};
use crate::oracle_tree::{Edge, Node, OracleTree, ORACLE_TREE};

// TODO
const MAX_TID: u32 = 4;

fn search(num_core: u32) {
    OracleTree::init(num_core);
    let tree = ORACLE_TREE.lock();
    let mut stack: Vec<&Node> = tree.get_init_nodes();
    let mut path = vec![];

    while let Some(current) = stack.pop() {
        path.push(current);
        for func in Function::iter() {
            let f = get_function(func);
            for args in cartesian_product(f.args()).into_iter() {
                for caller in 1_u32..=MAX_TID {
                    if f.is_invokable(current.get_state(), caller, &args) {
                        let mut node_group = vec![];
                        for next in f.call(current.get_state(), caller, &args) {
                            let node = Node::new(next);
                            stack.push(&node);
                            node_group.push(node);
                        }
                    }
                }
            }
        }
    }
}

// e.g.) [(0,2),(1,1),(3,4)] |-> [[0, 1, 3], [0, 1, 4], [1, 1, 3], [1, 1, 4], [2, 1, 3], [2, 1, 4]]
fn cartesian_product(ak: &[(u32, u32)]) -> Vec<Vec<u32>> {
    if ak.is_empty() {
        vec![vec![]]
    } else if ak.len() == 1 {
        (ak[0].0..=ak[0].1).map(|x| vec![x]).collect()
    } else {
        let mut nxt_ak: Vec<(u32, u32)> = ak.to_vec();
        let (fst, snd) = nxt_ak.pop().unwrap();
        iproduct!(cartesian_product(&nxt_ak), fst..=snd)
            .map(|(mut a, b)| {
                a.append(&mut vec![b]);
                a
            })
            .collect()
    }
}
