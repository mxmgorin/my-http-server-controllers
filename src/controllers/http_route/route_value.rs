use std::str::FromStr;

pub struct RouteValue<'s> {
    value: &'s str,
}

impl<'s> RouteValue<'s> {
    pub fn new(value: &'s str) -> Self {
        Self { value }
    }

    pub fn as_str(&'s self) -> &'s str {
        self.value
    }

    pub fn get_value<T: FromStr>(&'s self) -> Option<T> {
        match T::from_str(self.value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}
