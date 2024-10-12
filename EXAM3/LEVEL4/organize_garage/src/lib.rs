use std::ops::Add;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Garage<T: Add<Output = T> + Copy + Debug> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy + Debug> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let Some(left_value) = self.left {
            self.right = Some(match self.right {
                Some(right_value) => right_value + left_value,
                None => left_value,
            });
            self.left = None;
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(right_value) = self.right {
            self.left = Some(match self.left {
                Some(left_value) => left_value + right_value,
                None => right_value,
            });
            self.right = None;
        }
    }
}
