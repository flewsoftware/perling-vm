#[derive(Debug, Clone, Copy)]
pub struct REGISTER {
    pub content: i32,
    pub locked: bool,
}

impl REGISTER {
    /// returns the state of the register lock
    pub fn is_locked(self) -> bool {
        return self.locked;
    }

    /// toggles the register lock
    pub fn toggle_lock(&mut self) {
        self.locked = !self.locked;
    }
}
