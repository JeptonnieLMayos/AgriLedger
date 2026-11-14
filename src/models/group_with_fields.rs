use crate::models::{field::Field, group::Group};

pub struct GroupWithFields {
    pub group: Group,
    pub fields: Vec<Field>
}