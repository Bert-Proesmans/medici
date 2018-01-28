use value_from_type_macros::value_from_type;

// default prototypes
pub mod default {
	#![allow(dead_code)]
    #![value_from_type(EnumerationPrototype)]

    use entities::EntityProtoType;
    use entities::default::Entity;

    #[derive(Debug)]
    pub struct Game<'a>(&'a Entity);
    impl<'a> EntityProtoType for Game<'a> {}
}
