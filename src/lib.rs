use std::marker::PhantomData;

struct ConcreteType;

struct GenericType {
    generics: GenericParameterList,
}

enum TypeBound {
    Type(ConcreteType),
    And(Box<TypeBound>, Box<TypeBound>),
    Implements(Id<ConcreteTrait>),
    Generic(Id<GenericType>),
    Any,
}

impl TypeBound {
    fn test(&self, typ: Id<ConcreteType>, reg: &mut TypeRegistry) -> Result<(), TypeError> {
        todo!()
    }
}

struct GenericParameterList(Vec<TypeBound>);

struct TypeInferencer {
    pub generics: GenericParameterList,
}

enum TypeError {
    IncompatibleTypes(Id<ConcreteType>, Id<ConcreteType>),
    NotImplemented(Id<ConcreteTrait>),
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Id<T>(usize, PhantomData<T>);

impl<T> From<usize> for Id<T> {
    fn from(id: usize) -> Self {
        Id(id, PhantomData)
    }
}

struct GenericTrait {
    pub generics: GenericParameterList,
    pub assoc_types: Vec<GenericType>,
}

struct ConcreteTrait {
    pub generics: Vec<ConcreteType>,
}

struct TypeRegistry {}
