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
        syn::Data::Enum(_) => panic!("Builder doesn't support enums"),
        syn::Data::Union(_) => panic!("Builder doesn't support unions"),
    };

    let fields_iter = fields.iter();
    let fields_len = fields_iter.len();
    let mut fields_idents = Vec::with_capacity(fields_len);
    let mut fields_types = Vec::with_capacity(fields_len);
    let mut fields_arg_types = Vec::with_capacity(fields_len);
    let mut fields_fn_name = Vec::with_capacity(fields_len);

    for f in fields_iter {
        let ident = f.ident.clone().unwrap();
        let mut ty = f.ty.clone();
        let fn_name = Ident::new(&format!("with_{}", ident.to_string().to_lowercase()), span.to_owned());
        fields_idents.push(ident);
        fields_types.push(ty.clone());
        fields_fn_name.push(fn_name);

        if let Type::Path(type_path) = &ty.clone() {
            // if type_path.path.segments[0].ident.to_string() == "Option" {
            //     // ty = parse_quote! { String };
            //     if let Some(inner_type) = type_path.path.segments.last().and_then(|seg| {
            //         if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
            //             args.args.first().and_then(|arg| {
            //                 if let syn::GenericArgument::Type(inner_type) = arg {
            //                     Some(inner_type)
            //                 } else {
            //                     None
            //                 }
            //             })
            //         } else {
            //             None
            //         }
            //     }) {
            //         ty = parse_quote! { #inner_type };
            //     }
            // }
            if let Some(segment) = type_path.path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "String" => ty = parse_quote! { impl Into<String> },
                    _ => ()
                }
            }
        }
        fields_arg_types.push(ty);
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
                pub fn #fields_fn_name (mut self, #fields_idents: #fields_arg_types) -> #builder_name {
                    self.inner.#fields_idents = #fields_idents.into(); self
                }
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

