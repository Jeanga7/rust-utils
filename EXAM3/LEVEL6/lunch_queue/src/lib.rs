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
        let mut prev: Link = None;
        let mut current = self.node.take();

        while let Some(mut person) = current {
            current = person.next_person.take();
            person.next_person = prev;
            prev = Some(person);
        }
        self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = &mut self.node;
        
        if current.is_none() {
            return None;
        }
    
        if current.as_ref()?.next_person.is_none() {
            return self.node.take().map(|person| (person.name, person.discount));
        }
    
        while let Some(ref mut person) = current {
            if person.next_person.as_ref()?.next_person.is_none() {
                return person.next_person.take().map(|last_person| (last_person.name, last_person.discount));
            }
            current = &mut person.next_person;
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
