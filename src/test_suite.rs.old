use crate::search::*;
use crate::spec::*;
use memory_stats::memory_stats;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    AND,
    OR,
    TC,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestResult {
    result: bool,
    time: std::time::Duration,
    // How many times the test program was failed to run
    used: Vec<String>,
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "result: {}", self.result)?;
        writeln!(f, "time: {:?}", self.time)?;
        writeln!(f, "used: {:?}", self.used)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    n_type: NodeType,
    seq: Vec<(FnAndArg, SchedState)>,
}

impl Node {
    pub fn new(n_type: NodeType, seq: Vec<(FnAndArg, SchedState)>) -> Self {
        Node { n_type, seq }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}(", self.n_type)?;
        let mut v = vec![];
        for it in self.seq.iter() {
            v.push(format!("({}{:?}({}),{})", it.0 .0, it.0 .1, it.0 .2, it.1));
        }
        write!(f, "{}", v.join(","))?;
        writeln!(f, ")")
    }
}

#[derive(Debug)]
pub struct TestSuite {
    root: Node,
    nodes: Vec<Node>,
    edges: Vec<(Node, Node)>,
}

impl TestSuite {
    pub fn new(root: Node) -> Self {
        TestSuite {
            root: root.clone(),
            nodes: vec![root],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, n: Node) {
        self.nodes.push(n);
    }

    pub fn add_edge(&mut self, n1: Node, n2: Node) {
        self.edges.push((n1, n2));
    }

    // returns the file name of the test pragram
    fn gen_test_program(n: &Node) -> (String, String) {
        TestSuite::cd_to_project_root();
        // The file names of the test programs are "tp_XXXX.cpp", where XXXX is a serial number.
        // get the earliest number that is not used
        let tp_num = {
            let mut i = 0;
            while Path::new(&format!("tp/tp_{}.cpp", i)).exists() {
                i += 1;
            }
            i
        };
        let tp_bin_name = format!("tp/tp_{}", tp_num);
        let tp_src_name = format!("{}.cpp", tp_bin_name);

        let import = r#"#include "../TestProgramGen/util.h""#;
        let mut v = vec![];
        let mut w = vec![];
        for it in n.seq.iter() {
            if it.0 .0 == "_" {
                continue;
            }
            let arg = format!("{:?}", it.0 .1).replace('[', "{").replace(']', "}");
            v.push(format!(
                r#"{{"{}", {}, {}, {{{}}}}}"#,
                it.0 .0,
                arg,
                it.0 .2,
                it.1.get_state()
            ));
            w.push(format!(
                r#"{{"{}{:?}({})",{}}}"#,
                it.0 .0, it.0 .1, it.0 .2, it.1
            ));
        }
        let tseq = format!("test_t test_seq[] = {{{}}};", v.join(","));
        let size = r#"size_t test_seq_size = sizeof(test_seq) / sizeof(test_t);"#;
        let context = [import, &tseq, size].join("\n");
        let mut file = File::create(&tp_src_name).expect("failed to create file");

        file.write_all(context.as_bytes())
            .expect("failed to write file");

        // compile with g++
        let output = Command::new("g++")
            .arg("-std=c++17")
            .arg("-O0")
            .arg("-g3")
            .arg("-Wall")
            .arg("-Wextra")
            .arg("-fsanitize=address")
            .arg("-fno-omit-frame-pointer")
            .arg("-o")
            .arg(tp_bin_name.clone())
            .arg(tp_src_name)
            .arg("TestProgramGen/impl.cpp")
            .arg("TestProgramGen/checker.cpp")
            .arg("TestProgramGen/mapping.cpp")
            .arg("TestProgramGen/util.cpp")
            .arg("TestProgramGen/main.cpp")
            .arg("-lpthread")
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            eprintln!("=== g++ ===");
            eprint!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("===========");
            panic!("failed to compile test program");
        }
        (tp_bin_name, w.join(","))
    }

    fn run_test_program(file_name: &String) -> bool {
        TestSuite::cd_to_project_root();
        // run test program
        let output = Command::new("sudo")
            .arg(file_name)
            .output()
            .expect("failed to execute process");

        eprintln!("=== {} ===", file_name);
        eprint!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("===========");

        output.status.success()
    }

    fn cd_to_project_root() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        std::env::set_current_dir(project_root).expect("failed to change directory");
    }

    pub fn eval(&self) -> TestResult {
        let start = Instant::now();
        TestSuite::cd_to_project_root();
        fn sub_eval(test_suite: &TestSuite, current: &Node, used: &mut Vec<String>) -> bool {
            let mut nexts = test_suite.edges.iter().filter(|(p, _)| p == current);
            match current.n_type {
                NodeType::AND => nexts.all(|(_, c)| sub_eval(test_suite, c, used)),
                NodeType::OR => nexts.any(|(_, c)| sub_eval(test_suite, c, used)),
                NodeType::TC => {
                    if let Some(usage) = memory_stats() {
                        println!("Current physical memory usage: {}", usage.physical_mem);
                        println!("Current virtual memory usage: {}", usage.virtual_mem);
                    } else {
                        println!("Memory stats not available");
                    }

                    let (file_name, tseq) = TestSuite::gen_test_program(current);
                    println!("test program start: {}", tseq);
                    used.push(file_name.clone());
                    TestSuite::run_test_program(&file_name)
                }
            }
        }
        // remove tp directory to clean up
        if Path::new("tp").exists() {
            std::fs::remove_dir_all("tp").expect("failed to remove directory tp");
        }
        std::fs::create_dir("tp").expect("failed to create directory");

        let mut used = vec![];
        let result = sub_eval(self, &self.root, &mut used);
        let time = start.elapsed();
        TestResult { result, time, used }
    }
}

impl std::fmt::Display for TestSuite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn sub_print(
            test_suite: &TestSuite,
            id: String,
            n: &Node,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            write!(f, "{}: {}", id, n)?;
            let children: Vec<&Node> = test_suite
                .edges
                .iter()
                .filter(|(p, _)| p == n)
                .map(|(_, c)| c)
                .collect();
            for (i, c) in children.iter().enumerate() {
                sub_print(test_suite, id.clone() + &format!(".{}", i), c, f)?;
            }
            Ok(())
        }
        sub_print(self, "0".to_string(), &self.root, f)
    }
}
