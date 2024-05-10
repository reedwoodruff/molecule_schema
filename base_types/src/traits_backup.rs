use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::ConstraintSchema,
};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

pub trait GraphEnvironment<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    // fn get_element(&self, id: &Uid) -> Option<&TSchema>;
    // fn instantiate_element(&mut self, element: TSchema) -> Uid;
    // fn get_constraint_schema(&self) -> ConstraintSchema<TTypes, TValues>;
}
// pub struct AnyGSO {
//     value_type_id: TypeId,
//     value: Box<dyn Any>,
//     editor_type_id: TypeId,
//     editor: Box<dyn Any>,
//     edit: fn(&mut dyn Any),
// }
// impl AnyGSO {
//     fn initiate_edit(&self) -> GSOEditor {}
// }
//
// pub trait IntoAny {
//     fn into_any(self) -> AnyGSO;
// }

pub trait GSO {
    type Builder;

    /// Instance ID
    fn get_id(&self) -> Uid;
    fn get_constraint_schema_operative_tag(&self) -> &Tag;
    fn get_constraint_schema_template_tag(&self) -> &Tag;
    fn initiate_build() -> Self::Builder
    where
        Self: Sized;
    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid>;
}

pub trait Finalizable<T>: Default + Validate {
    fn finalize(&self) -> Result<T, ValidationErrors>;
}

pub struct GSOEditor {}

struct GSOBuilder<F, T>
where
    F: Finalizable<T>,
{
    wip_instance: F,
    _phantom: PhantomData<T>,
}
impl<F, T> GSOBuilder<F, T>
where
    F: Finalizable<T>,
{
    fn build(&self) -> Result<T, ValidationErrors> {
        // self.wip_instance.validate()?;
        // Ok(self.wip_instance.finalize())
        self.wip_instance.finalize()
    }
    fn new() -> Self {
        Self {
            wip_instance: F::default(),
            _phantom: PhantomData,
        }
    }
}

trait SetDisplay {
    fn set_display(&mut self, new_display: &str);
}

struct Word {
    display: String,
}

#[derive(Default, Validate)]
struct WordBuilder {
    #[validate(required)]
    display: Option<String>,
}
impl SetDisplay for WordBuilder {
    fn set_display(&mut self, new_display: &str) {
        self.display = Some(new_display.to_string());
    }
}

impl Finalizable<Word> for WordBuilder {
    fn finalize(&self) -> Result<Word, ValidationErrors> {
        self.validate()?;
        Ok(Word {
            display: self.display.as_ref().unwrap().clone(),
        })
    }
}
impl<F: SetDisplay + Finalizable<T>, T> GSOBuilder<F, T> {
    fn set_display(&mut self, new_display: &str) -> &mut Self {
        self.wip_instance.set_display(new_display);
        self
    }
}

impl GSO for Word {
    type Builder = GSOBuilder<WordBuilder, Self>;

    fn get_id(&self) -> Uid {
        // Get Instance ID
        todo!()
    }

    fn get_constraint_schema_template_tag(&self) -> &Tag {
        todo!()
    }

    fn get_constraint_schema_operative_tag(&self) -> &Tag {
        todo!()
    }

    fn initiate_build() -> Self::Builder {
        Self::Builder::new()
    }

    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_builder() {
        let new_word = Word::initiate_build()
            .set_display("Humgub")
            .build()
            .unwrap();
        let test: Box<dyn Any> = Box::new(new_word);
    }
}
