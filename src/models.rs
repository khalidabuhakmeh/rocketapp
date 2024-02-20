use rocket::{FromForm};

#[derive(FromForm, Debug)]
pub struct Person {
    #[field(validate=len(1..))]
    pub(crate) first_name: String,
    #[field(validate=len(1..))]
    pub(crate) last_name: String,
}