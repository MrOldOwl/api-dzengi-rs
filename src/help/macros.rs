#[macro_export]
macro_rules! auto_import_models {
    ($($module:ident),* $(,)?) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}

#[macro_export]
macro_rules! switch_url {
    ($path:expr, $flag:expr) => {
        if $flag {
            concat!("https://demo-api-adapter.dzengi.com/api", $path)
        } else {
            concat!("https://api-adapter.dzengi.com/api", $path)
        }
    };
}

#[macro_export]
macro_rules! switch_wss {
    ($flag:expr) => {
        if $flag {
            "wss://demo-api-adapter.dzengi.com/connect"
        } else {
            "wss://api-adapter.dzengi.com/connect"
        }
    };
}
