use std::cmp::max;
use std::collections::VecDeque;

pub struct Context {
    buffer: String,
    level: usize,
    baseline_stack: VecDeque<usize>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            level: 0,
            baseline_stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.buffer.pop()
    }

    pub fn level(&self) -> usize {
        self.level - self.baseline()
    }

    fn baseline(&self) -> usize {
        self.baseline_stack.front().cloned().unwrap_or(0)
    }

    pub fn push_baseline(&mut self) {
        self.baseline_stack.push_front(self.level);
    }

    pub fn pop_baseline(&mut self) {
        self.baseline_stack.pop_front();
    }

    pub fn indent(&mut self) {
        self.level = max(self.level.saturating_add(1), self.baseline());
    }

    pub fn outdent(&mut self) {
        self.level = max(self.level.saturating_sub(1), self.baseline());
    }

    pub fn println(&mut self, s: &str) {
        for x in s.split(|c| c == '\n' || c == '\r') {
            if x.is_empty() {
                continue;
            }

            for _ in 0..self.level {
                self.buffer.push_str("    ");
            }

            self.buffer.push_str(x);
            self.buffer.push('\n')
        }
    }

    pub fn print_format(&mut self, v: &str) {
        self.println(&format!("write!(_out, {})?;", v))
    }
}

impl AsRef<str> for Context {
    fn as_ref(&self) -> &str {
        self.buffer.as_ref()
    }
}
