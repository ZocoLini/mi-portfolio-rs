extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(MultiLang)]
pub fn derive_multilang(input: TokenStream) -> TokenStream {
    // Parsear el input del macro
    let input = parse_macro_input!(input as DeriveInput);

    // Obtener el nombre de la estructura
    let struct_name = input.ident;

    // Generar la implementación del trait
    let expanded = match input.data {
        syn::Data::Struct(data) => {
            // Extraer los campos de la estructura
            if let Fields::Named(fields) = data.fields {
                let translate_fields = fields.named.into_iter().map(|field| {
                    let field_name = field.ident.unwrap(); // Nombre del campo
                    match field.ty {
                        // Si el tipo implementa MultiLang, traducirlo
                        Type::Path(ref _type_path) => quote! {
                            #field_name: self.#field_name.translate(),
                        },
                        // Si el tipo no se puede traducir, copiar directamente
                        _ => quote! {
                            #field_name: self.#field_name,
                        },
                    }
                });

                // Crear el código generado
                quote! {
                    impl MultiLang for #struct_name {
                        fn translate(self) -> Self {
                            Self {
                                #(#translate_fields)*
                            }
                        }
                    }
                }
            } else {
                panic!("MultiLang can only be derived for structs with named fields");
            }
        }
        _ => panic!("MultiLang can only be derived for structs"),
    };

    // Convertir el código generado a un TokenStream
    TokenStream::from(expanded)
}
