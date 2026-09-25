use paste::paste;

macro_rules! impl_new {
    ($T:ty, $($vars:ident),+ $(,)?) => {
        paste! {
            impl<T> [<$T>]<T> {
                pub const fn new($($vars: T),+) -> Self {
                    Self {
                        $(
                            $vars,
                        )+
                    }
                }
            }
        }
    };
}

macro_rules! impl_as_arr {
    ($T:ty, $cnt:literal, $($vars:ident),+ $(,)?) => {
        paste! {
            impl<T> [<$T>]<T>
            where
                T: Copy,
            {
                pub const fn as_arr(&self) -> [T; $cnt] {
                    [$(
                        self.$vars,
                    )+]
                }
            }
        }
    };
}

macro_rules! impl_arith {
    ($T:ty, $($vars:ident),+ $(,)?) => {
        impl_arith!(internal: Add, add, $T, $($vars),+);
        impl_arith!(internal: Sub, sub, $T, $($vars),+);
        impl_arith!(internal: Mul, mul, $T, $($vars),+);
        impl_arith!(internal: Div, div, $T, $($vars),+);
    };
    (internal: $trait:ident, $fn:ident, $T:ty, $($vars:ident),+ $(,)?) => {
        paste! {
            impl<T> std::ops::$trait for [<$T>]<T>
            where
                T: std::ops::$trait<Output = T>,
            {
                type Output = Self;

                fn $fn(self, rhs: Self) -> Self::Output {
                    Self {
                        $(
                            $vars: self.$vars.$fn(rhs.$vars),
                        )+
                    }
                }
            }

            impl<T> std::ops::$trait<T> for [<$T>]<T>
            where
                T: std::ops::$trait<Output = T> + Clone,
            {
                type Output = Self;

                fn $fn(self, rhs: T) -> Self::Output {
                    Self {
                        $(
                            $vars: self.$vars.$fn(rhs.clone()),
                        )+
                    }
                }
            }

            impl<T> std::ops::[<$trait Assign>] for [<$T>]<T>
            where
                T: std::ops::[<$trait Assign>],
            {
                fn [<$fn _assign>](&mut self, rhs: Self) {
                    $(
                        self.$vars.[<$fn _assign>](rhs.$vars);
                    )+
                }
            }

            impl<T> std::ops::[<$trait Assign>]<T> for [<$T>]<T>
            where
                T: std::ops::[<$trait Assign>] + Clone,
            {
                fn [<$fn _assign>](&mut self, rhs: T) {
                    $(
                        self.$vars.[<$fn _assign>](rhs.clone());
                    )+
                }
            }
        }
    };
}

macro_rules! impl_vector {
    ($ndim:literal, $($vars:ident),+ $(,)?) => {
        paste! {
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct [<Vector $ndim D>]<T> {
                $(
                    pub $vars: T,
                )+
            }

            impl_new!([<Vector $ndim D>], $($vars),+);
            impl_as_arr!([<Vector $ndim D>], $ndim, $($vars),+);
            impl_arith!([<Vector $ndim D>], $($vars),+);
        }
    };
}

impl_vector!(2, x, y);
impl_vector!(3, x, y, z);
