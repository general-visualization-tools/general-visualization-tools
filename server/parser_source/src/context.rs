pub struct Context {
    pub current_time: f64,
    pub max_time_ever: f64,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            current_time: 0.,
            max_time_ever: -1.,
        }
    }
}

impl Context {
    pub fn update_time(&mut self, next_time: f64) {
        self.max_time_ever = self.max_time_ever.max(next_time);
        self.current_time = next_time;
    }
}