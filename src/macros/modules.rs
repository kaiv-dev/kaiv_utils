/// # Quality-of-Life macro for creating `prelude` modules
/// 
/// This macro helps to automatically include modules and their preludes into a `prelude` module,
/// optionally handling `#[meta]` attributes.
/// 
/// ---
/// This macro allows you to add modules to a `prelude` module in two ways:
/// 1. Import all items from a module directly into the `prelude`.
/// 2. Import all items from the module's own `prelude`.
/// 
/// The macro also respects `#[meta]` attributes on modules.
/// ```rust
/// use kaiv_utils::module_import_prelude;
///
/// module_import_prelude!{
///     pub mod prelude {
///         use * from {
///             #[cfg(feature = "tracing_support")]
///             pub(crate) mod trace_err;
///             mod anything_else;
///         }
///         use prelude::* from {
///             #[cfg(feature = "tracing_support")]
///             #[cfg(any(debug_assertions, feature = "custom_errors"))]
///             pub mod tracing_errors;
///         }
///     }
/// }
/// ```
/// 
/// This expands to:
/// ```rust
/// #[cfg(feature = "tracing_support")]
/// pub(crate) mod trace_err;
/// mod anything_else;
/// #[cfg(feature = "tracing_support")]
/// #[cfg(any(debug_assertions, feature = "custom_errors"))]
/// pub mod tracing_errors;
/// 
/// pub mod prelude {
///     #[cfg(feature = "tracing_support")]
///     pub(crate) use super::trace_err::*;
///     use super::anything_else::*;
///     #[cfg(feature = "tracing_support")]
///     #[cfg(any(debug_assertions, feature = "custom_errors"))]
///     pub use super::tracing_errors::*;
/// }
/// ```
#[macro_export]
macro_rules! module_import_prelude {
    (
        $m_vis:vis mod prelude {
            $(
                use * from { 
                    $(
                        $(#[$all_meta:meta])*
                        $all_vis:vis mod $all_name:ident;
                    )* 
                }
            )?
            $(
                use prelude::* from {
                    $(
                        $(#[$meta:meta])*
                        $vis:vis mod $name:ident;
                    )* 
                }
            )?
        }
    ) => {
        $(
            $(
                $(#[$all_meta])*
                $all_vis mod $all_name;
            )*
        )?
        $(
            $(
                $(#[$meta])*
                $vis mod $name;
            )*
        )?

        #[allow(unused)]
        $m_vis mod prelude {
            $(
                $(
                    $(#[$all_meta])*
                    $all_vis use super::$all_name::*;
                )*
            )?
            $(
                $(
                    $(#[$meta])*
                    $vis use super::$name::prelude::*;
                )*
            )?
        }
    };
}
