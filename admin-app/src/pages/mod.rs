pub use self::{dashboard::*, login::*};

pub mod dashboard;
pub mod login;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
        }
    }
}
