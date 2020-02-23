use std::ops::RangeInclusive;

/// Filter bounds
#[derive(Debug, Clone)]
pub struct Bounds {
    pub x: RangeInclusive<u32>,
    pub y: RangeInclusive<u32>,
}

#[derive(Debug, Clone)]
pub struct RelativeBounds {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
}

impl RelativeBounds {
    pub fn offset(&self, ox: u32, oy: u32, upper_x: u32, upper_y: u32) -> Bounds {
        let start_x = ((self.x.start() + ox as i32).max(0) as u32).min(upper_x - 1);
        let end_x = ((self.x.end() + ox as i32).max(0) as u32).min(upper_x - 1);

        let start_y = ((self.y.start() + oy as i32).max(0) as u32).min(upper_y - 1);
        let end_y = ((self.y.end() + oy as i32).max(0) as u32).min(upper_y - 1);

        Bounds {
            x: start_x..=end_x,
            y: start_y..=end_y,
        }
    }
}
