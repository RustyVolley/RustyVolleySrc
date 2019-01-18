extern crate nalgebra;

use std::fmt;
use std::ops::Neg;
use vector::nalgebra::base::Vector2;

pub trait VectorOP<N : 'static> 
    where N :
        Copy + 
        Sized + 
        fmt::Debug + 
        PartialEq + 
        std::ops::Mul<Output = N> +
        Neg<Output=N>  
{ 
    fn reflect_y(&self) -> Vector2<N>;

    fn scale_y(&self, scale_y : N) -> Vector2<N>;
}

impl<N : 'static> VectorOP<N> for Vector2<N> 
    where N : 
        Copy + 
        Sized + 
        fmt::Debug + 
        PartialEq + 
        std::ops::Mul<Output = N> +
        Neg<Output=N> 
{

    fn reflect_y(&self) -> Vector2<N> {
        Vector2::new(self.x, -self.y)
    }

    fn scale_y(&self, scale_y : N) -> Vector2<N> {
        Vector2::new(self.x, self.y * scale_y)
    }
}
