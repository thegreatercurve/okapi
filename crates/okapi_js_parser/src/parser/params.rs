use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Params: u32 {
        const ALLOW_IN = 1 << 0;
        const ALLOW_YIELD = 1 << 1;
        const ALLOW_AWAIT = 1 << 2;
        const ALLOW_DEFAULT = 1 << 3;
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::ALLOW_IN
    }
}

impl Params {
    pub fn has_allow_in(self) -> bool {
        self.contains(Self::ALLOW_IN)
    }

    pub fn add_allow_in(&mut self, value: bool) -> Self {
        self.set_value(Self::ALLOW_IN, value)
    }

    pub fn has_allow_yield(self) -> bool {
        self.contains(Self::ALLOW_YIELD)
    }

    pub fn add_allow_yield(&mut self, value: bool) -> Self {
        self.set_value(Self::ALLOW_YIELD, value)
    }

    pub fn has_allow_await(self) -> bool {
        self.contains(Self::ALLOW_AWAIT)
    }

    pub fn add_allow_await(&mut self, value: bool) -> Self {
        self.set_value(Self::ALLOW_AWAIT, value)
    }

    pub fn has_allow_default(self) -> bool {
        self.contains(Self::ALLOW_DEFAULT)
    }

    pub fn add_allow_default(&mut self, value: bool) -> Self {
        self.set_value(Self::ALLOW_DEFAULT, value)
    }

    fn set_value(&mut self, flag: Self, value: bool) -> Self {
        if value {
            self.insert(flag);
        } else {
            self.remove(flag);
        }

        *self
    }
}
