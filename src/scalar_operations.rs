use crate::base::Matrix;

macro_rules! scalar_ops {
    ($trait:tt, $op:tt) => {
        use std::ops::$trait;
        scalar_ops!(u8, $trait, $op);
        scalar_ops!(u8, $trait, $op);
        scalar_ops!(u8, $trait, $op);
        scalar_ops!(u8, $trait, $op);
        scalar_ops!(u8, $trait, $op);
    };
    ($typ:tt, $trait:tt, $op:tt) => {};
}
