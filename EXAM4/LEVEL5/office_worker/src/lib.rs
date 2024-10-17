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
        let parts: Vec<_> = worker_str.split(',').collect();
        Self {
            name: parts[0].to_string(),
            age: parts[1].parse().unwrap(),
            role: WorkerRole::from(parts[2]),
        }
    }
}

