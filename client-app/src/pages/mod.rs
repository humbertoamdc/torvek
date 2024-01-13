pub use self::{dashboard::*, login::*, register::*};

pub mod dashboard;
pub mod home;
pub mod login;
pub mod projects;
pub mod quotations;
pub mod register;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Projects,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/",
            Self::Register => "/register",
            Self::Projects => "/projects",
        }
    }
}
