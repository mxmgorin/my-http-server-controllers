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
}
