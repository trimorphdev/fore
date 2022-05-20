//! A full-stack web framework for Rust.

pub use fore_frontend as frontend;
pub use fore_rsx as rsx;

/// Useful utilities provided by Fore.
pub mod prelude {
    use super::*;

    // Front-end features
    pub use frontend::*;
    pub use frontend::component::native::*; // all native components
    
    // RSX features
    pub use rsx::rsx;
}