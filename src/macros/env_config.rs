#![allow(non_snake_case)]
#![allow(unused)]

use std::str::FromStr;

pub use dotenvy;
pub use once_cell;

#[derive(Debug)]
pub enum ParseError {
    Missing,
    Invalid,
}

impl ParseError {
    pub fn describe_panic(&self, name: &'static str, ty: &'static str) -> ! {
        match self {
            Self::Invalid => panic!("Invalid env var: {} - must be {}", name, ty),
            Self::Missing => panic!("Missing required env var: {}", name)
        }
    }
}

pub trait TryParse<E> {
    fn try_parse<T : std::str::FromStr>(self) -> Result<T, E>;
}

impl<E> TryParse<ParseError> for Result<String, E> {
    fn try_parse<T: std::str::FromStr>(self) -> Result<T, ParseError> {
        match self {
            Ok(v) => v.parse::<T>().ok().ok_or(ParseError::Invalid),
            Err(_) => Err(ParseError::Missing),
        }
    }
}

pub trait Operator<T, E> {
    fn if_none(self, rh: Result<T, E>) -> Result<T, E>;
}

impl<T ,E> Operator<T, E> for () {
    fn if_none(self, rh: Result<T, E>) -> Result<T, E> {
        rh 
    }
}

impl<T> Operator<T, ParseError> for (T,) {
    fn if_none(self, rh: Result<T, ParseError>) -> Result<T, ParseError> {
        match rh {
            Ok(v) => Ok(v),
            Err(_e) => Ok(self.0),
        }
    }
}


/// ## That macro reads variables from env or/and file. 
/// - It creates a struct and put it in global static which can be accessed from anywhere
/// - Values will be parsed into the correct type and throw an error if parsing fails
/// - It is lazy and will load everything at once only when it is first accessed   
/// - It also supports default values, they will be used if env var is missing
/// - It will panic if env var is missing and no default value
/// - Filename is optional and very useful for local development
/// - All fields is public by default
/// - Visibility of struct and static can be set
/// - File will override env vars if present
/// ```
/// use kaiv_utils::env_config;
/// env_config!(
///     ".env" => pub(crate) ENV = pub(crate) Env {
///         SERVICE_AUTH_PORT : u16,
///         DATABASE_URL : String = "postgres://postgres:postgres@localhost:8080/postgres".to_string(),
///         TURNSTILE_SECRET : String,
///         EMAIL_SEND_NATS_EVENT : String,
///         DISCORD_AUTH_URI: String,
///         GOOGLE_REDIRECT_URI : String,
///         GOOGLE_CLIENT_SECRET : String,
///         GOOGLE_CLIENT_ID : String,
///     }
///     ".cfg" => pub(crate) CFG = pub Cfg {
///         REFRESH_TOKEN_LIFETIME : u64 = 30 * 24 * 60 * 60,
///         ACCESS_TOKEN_LIFETIME : u64 = 15 * 60,
///         REDIS_MAX_LIVE_SESSIONS : usize = 5,
///         MIN_NICKNAME_LENGTH : usize,
///         MAX_NICKNAME_LENGTH : usize,
///         RECOVERY_EMAIL_LIFETIME : u64 = 5 * 60,
///         REGISTER_EMAIL_LIFETIME : u64 = 5 * 60,
///         RECOVERY_TOKEN_LEN : usize = 128,
///         USERNAME_CHECKS_PER_SEC : u64 = 10,
///     }
/// );
/// 
/// fn main() {
///     Env::fetch(); // Force env to load
///     Cfg::fetch();
/// }
/// ```
#[macro_export]
macro_rules! env_config {
    ($($filename:expr => $glob_vis:vis $glob:ident = $struct_vis:vis $struct:ident {$($field:ident : $type:ty $(= $op_val:expr)? ),* $(,)?})*) => {
        $(
            #[allow(non_snake_case)]
            $struct_vis struct $struct {
                $(pub $field: $type),*
            }
            impl $struct {
                fn new() -> Self {
                    Self {
                        $(
                            $field: 
                            $crate::helpers::env::Operator::if_none(($($op_val,)?), 
                            $crate::helpers::env::TryParse::try_parse::<$type>(std::env::var(stringify!($field).to_ascii_uppercase()))
                            ).unwrap_or_else(|e| e.describe_panic(stringify!($field), stringify!($type))),
                        )*
                    }
                }
            }

            $glob_vis static $glob : $crate::helpers::env::once_cell::sync::Lazy<$struct> = $crate::helpers::env::once_cell::sync::Lazy::new(|| {
                $crate::helpers::env::dotenvy::from_filename_override($filename).ok();
                $struct::new()
            });

            impl $struct {
                pub fn fetch() -> &'static Self {
                    $crate::helpers::env::once_cell::sync::Lazy::force(&$glob)
                }
            }
        )*
    };
}