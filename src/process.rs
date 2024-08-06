#[derive(Clone)]
pub struct Process {
    pid: i32,
    name: String,
    children: Vec<i32>,
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
    pub fn add_child(&mut self, child: i32) {
        self.children.push(child);

    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_children(&self) -> &Vec<i32> {
        &self.children
    }
    pub fn traverse(&self) -> Vec<&i32> {
        let mut result = Vec::new();
        result.push(self);
        for child in &self.children {
            result.extend(child.traverse());
        }
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