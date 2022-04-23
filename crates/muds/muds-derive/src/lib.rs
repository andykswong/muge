//! Implements the `#[derive(Entity)]`, `#[derive(Component)]` macro and `#[storage]` attribute.

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream, Result},
    DeriveInput, Path,
};

/// Derive macro for the `Entity` trait.
///
/// ## Examples
/// ```rust,ignore
/// use muds::ecs::storage::ArenaStorage;
///
/// #[derive(Entity, Debug)]
/// #[storage(ArenaStorage)] //optional, defaults to `ArenaStorage`
/// struct E(f32);
/// ```
#[proc_macro_derive(Entity, attributes(storage))]
pub fn entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_entity(&ast);
    gen.into()
}

fn impl_entity(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let storage = ast
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "storage")
        .map(|attr| {
            syn::parse2::<StorageAttribute>(attr.tokens.clone())
                .unwrap()
                .storage
        })
        .unwrap_or_else(|| parse_quote!(muds::ecs::storage::ArenaStorage));

    quote! {
        impl #impl_generics muds::ecs::Entity for #name #ty_generics #where_clause {
            type Storage = #storage<Self>;
        }
    }
}

/// Derive macro for the `Component` trait.
///
/// ## Examples
/// ```rust,ignore
/// use muds::ecs::storage::VecStorage;
///
/// #[derive(Component, Debug)]
/// #[storage(VecStorage)] // optional, defaults to `VecStorage`
/// struct C(f32);
/// ```
#[proc_macro_derive(Component, attributes(storage))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_component(&ast);
    gen.into()
}

fn impl_component(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_params = ast.generics.type_params();

    let storage = ast
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "storage")
        .map(|attr| {
            syn::parse2::<StorageAttribute>(attr.tokens.clone())
                .unwrap()
                .storage
        })
        .unwrap_or_else(|| parse_quote!(muds::ecs::storage::VecStorage));

    quote! {
        impl <Entity: muds::ecs::Entity + 'static, #(#type_params),*> muds::ecs::Component<Entity> for #name #ty_generics #where_clause {
            type Storage = #storage<Entity, Self>;
        }
    }
}

struct StorageAttribute {
    storage: Path,
}

impl Parse for StorageAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(StorageAttribute {
            storage: content.parse()?,
        })
    }
}
