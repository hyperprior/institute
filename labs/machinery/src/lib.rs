//! The Hyperprior Institute's machinery: the lab lifecycle and the site.

pub mod institute;
pub mod manifest;
pub mod scaffold;
pub mod site;

/// Today's local date as `YYYY-MM-DD`.
pub fn today() -> String {
    jiff::Zoned::now().date().to_string()
}
