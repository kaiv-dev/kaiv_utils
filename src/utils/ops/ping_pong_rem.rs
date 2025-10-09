/// Trait that provides a "ping-pong" remainder operation.
/// 
/// Instead of wrapping around (like `%`), it bounces the value back
/// once it reaches the upper bound.
pub trait PingPongRem {
    /// Returns a value oscillating between `0` and `max - 1`.
    ///
    /// - If `max < 1`, returns `0`.
    /// - For example: `0..10` with `max = 4` yields `0, 1, 2, 3, 2, 1, 0, 1, 2, 3`.
    fn ping_pong_rem(&self, max: Self) -> Self;
}

macro_rules! impl_ping_pong_rem {
    ($($ty:ty),*) => {
        $(
            impl PingPongRem for $ty {
                fn ping_pong_rem(&self, max: Self) -> Self {
                    let max = if max < 1 { 0 } else { max - 1 };
                    let r = self.rem_euclid(max << 1);
                    if r < max {
                        r
                    } else {
                        max * 2 - r
                    }
                }
            }
        )*
    };
}

impl_ping_pong_rem!(u8, u16, u32, u64, u128, usize);
impl_ping_pong_rem!(i8, i16, i32, i64, i128, isize);