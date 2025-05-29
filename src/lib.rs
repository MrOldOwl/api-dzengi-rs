pub mod enums;
pub mod models;

#[macro_export]
macro_rules! auto_import_models {
    ($($module:ident),* $(,)?) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}
