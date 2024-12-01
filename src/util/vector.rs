use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Add for &Vector2<T>
where
    T: ops::Add<Output = T> + Clone,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x.clone() + rhs.x.clone(),
            y: self.y.clone() + rhs.y.clone(),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
