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

fn ini_struct_derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (from_impl, to_impl) = match &ast.data {
        syn::Data::Struct(st) if matches!(&st.fields, syn::Fields::Named(_)) => {
            let (field_idents, field_types) = st
                .fields
                .iter()
                .filter_map(|f| f.ident.as_ref().map(|ident| (ident, &f.ty)).clone())
                .unzip::<_, _, Vec<_>, Vec<_>>();
            // TODO: this error handling is GOD AWFUL and the expansion is ugly
            let from_impl = quote! {
                let ini = ini.as_ref();
                let pairs = ini
                    .lines()
                    .map(|line| {
                        let mut parts = line.splitn(2, '=');
                        let key = parts.next().ok_or(::rustini_core::anyhow::anyhow!("missing key"))?.trim();
                        let value = parts.next().map(|s| s.trim());
                        Ok((key, value))
                    })
                    .collect::<Result<::std::collections::HashMap<_, _>, ::rustini_core::anyhow::Error>>()?;
                Ok(Self {
                    #(
                        #field_idents: pairs
                            .get(stringify!(#field_idents))
                            .ok_or(::rustini_core::anyhow::anyhow!("missing key"))?
                            .ok_or(::rustini_core::anyhow::anyhow!("missing value for key: {}", stringify!(#field_idents)))?
                            .parse::<#field_types>()
                            .map_err(|_| ::rustini_core::anyhow::anyhow!("invalid value for key: {}", stringify!(#field_idents)))?
                    ),*
                })
            };
            let to_impl = quote! {
                todo!()
            };
            (from_impl, to_impl)
        }
        _ => abort_call_site!("IniStruct can only be derived for structs with named fields"),
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
