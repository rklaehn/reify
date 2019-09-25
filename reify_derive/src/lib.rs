extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

#[proc_macro_derive(Reify)]
pub fn derive_reify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Reify` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let sum = reify_combine(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics reify::Reify for #name #ty_generics #where_clause {
            fn ast(_: std::marker::PhantomData<&Self>) -> reify::Ast2 {
                #sum
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: Reify` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(reify::Reify));
        }
    }
    generics
}

// Generate an expression to sum up the heap size of each field.
fn reify_combine(data: &Data) -> TokenStream {
    let transform_fields = |fields: &syn::Fields| -> TokenStream {
        match fields {
            Fields::Named(ref fields) => {
                // We take some care to use the span of each `syn::Field` as
                // the span of the corresponding `ast`
                // call. This way if one of the field types does not
                // implement `Reify` then the compiler's error message
                // underlines which field it is. An example is shown in the
                // readme of the parent directory.
                let recurse = fields.named.iter().map(|f| {
                    let name = format!("{}", f.ident.as_ref().unwrap());
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() => (#name, reify::ast::<#ty>())
                    }
                });
                quote! {
                    reify::Fields::Named(vec![#(#recurse, )*])
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    // let index = Index::from(i);
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() => reify::ast::<#ty>()
                    }
                });
                quote! {
                    reify::Fields::Unnamed(vec![#(#recurse, )*])
                }
            }
            Fields::Unit => {
                quote! {
                    reify::Fields::Unit
                }
            }
        }
    };
    match *data {
        Data::Struct(ref data) => {
            let fields = transform_fields(&data.fields);
            quote!(reify::Ast2::Struct(#fields))
        }
        Data::Enum(ref data) => {
            let for_each_variant = data.variants.iter().map(|f| {
                let name = format!("{}", f.ident);
                let fields = transform_fields(&f.fields);
                quote_spanned! {
                    f.span() => (#name, #fields)
                }
            });
            quote! {
                reify::Ast2::Enum(vec![#(#for_each_variant, )*])
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}
