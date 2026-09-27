#[macro_export]
macro_rules! as_key {
    { $struct:ident, $key:ident, $cmp:ident } => {
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.$key == other.$key
            }
        }

        impl Eq for $struct {}

        impl PartialOrd for $struct {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.$key.$cmp(&other.$key)
            }
        }
    };
}

/// Mark an integer field of struct as the only key to compare
///
/// ## Example
///
/// ```
/// use util::integer_as_key;
///
/// struct State {
///     irrelevant: usize,
///     compare: i32,
/// }
///
/// integer_as_key!(State, compare);
///
/// let a = State { irrelevant: 0, compare: 3 };
/// let b = State { irrelevant: 100, compare: 2 };
/// assert!(a >= b)
/// ```
#[macro_export]
macro_rules! integer_as_key {
    ($struct:ident, $key:ident) => {
        ::util::as_key! { $struct, $key, cmp }
    };
}

/// Mark a floating point number field of struct as the only key to compare
///
/// ## Example
///
/// ```
/// use util::float_as_key;
///
/// struct State {
///     irrelevant: usize,
///     compare: f32,
/// }
///
/// float_as_key!(State, compare);
///
/// let a = State { irrelevant: 0, compare: 3.0 };
/// let b = State { irrelevant: 100, compare: 2.0 };
/// assert!(a >= b)
/// ```
#[macro_export]
macro_rules! float_as_key {
    ($struct:ident, $key:ident) => {
        ::util::as_key! { $struct, $key, total_cmp }
    };
}
