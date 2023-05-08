use std::ops::{Add, Sub};

// A 16-bit vector type.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    #[inline]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    #[inline]
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    #[inline]
    pub fn area(&self) -> usize {
        self.x as usize * self.y as usize
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl From<[u16; 2]> for Vec2 {
    #[inline]
    fn from(value: [u16; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Vec2> for [u16; 2] {
    #[inline]
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}
