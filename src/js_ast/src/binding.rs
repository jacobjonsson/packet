use crate::{
    expression::{Expression, Identifier},
    object::LiteralPropertyName,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Identifier(Identifier),
    RestElementBinding(RestElementBinding),
    ObjectBinding(ObjectBinding),
    ArrayBinding(ArrayBinding),
}

/// (...a) => {}
/// (...{}) => {}
/// (...[]) => {}
#[derive(Debug, PartialEq, Clone)]
pub struct RestElementBinding {
    pub key: RestElementBindingKey,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RestElementBindingKey {
    Identifier(Identifier),
    ObjectBinding(ObjectBinding),
    ArrayBinding(ArrayBinding),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBinding {
    pub properties: Vec<ObjectBindingProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingProperty {
    pub property: ObjectBindingPropertyKind,
    /// NB: Default value will always be None when property is a rest element.
    pub default_value: Option<Expression>,
}

/// ({ a }) => {}
/// ({ a: b }) => {}
/// ({ a: { b } }) => {}
/// ({ a: [ b ] }) => {}
/// ({ [a]: b }) => {}
/// ({ [a]: { b } }) => {}
/// ({ [a]: { b } }) => {}
/// ({ c, ...b }) => {}
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectBindingPropertyKind {
    ObjectBindingStaticProperty(ObjectBindingStaticProperty),
    ObjectBindingShorthandProperty(ObjectBindingShorthandProperty),
    ObjectBindingComputedProperty(ObjectBindingComputedProperty),
    ObjectBindingRestProperty(ObjectBindingRestProperty),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingShorthandProperty {
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingStaticProperty {
    pub identifier: LiteralPropertyName,
    pub value: Binding,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingComputedProperty {
    pub key: Expression,
    pub value: Binding,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingRestProperty {
    pub identifier: Identifier,
}

/// ([a]) => {}
/// ([[a]]) => {}
/// ([{a}]) => {}
/// ([{a: [a]}]) => {}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBinding {
    pub items: Vec<ArrayBindingItem>,
}

// The default_value will always be none for rest elements since they can't have a default value.
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBindingItem {
    pub value: Binding,
    pub default_value: Option<Expression>,
}
