use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
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
        syn::Data::Enum(_) => panic!("Builder doesn't support enums"),
        syn::Data::Union(_) => panic!("Builder doesn't support unions"),
    };

    let fields_iter = fields.iter();
    let fields_len = fields_iter.len();
    let mut fields_idents = Vec::with_capacity(fields_len);

    let mut builder_fn = Vec::with_capacity(fields_len);

    for f in fields_iter {
        let arg_name = f.ident.clone().unwrap();
        let mut arg_ty = f.ty.clone();
        let fn_name = Ident::new(&format!("with_{}", arg_name.to_string().to_lowercase()), span.to_owned());
        fields_idents.push(arg_name.clone());
        // fields_types.push(ty.clone());
        // fields_fn_name.push(fn_name);
        let mut builder_fn_body = quote! {
            self.inner.#arg_name = #arg_name.into()
        };

        if let Type::Path(ref type_path) = arg_ty.clone() {


            let Some(segment) = type_path.path.segments.last() else { continue; };
            // let seg_ident = segment.ident.to_string();
            // if seg_ident == "String" {
            //     arg_ty = parse_quote! { arg_ty.to_token_stream().to_string().replace("String", "impl Into<String>").into_token_stream() }
            // }
            match segment.ident.to_string().as_str() {
                "String" => {
                    arg_ty = parse_quote! { impl Into<String> };
                }
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(arg) = args.args.first() {
                            if let syn::GenericArgument::Type(inner_type) = arg {
                                // let s = inner_type.to_token_stream().to_string();
                                // panic!("{s}");
                                arg_ty = match arg.to_token_stream().to_string().as_str() {
                                    "String" => parse_quote!{ impl Into<String> },
                                    _ => parse_quote!{ #inner_type },
                                };
                                builder_fn_body = quote! {
                                    self.inner.#arg_name = Some(#arg_name.into())
                                }
                            }
                            else if arg.to_token_stream().to_string() == "String" {
                                arg_ty = parse_quote! { impl Into<String> };
                                builder_fn_body = quote! {
                                    self.inner.#arg_name = Some(String::from(#arg_name.into()))
                                }
                            }
                        }
                    }
                    // let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { continue; };
                    // let Some(arg) = args.args.first() else { continue; };
                    
                    // if let syn::GenericArgument::Type(inner_type) = arg {
                    //     // let s = inner_type.to_token_stream().to_string();
                    //     // panic!("{s}");
                    //     arg_ty = parse_quote! { #inner_type };
                    //     builder_fn_body = quote! {
                    //         self.inner.#arg_name = Some(#arg_name)
                    //     }
                    // }
                    
                    // else if arg.to_token_stream().to_string() == "String" {
                    //     arg_ty = parse_quote! { impl Into<String> };
                    //     builder_fn_body = quote! {
                    //         self.inner.#arg_name = Some(String::from(#arg_name.into()))
                    //     }
                    // }
                        // while let Some(arg) = args.args.next() {
                        //     let disp_arg = arg.to_token_stream().to_string();
                        // }
                        // if let Some(arg) = args.args.first() {
                            
                        //     ty = parse_quote! {  }
                        // }
                    // }
                },
                _ => ()
            }

        }

        builder_fn.push(quote! {
            pub fn #fn_name (mut self, #arg_name: #arg_ty) -> #builder_name {
                #builder_fn_body; self
            }
        });
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
                    #(#fields_idents: Default::default()),*
                };
                #builder_name { inner }
            }

            #(
                // pub fn #fields_fn_name (mut self, #fields_idents: #fields_arg_types) -> #builder_name {
                //     self.inner.#fields_idents = #fields_idents.into(); self
                // }
                #builder_fn
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

