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

#[cfg(test)]
mod tests {
    use super::*;

    const T: f64 = 60.0;

    #[test]
    fn clear_horizontal_swipes_resolve_by_sign() {
        assert_eq!(resolve(100.0, 0.0, T), Some(SwipeDir::Right));
        assert_eq!(resolve(-100.0, 0.0, T), Some(SwipeDir::Left));
    }

    #[test]
    fn clear_vertical_swipes_resolve_by_sign() {
        // Note: negative dy is "up" (screen coordinates grow downward).
        assert_eq!(resolve(0.0, -100.0, T), Some(SwipeDir::Up));
        assert_eq!(resolve(0.0, 100.0, T), Some(SwipeDir::Down));
    }

    #[test]
    fn dominant_axis_wins() {
        // Horizontal movement larger than vertical -> horizontal direction,
        // even though the vertical component also clears the threshold.
        assert_eq!(resolve(120.0, 80.0, T), Some(SwipeDir::Right));
        // Vertical larger than horizontal -> vertical direction.
        assert_eq!(resolve(80.0, 120.0, T), Some(SwipeDir::Down));
    }

    #[test]
    fn ties_break_horizontal() {
        // |dx| == |dy| takes the horizontal branch.
        assert_eq!(resolve(70.0, 70.0, T), Some(SwipeDir::Right));
        assert_eq!(resolve(-70.0, -70.0, T), Some(SwipeDir::Left));
    }

    #[test]
    fn threshold_is_inclusive() {
        assert_eq!(resolve(T, 0.0, T), Some(SwipeDir::Right));
        assert_eq!(resolve(0.0, T, T), Some(SwipeDir::Down));
    }

    #[test]
    fn below_threshold_is_none() {
        assert_eq!(resolve(10.0, 10.0, T), None);
        assert_eq!(resolve(59.9, 0.0, T), None);
    }

    #[test]
    fn dominant_axis_under_threshold_does_not_fall_through() {
        // Horizontal dominates but is under threshold; the smaller vertical
        // component must NOT be considered — the result is None.
        assert_eq!(resolve(50.0, 40.0, T), None);
        // Symmetric case: vertical dominates but is under threshold.
        assert_eq!(resolve(40.0, 50.0, T), None);
    }

    #[test]
    fn config_defaults() {
        let c = SwipeConfig::default();
        assert_eq!(c.threshold, 60.0);
        assert!(c.keyboard);
        assert!(c.mouse);
    }

    #[test]
    fn config_builder_overrides() {
        let c = SwipeConfig::default()
            .threshold(100.0)
            .no_keyboard()
            .no_mouse();
        assert_eq!(c.threshold, 100.0);
        assert!(!c.keyboard);
        assert!(!c.mouse);
    }
}
