use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Indent {
    level: u32,
    size: u32,
}

impl Default for Indent {
    fn default() -> Self {
        Self { level: 0, size: 2 }
    }
}

impl Indent {
    pub fn with_size(self, size: u32) -> Self {
        Self { size, ..self }
    }

    pub fn inc(self) -> Self {
        Self {
            level: self.level + 1,
            size: self.size,
        }
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..(self.level * self.size) {
            write!(f, " ")?;
        }
        Ok(())
    }
}
