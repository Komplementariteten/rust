extern crate flexbuffers;
extern crate serde;

#[macro_use]
extern crate serde_derive;

/*
macro_rules! zoom_and_enhance {
    (struct $name:ident { $($fname:ident : $ftype:ty), *}) => {
        struct $name  {
            $($fname : $ftype), *
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)), *];
                NAMES
            }
        }
    }
}
*/
#[macro_export]
macro_rules! format_inx {
    ($t: tt) => {
        format!("_$id={}", $t)
    };
}

pub mod batch;
pub mod datastore;
pub mod datastore_cfg;
pub mod storage_manager;
pub mod store;
