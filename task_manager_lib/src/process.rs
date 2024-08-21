// derive clone eq and hash
use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Process {
    pid: i32,
    name: String,
    children_by_id: Vec<i32>,
}

impl Process {
    pub fn new(pid: i32, name: String) -> Process {
        Process {
            pid,
            name,
            children_by_id: Vec::new(),
        }
    }
    pub fn get_pid(&self) -> i32 {
        self.pid
    }
    pub fn has_children(&self) -> bool {
        !self.children_by_id.is_empty()
    }
    pub fn add_child(&mut self, child: i32) {
        self.children_by_id.push(child);

    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_children(&self) -> &Vec<i32> {
        &self.children_by_id
    } 
}