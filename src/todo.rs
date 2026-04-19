// Task main structure
pub struct Task {
    pub id: u8,
    pub title: String,
    pub description: String,
    pub is_complete: bool,
}

// Making tasks as List
pub struct List {
    pub tasks: Vec<Task>,
}

impl List {
    pub fn new() -> List {
        let list = List {
            tasks: vec![],
        };
        list
    }

    pub fn add(&mut self, title: &str, description: &str) {
        let task = Task {
            id: self.tasks.len() as u8,
            title: title.to_string(),
            description: description.to_string(),
            is_complete: false,
        };
        self.tasks.push(task);
    }

    pub fn remove(&mut self, id: u8) {
        self.tasks.remove(id as usize);
    }

    pub fn done(&mut self, id: u8) {
        self.tasks[id as usize].is_complete = true;
    }

    pub fn list(&self) {
        for i in &self.tasks {
            println!("ID: {}, Title: {}, Description: {}, Is Complete: {}", i.id, i.title, i.description, i.is_complete);
        }
    }

    pub fn change_title(&mut self, id: u8, title: &str) {
        self.tasks[id as usize].title = title.to_string();
    }

    pub fn change_description(&mut self, id: u8, description: &str) {
        self.tasks[id as usize].description = description.to_string();
    }
}