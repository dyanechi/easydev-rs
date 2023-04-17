use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, parse_macro_input, Token};

use super::*;

pub fn with_builder(item: TokenStream) -> TokenStream {
    let DeriveInput {
        vis,
        ident: struct_name,
        data,
        generics,
        attrs,
    } = parse_macro_input!(item as DeriveInput);

    let span = Span::call_site();
    let builder_name = Ident::new(&(struct_name.to_string() + "Builder"), span); 

    let syn::Data::Struct(syn::DataStruct { fields, ..}) = data else { panic!("WithBuilder supports only structs") };

    let mut field_idents = Vec::with_capacity(fields.len());
    let mut builder_fns = Vec::with_capacity(fields.len());

    for field in fields.iter() {
        let mut arg_ty = field.ty.to_owned();
        let arg_name = field.ident.to_owned().unwrap();
        let fn_name = Ident::new(&format!("with_{}", arg_name.to_string()), span);

        let mut fn_val_assign = quote! { #arg_name.into() };

        if let Type::Path(ref type_path) = arg_ty.clone()
        && let Some(segment) = type_path.path.segments.last() {
            let seg_ident = segment.ident.to_string();
            if seg_ident == "String" {
                arg_ty = parse_quote! { impl Into<String> };
            }

            if seg_ident == "Option" 
            && let syn::PathArguments::AngleBracketed(generics) = &segment.arguments
            && let Some(generic_arg) = generics.args.first()
            && let syn::GenericArgument::Type(inner_type) = generic_arg {
                arg_ty = match generic_arg.to_token_stream().to_string().as_str() {
                    "String" => parse_quote!{ impl Into<String> },
                    _ => parse_quote!{ #inner_type },
                };
                fn_val_assign = quote! { Some(#arg_name.into()) }
            }
        }

        builder_fns.push(quote! {
            pub fn #fn_name (mut self, #arg_name: #arg_ty) -> #builder_name {
                self.inner.#arg_name = #fn_val_assign; self
            }
        });
        field_idents.push(arg_name);
    }

    let stream = quote! {
        impl #struct_name {
            pub fn new() -> #builder_name {
                #builder_name ::new()
            }
        }
        impl BuildAble for #struct_name {}

        #vis struct #builder_name {
            inner: #struct_name
        }

        impl #builder_name {
            fn new() -> #builder_name {
                let inner = #struct_name {
                    #(#field_idents: Default::default()),*
                };
                #builder_name { inner }
            }

            #( #builder_fns )*
        }

        impl WithBuilder<#struct_name> for #builder_name {
            fn build(self) -> #struct_name {
                self.inner
            }
        }
    };

    stream.into()
}

// #[macro_export]
// pub macro builder {
//     (
//         $vis:vis $ident:ident {
//             $($prop:ident: $value:ty,)*
//         }
//     ) => {
//         paste::paste! {
//             $vis struct $ident {
//                 $($prop: $value),*
//             }
//             impl $ident {
//                 pub fn new() -> [<$ident Builder>] {
//                     [<$ident Builder>]::new()
//                 }
//             }

//             $vis struct [<$ident Builder>] {
//                 inner: $ident
//             }

//             impl [<$ident Builder>] {
//                 fn new() -> [<$ident Builder>] {
//                     let inner = $ident {
//                         $($prop: Default::default()),*
//                     };
//                     [<$ident Builder>] { inner }
//                 }

//                 pub fn build(self) -> $ident {
//                     self.inner
//                 }

//                 $(
//                     $vis fn [<with_ $prop>](mut self, $prop: $value) -> [<$ident Builder>] {
//                         self.inner.$prop = $prop; self
//                     }
//                 )*
//             }
//         }
//     }
// }

