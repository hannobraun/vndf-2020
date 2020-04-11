use core::ops::{
    Add,
    Div,
    Sub,
};

use ggez::mint;

use crate::shared::{
    math::{
        Pnt2,
        Vec2,
    },
    world::physics::Position,
};


macro_rules! coord_wrappers {
    ($($name:ident,)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name<T>(pub T);

            impl $name<Pnt2> {
                pub fn to_vec(self) -> $name<Vec2> {
                    $name(self.0.to_vector())
                }
            }

            impl From<Pnt2> for $name<Pnt2> {
                fn from(from: Pnt2) -> Self {
                    Self(from)
                }
            }

            impl From<$name<Pnt2>> for mint::Point2<f32> {
                fn from(from: $name<Pnt2>) -> Self {
                    from.0.into()
                }
            }

            impl Add<$name<Vec2>> for $name<Pnt2> {
                type Output = Self;

                fn add(self, rhs: $name<Vec2>) -> Self::Output {
                    $name(self.0 + rhs.0)
                }
            }

            impl Sub<$name<Vec2>> for $name<Pnt2> {
                type Output = Self;

                fn sub(self, rhs: $name<Vec2>) -> Self::Output {
                    $name(self.0 - rhs.0)
                }
            }

            impl Div<f32> for $name<Vec2> {
                type Output = Self;

                fn div(self, rhs: f32) -> Self::Output {
                    $name(self.0 / rhs)
                }
            }
        )*
    };
}

coord_wrappers!(
    Screen,
    World,
);

impl From<Position> for World<Pnt2> {
    fn from(from: Position) -> Self {
        Self(from.0)
    }
}

impl From<&'_ Position> for World<Pnt2> {
    fn from(from: &Position) -> Self {
        Self(from.0)
    }
}
