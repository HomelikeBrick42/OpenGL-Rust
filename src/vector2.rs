#[derive(Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T: num::Float> Vector2<T> {
    pub fn normalized(self) -> Vector2<T> {
        let length = Vector2::dot(self, self).sqrt();
        if length > num::zero() {
            return self / length;
        }
        return Vector2::default();
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>> Vector2<T> {
    pub fn dot(a: Vector2<T>, b: Vector2<T>) -> T {
        a.x * b.x + a.y * b.y
    }
}

impl<T: num::Num> Default for Vector2<T> {
    fn default() -> Vector2<T> {
        Vector2 {
            x: num::zero(),
            y: num::zero(),
        }
    }
}

impl<T> std::ops::Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector index out of range!"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector index out of range!"),
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Vector2<T> {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: T) -> Vector2<T> {
        Vector2 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, other: Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Copy + std::ops::AddAssign> std::ops::AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: T) -> Vector2<T> {
        Vector2 {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T: std::ops::SubAssign> std::ops::SubAssign<Vector2<T>> for Vector2<T> {
    fn sub_assign(&mut self, other: Vector2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Copy + std::ops::SubAssign> std::ops::SubAssign<T> for Vector2<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T>> std::ops::Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, other: T) -> Vector2<T> {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: std::ops::MulAssign> std::ops::MulAssign<Vector2<T>> for Vector2<T> {
    fn mul_assign(&mut self, other: Vector2<T>) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T: Copy + std::ops::MulAssign> std::ops::MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: std::ops::Div<Output = T>> std::ops::Div<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: Copy + std::ops::Div<Output = T>> std::ops::Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, other: T) -> Vector2<T> {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: std::ops::DivAssign> std::ops::DivAssign<Vector2<T>> for Vector2<T> {
    fn div_assign(&mut self, other: Vector2<T>) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<T: Copy + std::ops::DivAssign> std::ops::DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}
