use super::*;

pub fn with_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let vis = &ast.vis;
    let name  = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        syn::Data::Enum(_) => panic!("Builder doesn't support enums"),
        syn::Data::Union(_) => panic!("Builder doesn't support unions"),
    };

    let mut fields_idents = vec![];
    let mut fields_types = vec![];
    let mut fields_arg_types = vec![];
    let fields_iter = fields.iter();

    for f in fields_iter {
        let ident = f.ident.clone().unwrap();
        let ty = f.ty.clone();
        fields_idents.push(ident);
        fields_types.push(ty.clone());

        if let Type::Path(type_path) = &ty {
            if type_path.path.is_ident("String") {
                let arg_type = parse_quote! { impl Into<String> };
                fields_arg_types.push(arg_type);
                continue;
            } 
        }
        fields_arg_types.push(ty);
    }

    let stream = quote! {
        paste::paste! {
            impl #name {
                pub fn new() -> [<#name Builder>] {
                    [<#name Builder>]::new()
                }
            }

            #vis struct [<#name Builder>] {
                inner: #name
            }

            impl [<#name Builder>] {
                fn new() -> [<#name Builder>] {
                    let inner = #name {
                        #(#fields_idents: Default::default()),*
                    };
                    [<#name Builder>] { inner }
                }

                pub fn build(self) -> #name {
                    self.inner
                }

                #(
                    pub fn [<with_ #fields_idents>](mut self, #fields_idents: #fields_arg_types) -> [<#name Builder>] {
                        self.inner.#fields_idents = #fields_idents.into(); self
                    }
                )*
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