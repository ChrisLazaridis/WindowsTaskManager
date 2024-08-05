#[derive(Clone)]
pub struct Process {
    pid: i32,
    name: String,
    children: Vec<Process>,
}

impl Process {
    pub fn new(pid: i32, name: String) -> Process {
        Process {
            pid,
            name,
            children: Vec::new(),
        }
    }
    pub fn get_pid(&self) -> i32 {
        self.pid
    }
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn add_child(&mut self, child: &Process) {
        self.children.push(child.clone());
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_children(&self) -> &Vec<Process> {
        &self.children
    }
    pub fn traverse(&self) -> Vec<Process> {
        let mut result = Vec::new();
        self.traverse_helper(&mut result);
        result
    }


    fn traverse_helper(&self, result: &mut Vec<Process>) {
        result.push(self.clone());
        if self.has_children() {
            for child in &self.children {
                child.traverse_helper(result);
            }
        }
    }
}