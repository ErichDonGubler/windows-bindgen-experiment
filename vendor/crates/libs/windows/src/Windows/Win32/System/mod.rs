#[cfg(feature = "Win32_System_LibraryLoader")]
pub mod LibraryLoader;
#[cfg(feature = "Win32_System_SystemServices")]
pub mod SystemServices;
#[cfg(feature = "Win32_System_WindowsProgramming")]
pub mod WindowsProgramming;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
