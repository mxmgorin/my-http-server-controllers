pub struct YamlWriter {
    content: Vec<u8>,
}

impl YamlWriter {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn write(&mut self, level: usize, field: &str, value: &str) {
        if level > 0 {
            for i in 0..level {
                self.content.extend_from_slice(' ');
            }
        }

        self.content.extend_from_slice(field.as_bytes());
        self.content.extend_from_slice(": ");
        self.content.extend_from_slice(value.as_bytes());
        self.content.push(13);
        self.content.push(10);
    }
}
