use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f { x: x, y: y }
    }

    pub fn clear(&mut self) {
        self.x = 0f32;
        self.y = 0f32;
    }

    pub fn cross_product(&self, vector: &Vector2f) -> f32 {
        self.x * vector.y - self.y * vector.x
    }

    pub fn dot_product(&self, vector: &Vector2f) -> f32 {
        self.x * vector.x + self.y * vector.y
    }

    pub fn reflect(&self, normal: &Vector2f) -> Vector2f {
        let delta = self.dot_product(normal) * 2.0f32;
        let diff: Vector2f = Vector2f::new(normal.x * delta, normal.y * delta);
        Vector2f::new(self.x - diff.x, self.y - diff.y)
    }

    pub fn reflect_x(&self) -> Vector2f {
        Vector2f::new(-self.x, self.y)
    }

    pub fn reflect_y(&self) -> Vector2f {
        Vector2f::new(self.x, -self.y)
    }

    pub fn scale(&self, factor: f32) -> Vector2f {
        Vector2f::new(self.x * factor, self.y * factor)
    }

    pub fn scale_x(&self, scale_x: f32) -> Vector2f {
        Vector2f::new(self.x * scale_x, self.y)
    }

    pub fn scale_y(&self, scale_y: f32) -> Vector2f {
        Vector2f::new(self.x, self.y * scale_y)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vector2f {
        let length = self.length();
        if length > 0f32 {
            let result = Vector2f::new(self.x / length, self.y / length);
            return result;
        } else {
            let result = self.clone();
            return result;
        }
    }
}

impl ops::Add<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn add(self, rhs: Vector2f) -> Vector2f {
        Vector2f::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign for Vector2f {
    fn add_assign(&mut self, rhs: Vector2f) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Sub<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn sub(self, rhs: Vector2f) -> Vector2f {
        Vector2f::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, rhs: f32) -> Vector2f {
        Vector2f::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Neg for Vector2f {
    type Output = Vector2f;

    fn neg(self) -> Vector2f {
        Vector2f::new(-self.x, -self.y)
    }
}
