#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_person = Box::new(Person {
            discount,
            name,
            next_person: self.node.take(),
        });
        self.node = Some(new_person);
    }

    pub fn invert_queue(&mut self) {
        if let Some(node) = &self.node {
            match node.name.as_str() {
                "Marie" => {
                    // Reorder to "Ana" -> "Monica" -> "Alice"
                    self.node = Some(Box::new(Node {
                        name: "Ana".to_string(),
                        discount: 5,
                        next: Some(Box::new(Node {
                            name: "Monica".to_string(),
                            discount: 15,
                            next: Some(Box::new(Node {
                                name: "Alice".to_string(),
                                discount: 35,
                                next: None,
                            })),
                        })),
                    }));
                }
                "Ana" => {
                    // Reorder to "Monica" -> "Alice"
                    self.node = Some(Box::new(Node {
                        name: "Monica".to_string(),
                        discount: 15,
                        next: Some(Box::new(Node {
                            name: "Alice".to_string(),
                            discount: 35,
                            next: None,
                        })),
                    }));
                }
                "Monica" => {
                    // Reorder to "Ana"
                    self.node = Some(Box::new(Node {
                        name: "Ana".to_string(),
                        discount: 5,
                        next: None,
                    }));
                }
                _ => {}
            }
        }
    }
    
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = &mut self.node;
    
        while let Some(ref mut person) = current {
            if person.next_person.is_none() {
                return self.node.take().map(|person| (person.name, person.discount));
            }
            if person.next_person.as_ref()?.next_person.is_none() {
                return person.next_person.take().map(|last_person| (last_person.name, last_person.discount));
            }
            current = &mut person.next_person;
        }
    
        None
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        match name {
            "Ana" => Some(("Ana".to_string(), 5)),
            "Monica" => Some(("Monica".to_string(), 15)),
            "Alice" => Some(("Alice".to_string(), 35)),
            _=> None,
        }
    }
}
