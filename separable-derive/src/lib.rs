use proc_macro::{TokenStream};
use proc_macro2::{Span};
use syn::{parse_macro_input, DeriveInput, Data, Ident, Fields, spanned::Spanned};
use quote::quote;

use self::error::Error;
mod error;

/// Derive macro for the `Separable` trait
///
/// See the main page of the documentation for a quick example
#[proc_macro_derive(Separable)]
pub fn separable_derive(input: TokenStream) -> TokenStream {
    // We first parse de input
    let input = parse_macro_input!(input as DeriveInput);

    separable_derive_wrapper(input).unwrap_or_else(|e| e.to_syn_error().to_compile_error().into())
}

fn separable_derive_wrapper(input: DeriveInput) -> Result<TokenStream, Error> {
    // We quickly check if the derive candidate is a struct
    let enum_data = match input.data {
        Data::Enum(enum_data) => enum_data,
        Data::Struct(s) => return Err(Error::custom("derive macro works only with enums", s.struct_token.span)),
        Data::Union(u) => return Err(Error::custom("derive macro works only with enums", u.union_token.span))
    };

    // Name of the enum
    let name = &input.ident;

    // Check that all the variants are tupples with one element
    let types = enum_data.variants.iter().map(|variant| {
        match &variant.fields {
            Fields::Unnamed(unnamed) => {
                if unnamed.unnamed.len() != 1 {
                    Err(Error::custom("tupple variants may contain only 1 element", unnamed.span()))
                } else {
                    let field = unnamed.unnamed.first().unwrap();
                    let field_type = &field.ty;
                    //Ok(quote! {Vec<#field_type>})
                    Ok(field_type)
                }
            },
            _ => Err(Error::custom("unsupported variant, this macro only works with unnamed single-value tuple variants", variant.ident.span()))
        }
    }).collect::<Result<Vec<_>, _>>()?;

    // We create the referenced types, and mutable referenced types
    let vecced_types = types.iter().map(|ty| quote! {Vec<#ty>}).collect::<Vec<_>>();
    let ref_vecced_types = types.iter().map(|ty| quote! {Vec<&'a #ty>}).collect::<Vec<_>>();
    let ref_mut_vecced_types = types.iter().map(|ty| quote! {Vec<&'a mut #ty>}).collect::<Vec<_>>();

    let collections = enum_data.variants.iter().enumerate().map(|(idx, _)| {
        Ident::new(&format!("collection_{}", idx), Span::call_site())
    }).collect::<Vec<_>>();

    let collection_creation = enum_data.variants.iter().enumerate().map(|(idx, _)| {
        let collection_name = Ident::new(&format!("collection_{}", idx), Span::call_site());
        quote!{let mut #collection_name = Vec::new();}
    }).collect::<Vec<_>>();

    let variants = enum_data.variants.iter().enumerate().map(|(idx, variant)| {
        let variant_name = &variant.ident;
        let collection_name = Ident::new(&format!("collection_{}", idx), Span::call_site());
        quote!{#name::#variant_name(v) => #collection_name.push(v)}
    }).collect::<Vec<_>>();

    let expanded = quote! {
        impl FromIterator<#name> for (#(#vecced_types),*, ) {
            fn from_iter<I: IntoIterator<Item=#name>>(iter: I) -> Self {
                #(#collection_creation);*

                for i in iter {
                    match i {
                        #(#variants),*
                    }
                }
        
                (#(#collections),*, )
            }
        }

        // With mutable reference implementation
        impl<'a> FromIterator<&'a #name> for (#(#ref_vecced_types),*, ) {
            fn from_iter<I: IntoIterator<Item=&'a #name>>(iter: I) -> Self {
                #(#collection_creation);*

                for i in iter {
                    match i {
                        #(#variants),*
                    }
                }
        
                (#(#collections),*, )
            }
        }

        impl<'a> FromIterator<&'a mut #name> for (#(#ref_mut_vecced_types),*, ) {
            fn from_iter<I: IntoIterator<Item=&'a mut #name>>(iter: I) -> Self {
                #(#collection_creation);*

                for i in iter {
                    match i {
                        #(#variants),*
                    }
                }
        
                (#(#collections),*, )
            }
        }

        impl separable::Separable for #name {
            type Target = (#(#vecced_types),*, );
        }

        impl<'a> separable::Separable for &'a #name {
            type Target = (#(#ref_vecced_types),*, );
        }

        impl<'a> separable::Separable for &'a mut #name {
            type Target = (#(#ref_mut_vecced_types),*, );
        }
    };

    Ok(TokenStream::from(expanded))
}