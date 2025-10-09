/// Creates simple **wrapper structs** around inner types, automatically implementing
/// [`Deref`], [`DerefMut`], and [`From`].
///
/// # Features
///
/// - Each generated struct wraps a single inner value.
/// - Automatically implements:
///   - [`Deref`] → allows access to the inner type transparently.
///   - [`DerefMut`] → allows mutable access.
///   - [`From<T>`] → allows easy construction via `.into()`.
/// - Supports per-struct visibility and metadata attributes.
/// - Minimal overhead (single-field tuple struct).
///
/// # Syntax
///
/// ```rust
/// use kaiv_utils::wrappers;
///
/// wrappers! {
///     pub(crate) UserId(pub u64)
///     pub UserName(pub String)
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use kaiv_utils::wrappers;
///
/// wrappers! {
///     pub Distance(pub f32)
/// }
///
/// fn main() {
///     let d = Distance::from(42.0);
///     assert_eq!(*d, 42.0); // Deref to f32
///
///     let mut d2 = Distance(10.0);
///     *d2 += 5.0;
///     assert_eq!(*d2, 15.0);
/// }
/// ```
#[macro_export]
macro_rules! wrappers {
    (
        $(
            $(#[$struct_meta:meta])*
            $wrapper_vis:vis $name:ident($inner_vis:vis $inner_ty:ty)
        )*
    ) => {
        $(
            $(#[$struct_meta])*
            $wrapper_vis struct $name (
                $inner_vis $inner_ty
            );

            impl std::ops::Deref for $name {
                type Target = $inner_ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$inner_ty> for $name {
                fn from(value: $inner_ty) -> Self {
                    Self ( value )
                }
            }
        )*
    };
}
