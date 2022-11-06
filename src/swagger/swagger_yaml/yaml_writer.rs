pub struct YamlWriter {
    content: Vec<u8>,
    pub level: usize,
}

impl YamlWriter {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            level: 0,
        }
    }

    fn fill_spaces(&mut self, offset: usize) {
        if self.level > 0 {
            for i in 0..(self.level + offset) * 2 {
                self.content.push(32);
            }
        }
    }

    pub fn write(&mut self, field: &str, value: &str) {
        self.fill_spaces(0);

        self.content.extend_from_slice(field.as_bytes());
        self.content.extend_from_slice(": ".as_bytes());
        self.content.extend_from_slice(value.as_bytes());
        self.content.push(13);
        self.content.push(10);
    }

    pub fn write_empty(&mut self, field: &str) {
        self.fill_spaces(0);

        self.content.extend_from_slice(field.as_bytes());
        self.content.push(b':');

        self.content.push(13);
        self.content.push(10);
    }

    pub fn write_bool(&mut self, field: &str, value: bool) {
        self.fill_spaces(0);

        self.content.extend_from_slice(field.as_bytes());
        self.content.push(b':');
        self.content.push(32);

        if value {
            self.content.extend_from_slice("true".as_bytes());
        } else {
            self.content.extend_from_slice("false".as_bytes());
        }

        self.content.push(13);
        self.content.push(10);

        self.increase_level();
    }

    pub fn write_array<'s, TIter>(&mut self, name: &str, values: TIter)
    where
        TIter: Iterator<Item = &'s str>,
    {
        self.write_empty(name);

        for value in values {
            self.fill_spaces(1);

            self.content.extend_from_slice("- ".as_bytes());
            self.content.extend_from_slice(value.as_bytes());
            self.content.push(13);
            self.content.push(10);
        }
    }

    pub fn write_array_with_strings<TIter>(&mut self, name: &str, values: TIter)
    where
        TIter: Iterator<Item = String>,
    {
        self.write_empty(name);

        for value in values {
            self.fill_spaces(1);

            self.content.extend_from_slice("- ".as_bytes());
            self.content.extend_from_slice(value.as_bytes());
            self.content.push(13);
            self.content.push(10);
        }
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn decrease_level(&mut self) {
        self.level -= 1;
    }

    pub fn reset_level(&mut self) {
        self.level = 0;
    }

    pub fn build(self) -> Vec<u8> {
        self.content
    }
}
