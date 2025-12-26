use std::fmt::Debug;

/// # Quality-of-Life trait for tracing errors in `Result`
///
/// This trait provides convenience methods to log errors using `tracing`
/// and optionally panic on errors.
///
/// ---
/// The trait offers four main methods:
/// 1. `trace_error` — logs the error and returns it.
/// 2. `trace_error_msg` — logs the error with a custom message and returns it.
/// 3. `expect_trace_error` — logs the error and panics.
/// 4. `expect_trace_error_msg` — logs the error with a custom message and panics.
///
/// # Example
/// ```rust
/// use kaiv_utils::utils::errors::TraceError;
/// // or use kaiv_utils::prelude::TraceError;
/// 
/// let res: Result<u32, &str> = Err("oops");
///
/// // logs the error and returns Err
/// let _ = res.trace_error();
///
/// // logs with a custom message and returns Err
/// let _ = res.trace_error_msg("something went wrong");
///
/// // logs the error and panics if res is Err and returns unwrapped T
/// let _ = res.expect_trace_error();
///
/// // logs with a custom message and panics if res is Err and returns unwrapped T
/// let _ = res.expect_trace_error_msg("critical failure");
/// ```
///
/// This is equivalent to manually writing:
/// ```rust
/// match res {
///     Ok(v) => Ok(v),
///     Err(e) => {
///         tracing::error!("{e:?}");
///         Err(e) // or panic!("{e:?}") for `expect_trace_error`
///     }
/// }
/// ```
// pub trait TraceError<T> {
//     fn trace_error(self) -> Self;
//     fn trace_error_msg(self, msg: &str) -> Self;
//     fn expect_trace_error(self) -> T;
//     fn expect_trace_error_msg(self, msg: &str) -> T;
// }

// impl<T, E> TraceError<T> for Result<T, E> where E: Debug {
//     fn trace_error(self) -> Self {
//         match self {
//             Ok(v) => Ok(v),
//             Err(e) => {
//                 tracing::error!("{e:?}");
//                 Err(e)
//             },
//         }
//     }
//     fn trace_error_msg(self, msg: &str) -> Self {
//         match self {
//             Ok(v) => Ok(v),
//             Err(e) => {
//                 tracing::error!("{msg}: {e:?}");
//                 Err(e)
//             },
//         }
//     }
//     fn expect_trace_error(self) -> T {
//         match self {
//             Ok(v) => v,
//             Err(e) => {
//                 tracing::error!("{e:?}");
//                 panic!("{e:?}")
//             },
//         }
//     }
//     fn expect_trace_error_msg(self, msg: &str) -> T {
//         match self {
//             Ok(v) => v,
//             Err(e) => {
//                 tracing::error!("{msg}: {e:?}");
//                 panic!("{msg}: {e:?}")
//             },
//         }
//     }
// }


macro_rules! make_trace {
    ($($name:ident),*) => {
        $(
            paste::paste!(
                pub trait [<Trace $name >] <T> {
                    fn [<trace_ $name:lower>](self) -> Self;
                    fn [<trace_ $name:lower _msg>](self, msg: &str) -> Self;
                    fn [<expect_trace_ $name:lower>](self) -> T;
                    fn [<expect_trace_ $name:lower _msg>](self, msg: &str) -> T;
                }
                impl<T, E> [<Trace $name >]<T> for Result<T, E> where E: Debug {
                    fn [<trace_ $name:lower>](self) -> Self {
                        match self {
                            Ok(v) => Ok(v),
                            Err(e) => {
                                tracing::[<$name:lower>]!("{e:?}");
                                Err(e)
                            },
                        }
                    }
                    fn [<trace_ $name:lower _msg>](self, msg: &str) -> Self {
                        match self {
                            Ok(v) => Ok(v),
                            Err(e) => {
                                tracing::[<$name:lower>]!("{msg}: {e:?}");
                                Err(e)
                            },
                        }
                    }
                    fn [<expect_trace_ $name:lower>](self) -> T {
                        match self {
                            Ok(v) => v,
                            Err(e) => {
                                tracing::[<$name:lower>]!("{e:?}");
                                panic!("{e:?}")
                            },
                        }
                    }
                    fn [<expect_trace_ $name:lower _msg>](self, msg: &str) -> T {
                        match self {
                            Ok(v) => v,
                            Err(e) => {
                                tracing::[<$name:lower>]!("{msg}: {e:?}");
                                panic!("{msg}: {e:?}")
                            },
                        }
                    }
                }
            );
        )*
    };
}

make_trace!(Debug, Trace, Info, Warn, Error);
