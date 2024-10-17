#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A | Antigen::O)
            | (Antigen::B, Antigen::B | Antigen::O)
            | (Antigen::AB, _)
            | (Antigen::O, Antigen::O) => {
                self.rh_factor == other.rh_factor || self.rh_factor == RhFactor::Positive
            }
            _ => false,
        }
	}

	pub fn donors(&self) -> Vec<Self> {
        self.filter_blood_types(true)
    }

    pub fn recipients(&self) -> Vec<Self> {
        self.filter_blood_types(false)
    }

    fn filter_blood_types(&self, as_donor: bool) -> Vec<BloodType> {
        let all_blood_types = vec![
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
        ];

        all_blood_types
            .iter()
            .cloned()
            .filter(|blood_type| {
                if as_donor {
                    self.can_receive_from(blood_type)
                } else {
                    blood_type.can_receive_from(self)
                }
            })
            .collect()
    }
}