#![warn(clippy::all, clippy::pedantic)]

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IniStruct)]
#[proc_macro_error]
pub fn derive_macro_ini_struct(input: TokenStream) -> TokenStream {
    ini_struct_derive_impl(input)
}

/// Extracts the inner type of an `Option<T>`. For example, if the type is
/// `Option<String>`, this function will return `Some(String)`. If the type is
/// not an `Option<T>`, this function will return None.
///
/// Modified from [Duskmoon](https://duskmoon314.com/en/blog/extract-type-from-option-in-rs-procmacro/#five-ways-to-write-option)
///
/// # Arguments
///
/// * `ty` - The type to extract the inner type from.
fn extract_option_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args,
            ..
        }) = &segments.last()?.arguments
        {
            args.first().and_then(|arg| {
                if let syn::GenericArgument::Type(ty) = arg {
                    Some(ty)
                } else {
                    None
                }
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn ini_struct_derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (from_impl, to_impl) = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(struct_fields),
        ..
    }) = &ast.data
    {
        let (optional_fields, required_fields) = struct_fields
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref().map(|ident| (ident, &f.ty)).clone())
            .partition::<Vec<_>, _>(|(_, ty)| {
                if let syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                }) = ty
                {
                    if segments.last().is_some_and(|s| s.ident == "Option") {
                        return true;
                    }
                }
                false
            });

        const MAP_NAME: &str = "pairs";
        let map_name_ident = syn::Ident::new(MAP_NAME, proc_macro2::Span::call_site());

        let (optional_from_defs, optional_to_defs) = optional_fields.iter().map(|(ident, ty)| {
            let inner_ty = extract_option_type(ty).unwrap_or_else(|| {
                abort_call_site!(
                    "expected Option<T> for field {}, found {}",
                    ident,
                    stringify!(ty)
                )
            });
            let from_def = quote! {
                let #ident = #map_name_ident
                    .remove(stringify!(#ident))
                    .map(|s| s.parse::<#inner_ty>())
                    .transpose()
                    .map_err(|_| ::rustini_core::anyhow::anyhow!("invalid value for key: {}", stringify!(#ident)))?;
            };
            // TODO: add option for None values to include blank key (i.e. `key = `)
            let to_def = quote! {
                if let Some(#ident) = &self.#ident {
                    lines.push(format!("{} = {}", stringify!(#ident), #ident));
                }
            };
            (from_def, to_def)
        }).unzip::<_, _, Vec<_>, Vec<_>>();

        let (req_from_defs, req_to_defs) = required_fields.iter().map(|(ident, ty)| {
            let from_def = quote! {
                let #ident: #ty = #map_name_ident
                    .remove(stringify!(#ident))
                    .ok_or(::rustini_core::anyhow::anyhow!("missing required field: {}", stringify!(#ident)))?
                    .parse::<#ty>()
                    .map_err(|_| ::rustini_core::anyhow::anyhow!("invalid value for key: {}", stringify!(#ident)))?;
            };
            let to_def = quote! {
                lines.push(format!("{} = {}", stringify!(#ident), self.#ident));
            };
            (from_def, to_def)
        }).unzip::<_, _, Vec<_>, Vec<_>>();

        let all_idents = struct_fields
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref().map(|ident| ident.clone()))
            .collect::<Vec<_>>();

        // TODO: sections
        // TODO: this error handling is GOD AWFUL and the expansion is ugly
        let from_impl = quote! {
            let ini = ini.as_ref();
            let mut #map_name_ident = ::std::collections::HashMap::from_ini(ini)?;
            #(
                #optional_from_defs
            )*
            #(
                #req_from_defs
            )*
            Ok(Self {
                #(#all_idents),*
            })
        };
        let to_impl = quote! {
            let mut lines = Vec::new();
            #(
                #optional_to_defs
            )*
            #(
                #req_to_defs
            )*
            lines.join("\n")
        };
        (from_impl, to_impl)
    } else {
        abort_call_site!("IniStruct can only be derived for structs with named fields")
    };
    let expanded = quote! {
        impl #impl_generics ::rustini_core::IniStruct<#name #ty_generics> for #name #ty_generics #where_clause {
            type Error = ::rustini_core::anyhow::Error;

            fn from_ini<S>(ini: S) -> Result<#name #ty_generics, Self::Error>
            where
                S: AsRef<str>,
            {
                #from_impl
            }

            fn to_ini(&self) -> String {
                #to_impl
            }
        }
    };
    expanded.into()
}
