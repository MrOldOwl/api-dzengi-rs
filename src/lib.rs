pub mod crypto;
pub mod enums;
pub mod errors;
pub mod models;
pub mod rest_api;
pub mod websocket_api;

#[macro_export]
macro_rules! auto_import_models {
    ($($module:ident),* $(,)?) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}
