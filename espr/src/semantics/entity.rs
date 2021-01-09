use super::{namespace::*, scope::*, *};
use crate::parser;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    name: String,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    name: String,
    ty: TypeRef,
    optional: bool,
}

impl Legalize for Entity {
    type Input = parser::entity::Entity;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        entity: &parser::entity::Entity,
    ) -> Result<Self, SemanticError> {
        let attributes = entity
            .attributes
            .iter()
            .map(|(name, ty)| {
                use parser::entity::ParameterType::*;
                let ty = match ty {
                    Named(name) => ns.lookup_type(scope, name)?,
                    Simple(ty) => namespace::TypeRef::SimpleType(*ty),
                };
                Ok(Attribute {
                    name: name.clone(),
                    ty,
                    optional: false,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Entity {
            name: entity.name.clone(),
            attributes,
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // EXPRESS identifier should be snake_case, but Rust struct should be PascalCase.
        let name = format_ident!("{}", self.name.to_pascal_case());

        let attr_name: Vec<_> = self
            .attributes
            .iter()
            .map(|Attribute { name, .. }| format_ident!("{}", name))
            .collect();
        let attr_type: Vec<_> = self
            .attributes
            .iter()
            .map(|Attribute { ty, optional, .. }| {
                if *optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                }
            })
            .collect();

        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #name {
                #(
                #attr_name : #attr_type,
                )*
            }

            impl #name {
                pub fn new(#(#attr_name : #attr_type),*) -> Self {
                    Self { #(#attr_name),* }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalize() {
        let example = SyntaxTree::example();
        let ns = Namespace::new(&example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &scope, entity).unwrap();
        dbg!(&entity);
    }
}