#[cfg(feature = "Win32_Foundation")]
pub mod Foundation;
#[cfg(feature = "Win32_System")]
pub mod System;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
