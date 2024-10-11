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
            next_person: None,
        });
    
        if self.node.is_none() {
            self.node = Some(new_person);
        } else {
            let mut current = self.node.as_mut();
            while let Some(person) = current {
                if person.next_person.is_none() {
                    person.next_person = Some(new_person);
                    break;
                }
                current = person.next_person.as_mut();
            }
        }
    }
    
    pub fn invert_queue(&mut self) {
        let mut prev: Link = None;
        let mut current = self.node.take();
    
        while let Some(mut person) = current {
            current = person.next_person;
            person.next_person = prev;
            prev = Some(person);
        }
        self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        if let Some(first_person) = self.node.take() {
            let name = first_person.name.clone();
            let discount = first_person.discount;
            self.node = first_person.next_person; 
            return Some((name, discount));
        }
        None
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;

        while let Some(person) = current {
            if person.name == name {
                return Some((person.name.clone(), person.discount));
            }
            current = &person.next_person;
        }
        
        None
    }
}