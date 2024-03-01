use bevy::prelude::*;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn new<T: Into<String>>(value: T) -> Self {
        Name(value.into())
    }
}
