use proc_macro2::Ident;
use syn::spanned::Spanned;

use super::*;

pub fn with_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let vis = &ast.vis;
    let name  = &ast.ident;
    let span = ast.span().clone();
    let builder_name = Ident::new(&(name.to_string() + "Builder"), ast.span().to_owned());

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("WithBuilder supports only structs")
    };

    let mut field_idents = Vec::with_capacity(fields.len());
    let mut builder_fns = Vec::with_capacity(fields.len());

    for f in fields.iter() {
        let mut arg_ty = f.ty.clone();
        let arg_name = f.ident.clone().unwrap();
        let fn_name = Ident::new(&format!("with_{}", arg_name.to_string().to_lowercase()), span.to_owned());

        let mut builder_fn_body = quote! {
            self.inner.#arg_name = #arg_name.into()
        };

        match arg_ty.clone() {
            Type::Path(ref type_path) => {
                let Some(segment) = type_path.path.segments.last() else { continue; };
                match segment.ident.to_string().as_str() {
                    "String" => {
                        arg_ty = parse_quote! { impl Into<String> };
                    }
                    "Option" => {
                        if let syn::PathArguments::AngleBracketed(generic_args) = &segment.arguments
                            && let Some(arg) = generic_args.args.first()
                            && let syn::GenericArgument::Type(inner_type) = arg {
                                arg_ty = match arg.to_token_stream().to_string().as_str() {
                                    "String" => parse_quote!{ impl Into<String> },
                                    _ => parse_quote!{ #inner_type },
                                };
                                builder_fn_body = quote! {
                                    self.inner.#arg_name = Some(#arg_name.into())
                                }
                            }
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        
        builder_fns.push(quote! {
            pub fn #fn_name (mut self, #arg_name: #arg_ty) -> #builder_name {
                #builder_fn_body; self
            }
        });
        field_idents.push(arg_name);
    }

    let stream = quote! {
        impl #name {
            pub fn new() -> #builder_name {
                #builder_name ::new()
            }
        }
        impl BuildAble for #name {}

        #vis struct #builder_name {
            inner: #name
        }

        impl #builder_name {
            fn new() -> #builder_name {
                let inner = #name {
                    #(#field_idents: Default::default()),*
                };
                #builder_name { inner }
            }

            #(
                #builder_fns
            )*
        }

        impl WithBuilder<#name> for #builder_name {
            fn build(self) -> #name {
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

