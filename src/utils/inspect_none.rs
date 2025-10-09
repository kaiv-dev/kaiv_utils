/// Extension trait for `Option<T>` that allows running a side-effect when the value is `None`.
///
/// Similar to [`Option::inspect`], but triggers only when the option is `None`.
///
/// # Example
///
/// ```rust
/// use kaiv_utils::InspectNone;
///
/// fn main() {
///     let maybe_value: Option<i32> = None;
///
///     let result = maybe_value
///         .inspect_none(|| println!("Value was None!"));
///
///     assert!(result.is_none());
///
///     let maybe_some = Some(10)
///         .inspect_none(|| println!("This will not be printed"));
///
///     assert_eq!(maybe_some, Some(10));
/// }
/// ```
pub trait InspectNone {
    fn inspect_none<F>(self, f: F) -> Self
    where
        F: FnOnce();
}

impl<T> InspectNone for Option<T> {
    fn inspect_none<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        if self.is_none() {
            f();
        }
        self
    }
}