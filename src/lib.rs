use std::marker::PhantomData;

pub const UNIT: TypeBound = TypeBound::Concrete(ConcreteType::Primitive(0));

pub enum ConcreteType {
    Primitive(usize),
}

pub struct GenericType {
    generics: GenericParameterList,
}

pub enum ContextualTypeBound {
    ExposedType(TypeBound),
    HiddenType(TypeBound),
}

pub struct GenericTraitImplementation {
    pub negative: bool,
    pub trait_id: Id<GenericTrait>,
    pub params: Vec<TypeBound>,
}

pub enum TypeBound {
    Generic(GenericBound),
    Concrete(ConcreteType),
}

pub struct GenericBound {
    pub exposed: bool,
    pub traits: Vec<GenericTraitImplementation>,
}

impl ContextualTypeBound {
    fn bound(&self) -> &TypeBound {
        use ContextualTypeBound::*;
        match self {
            ExposedType(t) | HiddenType(t) => &t,
        }
    }
}

impl TypeBound {
    fn test_type(
        &self,
        typ: Id<ConcreteType>,
        reg: &mut TypeRegistry,
        errs: &mut Vec<TypeError>,
    ) -> Result<(), ()> {
        todo!()
    }

    fn conflicts(
        &self,
        other: &TypeBound,
        reg: &mut TypeRegistry,
        errs: &mut Vec<TypeError>,
    ) -> Result<(), ()> {
        todo!()
    }
}

pub struct GenericParameterList(Vec<TypeBound>);

// struct  {
//     pub generics: Vec<(TypeBound, bool)>,
// }

enum TypeError {
    IncompatibleTypes(Id<ConcreteType>, Id<ConcreteType>),
    NotImplemented(Id<ConcreteTrait>),
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id<T>(usize, PhantomData<T>);

impl<T> From<usize> for Id<T> {
    fn from(id: usize) -> Self {
        Id(id, PhantomData)
    }
}

pub struct GenericTrait {
    pub generics: GenericParameterList,
    pub assoc_types: Vec<GenericType>,
}

struct ConcreteTrait {
    pub generics: Vec<ConcreteType>,
    pub assoc_types: Vec<GenericType>,
}

struct TypeRegistry {}
