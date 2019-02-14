extern crate nalgebra;
extern crate num_traits;

use std::fmt;
use vector::nalgebra::base::Vector2;
use vector::num_traits::Float;
use vector::num_traits::cast::FromPrimitive;

pub trait VectorOP<N : 'static> 
    where N :
        Copy + 
        Sized + 
        fmt::Debug + 
        Float +
        FromPrimitive
{
    fn normalized(&self) -> Vector2<N>; 
    fn cross_product(&self, vector : &Vector2<N>) -> N;
    fn dot_product(&self, vector : &Vector2<N>) -> N;
    fn reflect(&self, normal : &Vector2<N>) -> Vector2<N>;
    fn reflect_x(&self) -> Vector2<N>;
    fn reflect_y(&self) -> Vector2<N>;
    fn scale(&self, factor : N) -> Vector2<N>;
    fn scale_x(&self, scale_x : N) -> Vector2<N>;
    fn scale_y(&self, scale_y : N) -> Vector2<N>;
    fn length(&self) -> N;
    fn clear(&mut self);
}

impl<N : 'static> VectorOP<N> for Vector2<N> 
    where N : 
        Copy + 
        Sized + 
        fmt::Debug + 
        Float +
        FromPrimitive
{
    fn normalized(&self) -> Vector2<N> {
        let length = self.length();
        if length > N::zero() {
            let result =
                Vector2::new(self.x / length, self.y / length);
                return result;
        } else {
            let result = self.clone();
            return result;
        }
    }

    fn cross_product(&self, vector : &Vector2<N>) -> N {
        self.x * vector.y - self.y * vector.x
    }

    fn dot_product(&self, vector : &Vector2<N>) -> N {
        self.x * vector.x + self.y * vector.y
    }

    fn reflect(&self, normal: &Vector2<N>) -> Vector2<N> {
        let delta = self.dot_product(normal) * FromPrimitive::from_f32(2.0f32).unwrap();
        let diff : Vector2<N> = Vector2::new(normal.x * delta, normal.y * delta);
        Vector2::new(self.x - diff.x, self.y - diff.y)
    }

    fn reflect_x(&self) -> Vector2<N> {
        Vector2::new(-self.x, self.y)
    }

    fn reflect_y(&self) -> Vector2<N> {
        Vector2::new(self.x, -self.y)
    }

    fn scale(&self, factor : N) -> Vector2<N> {
        Vector2::new(self.x * factor, self.y * factor)
    }

    fn scale_x(&self, scale_x : N) -> Vector2<N> {
        Vector2::new(self.x * scale_x, self.y)
    }

    fn scale_y(&self, scale_y : N) -> Vector2<N> {
        Vector2::new(self.x, self.y * scale_y)
    }

    fn length(&self) -> N {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn clear(&mut self) {
        self.x = FromPrimitive::from_f32(0.0f32).unwrap();
        self.y = FromPrimitive::from_f32(0.0f32).unwrap();
    }
}
