use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub const UNIT: TypeBound = TypeBound::Concrete(ConcreteType::Primitive(0));

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ConcreteType {
    Primitive(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TypeKey {
    typ: Id<GenericType>,
    params: ConcreteParameterList,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TraitKey {
    id: Id<GenericTrait>,
    params: ConcreteParameterList,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericType {
    generics: GenericParameterList,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericTraitImpl {
    pub negative: bool,
    pub trait_id: Id<GenericTrait>,
    pub params: Vec<TypeBound>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TypeBound {
    Generic(GenericBound),
    Concrete(ConcreteType),
    Unknown,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericBound {
    pub exposed: bool,
    pub traits: Vec<GenericTraitImpl>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericParameterList(Vec<TypeBound>);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ConcreteParameterList(Vec<Id<ConcreteType>>);

enum TypeError {
    IncompatibleTypes(Id<ConcreteType>, Id<ConcreteType>),
    NotImplemented(Id<ConcreteTrait>),
    IncorrectGenericArgCount { expected: usize, actual: usize },
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id<T>(usize, PhantomData<T>);

impl<T> Id<T> {
    fn new(id: usize) -> Self {
        Id(id, PhantomData)
    }

    pub fn num(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Trait {
    Generic(GenericTrait),
    Concrete(ConcreteTrait),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Type {
    Generic(GenericType),
    Concrete(ConcreteType),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericTrait {
    pub generics: GenericParameterList,
    pub assoc_types: Vec<GenericType>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct ConcreteTrait {
    pub generic_id: Option<usize>,
    pub generics: Vec<ConcreteType>,
    pub assoc_types: Vec<GenericType>,
}

#[derive(Default)]
pub struct TypeRegistry {
    types: Vec<Type>,
    traits: Vec<Trait>,
    // trait ID -> implementors
    concrete_impls: Vec<Vec<Id<ConcreteType>>>,
    // generic trait ID -> blanket impls
    generic_impls: Vec<Vec<Id<GenericTraitImpl>>>,
    // id of generic type + concrete type parameters -> concrete type
    generic_type_lookup: HashMap<TypeKey, Id<ConcreteType>>,
    // id of generic trait + concrete type parameters -> concrete trait
    generic_trait_lookup: HashMap<TraitKey, Id<ConcreteTrait>>,
}

macro_rules! impl_index {
    ($enum:ident :: $variant:ident >> $id:ty = $field:ident) => {
        impl Index<Id<$id>> for TypeRegistry {
            type Output = $id;

            fn index(&self, index: Id<$id>) -> &Self::Output {
                let $enum::$variant(typ) = &self.$field[index.0] else {
                    panic!("ID type mismatch: {} is not {}", index.0, stringify!($id));
                };
                typ
            }
        }
    };
}

impl_index!(Type::Concrete >> ConcreteType = types);
impl_index!(Type::Generic >> GenericType = types);
impl_index!(Trait::Concrete >> ConcreteTrait = traits);
impl_index!(Trait::Generic >> GenericTrait = traits);

pub struct TypeScope {
    pub types: Vec<TypeBound>,
}
