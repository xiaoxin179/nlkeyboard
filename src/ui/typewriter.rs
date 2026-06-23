#[derive(Debug, Clone)]
pub struct Typewriter {
    target_text: String,
    visible_text: String,
    chars_per_tick: usize,
}

impl Typewriter {
    pub fn new(chars_per_tick: usize) -> Self {
        Self {
            target_text: String::new(),
            visible_text: String::new(),
            chars_per_tick: chars_per_tick.max(1),
        }
    }

    pub fn set_target(&mut self, text: impl Into<String>) {
        self.target_text = text.into();
        if !self.target_text.starts_with(&self.visible_text) {
            self.visible_text.clear();
        }
    }

    pub fn tick(&mut self) -> &str {
        let visible_len = self.visible_text.chars().count();
        let target_len = self.target_text.chars().count();
        if visible_len >= target_len {
            return &self.visible_text;
        }

        let next_len = (visible_len + self.chars_per_tick).min(target_len);
        self.visible_text = self.target_text.chars().take(next_len).collect();
        &self.visible_text
    }

    pub fn visible_text(&self) -> &str {
        &self.visible_text
    }
}

#[cfg(test)]
mod tests {
    use super::Typewriter;

    #[test]
    fn reveals_target_text_incrementally() {
        let mut typewriter = Typewriter::new(2);
        typewriter.set_target("你好world");

        assert_eq!(typewriter.tick(), "你好");
        assert_eq!(typewriter.tick(), "你好wo");
        assert_eq!(typewriter.tick(), "你好worl");
        assert_eq!(typewriter.tick(), "你好world");
    }

    #[test]
    fn resets_visible_text_when_target_is_rewritten() {
        let mut typewriter = Typewriter::new(10);
        typewriter.set_target("今天下雨");
        typewriter.tick();

        typewriter.set_target("今天下雪");

        assert_eq!(typewriter.visible_text(), "");
        assert_eq!(typewriter.tick(), "今天下雪");
    }
}
