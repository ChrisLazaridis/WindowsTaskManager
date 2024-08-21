use std::collections::HashMap;
use crate::process::Process;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessTree {
    root: Process,
    children: HashMap<i32, Box<ProcessTree>>, // Store children using `pid` as key
}

impl ProcessTree {
    pub fn new(root: Process) -> Self {
        ProcessTree {
            root,
            children: HashMap::new(),
        }
    }

    pub fn get_root(&self) -> &Process {
        &self.root
    }

    pub fn add_child(&mut self, parent: Process, child: Process) -> Result<(), &'static str> {
        if self.root == parent {
            self.children
                .entry(child.get_pid())
                .or_insert(Box::new(ProcessTree::new(child)));
            Ok(())
        } else {
            for subtree in self.children.values_mut() {
                if subtree.add_child(parent.clone(), child.clone()).is_ok() {
                    return Ok(());
                }
            }
            Err("Parent not found")
        }
    }

    pub fn serialize(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn exists(&self, pid: i32) -> bool {
        if self.root.get_pid() == pid {
            return true;
        }
        for subtree in self.children.values() {
            if subtree.exists(pid) {
                return true;
            }
        }
        false
    }
}
