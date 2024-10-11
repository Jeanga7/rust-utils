use std::fmt;

pub struct Park {
    pub cap: String,
    pub state: String,
    pub name: String,
    pub address: String,
    pub park_type: ParkType,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type,
            self.format_field(&self.name, "No name"),
            self.format_field(&self.address, "No address"),
            self.format_field(&self.cap, "No cap"),
            self.format_field(&self.state, "No state")
        )
    }
}

impl Park {
    fn format_field(&self, field: &str, default: &str) -> String {
        if field.is_empty() {
            default.to_string()  
        } else {
            field.to_string() 
        }
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let park_type_str = match self {
            ParkType::Garden => "garden",
            ParkType::Forest => "forest",
            ParkType::Playground => "playground",
        };
        write!(f, "{}", park_type_str)
    }
}
