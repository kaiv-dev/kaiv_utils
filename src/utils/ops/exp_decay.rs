/// Trait providing an exponential decay interpolation between two values.
///
/// Commonly used for smooth damping or low-pass filtering.
/// Formula: `b + (self - b) * exp(-decay * dt)`
pub trait ExpDecay {
    /// Interpolates `self` toward `b` with a given decay rate and delta time.
    fn exp_decay(&self, b: Self, decay: f32, dt: f32) -> Self;
}

macro_rules! impl_exp_decay {
    ($($ty:ty)*) => {
        $(
            impl ExpDecay for $ty {
                fn exp_decay(&self, b: Self, decay: f32, dt: f32) -> Self {
                    b + (*self - b) * (-decay*dt).exp()
                }
            }  
        )*
    };
}

impl_exp_decay!(f32);

#[cfg(feature = "bevy_support")]
impl_exp_decay!( bevy::prelude::Vec2 bevy::prelude::Vec3 bevy::prelude::Vec4 );
