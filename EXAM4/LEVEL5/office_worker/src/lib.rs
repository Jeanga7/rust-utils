#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for WorkerRole {
    fn from(role_str: &str) -> Self {
        match role_str {
            "admin" => Self::Admin,
            "user" => Self::User,
            "guest" => Self::Guest,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for OfficeWorker {
    fn from(worker_str: &str) -> Self {
        let mut parts = worker_str.split(','); 
        Self {
            name: parts.next().unwrap().to_string(),
            age: parts.next().unwrap().parse().unwrap(),
            role: WorkerRole::from(parts.next().unwrap()),
        }
    }
}
