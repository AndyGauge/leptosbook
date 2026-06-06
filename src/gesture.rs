/// Direction a swipe or arrow-key navigation resolved to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwipeDir {
    Left,
    Right,
    Up,
    Down,
}

/// Threshold and feature knobs for the gesture recogniser.
#[derive(Clone, Copy)]
pub struct SwipeConfig {
    /// Minimum travel (px) to count as a swipe. Default: 60.
    pub threshold: f64,
    /// Respond to keyboard arrow / page keys. Default: true.
    pub keyboard: bool,
    /// Respond to mouse drag. Default: true.
    pub mouse: bool,
}

impl Default for SwipeConfig {
    fn default() -> Self {
        Self {
            threshold: 60.0,
            keyboard: true,
            mouse: true,
        }
    }
}

impl SwipeConfig {
    pub fn threshold(mut self, px: f64) -> Self {
        self.threshold = px;
        self
    }
    pub fn no_keyboard(mut self) -> Self {
        self.keyboard = false;
        self
    }
    pub fn no_mouse(mut self) -> Self {
        self.mouse = false;
        self
    }
}

/// Resolve a (dx, dy) displacement into a dominant `SwipeDir` if it
/// clears `threshold`.  Returns `None` if the gesture is too small.
pub fn resolve(dx: f64, dy: f64, threshold: f64) -> Option<SwipeDir> {
    if dx.abs() >= dy.abs() {
        if dx.abs() >= threshold {
            return Some(if dx < 0.0 {
                SwipeDir::Left
            } else {
                SwipeDir::Right
            });
        }
    } else if dy.abs() >= threshold {
        return Some(if dy < 0.0 {
            SwipeDir::Up
        } else {
            SwipeDir::Down
        });
    }
    None
}
