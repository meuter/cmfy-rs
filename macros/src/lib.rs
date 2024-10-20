// TODO: cleanup
#![allow(unused)]

use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(node), supports(struct_named))]
struct Node {
    ident: Ident,
    data: Data<(), NodeField>,
    class_type: String,
    #[darling(default)]
    trait_name: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(node_input))]
struct NodeField {
    ident: Option<Ident>,
    ty: syn::Type,
    #[darling(default)]
    skip: bool,
}

impl Node {
    fn ident(&self) -> &Ident {
        &self.ident
    }

    fn trait_name_ident(&self) -> Ident {
        let trait_name = self.trait_name.as_ref().unwrap_or(self.class_type());
        Ident::new(trait_name.as_str(), Span::mixed_site())
    }

    fn class_type(&self) -> &String {
        &self.class_type
    }

    fn fields(&self) -> impl Iterator<Item = &NodeField> {
        match &self.data {
            Data::Enum(_) => unreachable!(),
            Data::Struct(fields) => fields.fields.iter().filter(|f| !f.skip),
        }
    }
}

impl NodeField {
    fn ident(&self) -> &Ident {
        self.ident.as_ref().expect("only named struct is supported")
    }

    fn getter_ident(&self) -> &Ident {
        self.ident()
    }

    fn setter_ident(&self) -> Ident {
        let name = format!("set_{}", self.ident().to_string());
        Ident::new(name.as_str(), Span::mixed_site())
    }
}

#[allow(unused)]
#[proc_macro_derive(Node, attributes(node, node_input))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let node = Node::from_derive_input(&input).unwrap();
    let node_ident = node.ident();
    let class_type = node.class_type();

    let fields_methods = node.fields().map(|field| {
        let field_name = field.ident();
        let field_type = &field.ty;
        let get_field_name = field.getter_ident();
        let set_field_name = field.setter_ident();
        quote!(
            fn #get_field_name(&self) -> ::cmfy::Result<#field_type>;
            fn #set_field_name(&mut self, value: &#field_type) -> ::cmfy::Result<()>;
        )
    });

    let fields_methods_impl = node.fields().map(|field| {
        let field_name = field.ident();
        let field_type = &field.ty;
        let get_field_name = field.getter_ident();
        let set_field_name = field.setter_ident();
        quote!(
            fn #get_field_name(&self) -> ::cmfy::Result<#field_type> {
                let (_, node) = self.first_by_class::<#node_ident>()?;
                Ok(node.#field_name)
            }
            fn #set_field_name(&mut self, value: &#field_type) -> ::cmfy::Result<()> {
                self.change_first_by_class(|node: &mut #node_ident| {
                    node.#field_name = value.clone();
                })
            }
        )
    });

    let node_trait = node.trait_name_ident();
    let generated = quote!(
        impl ::cmfy::dto::ClassType for #node_ident {
            const CLASS_TYPE: &'static str = #class_type;
        }

        pub trait #node_trait {
            #(#fields_methods)*
        }

        impl #node_trait for ::cmfy::dto::PromptNodes {
            #(#fields_methods_impl)*
        }
    );

    generated.into()
}
