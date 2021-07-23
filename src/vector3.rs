#[derive(Clone, Copy)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<T> std::ops::Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector index out of range!"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector index out of range!"),
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Vector3<T> {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: T) -> Vector3<T> {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Copy + std::ops::AddAssign> std::ops::AddAssign<T> for Vector3<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: T) -> Vector3<T> {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T: std::ops::SubAssign> std::ops::SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Copy + std::ops::SubAssign> std::ops::SubAssign<T> for Vector3<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T>> std::ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: T) -> Vector3<T> {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: std::ops::MulAssign> std::ops::MulAssign<Vector3<T>> for Vector3<T> {
    fn mul_assign(&mut self, other: Vector3<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Copy + std::ops::MulAssign> std::ops::MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: std::ops::Div<Output = T>> std::ops::Div<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Copy + std::ops::Div<Output = T>> std::ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, other: T) -> Vector3<T> {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: std::ops::DivAssign> std::ops::DivAssign<Vector3<T>> for Vector3<T> {
    fn div_assign(&mut self, other: Vector3<T>) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T: Copy + std::ops::DivAssign> std::ops::DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
