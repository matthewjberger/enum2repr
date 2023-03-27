use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(EnumRepr, attributes(enum2repr))]
pub fn derive_enum2repr(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let mut repr = None;
    for attr in input.attrs {
        if attr.path.is_ident("repr") {
            if let Ok(name) = attr.parse_args::<Ident>() {
                match name.to_string().as_str() {
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
                    | "i64" | "i128" | "isize" => {
                        repr = Some(quote!(#name));
                    }
                    _ => {}
                }
            }
        }
    }

    if repr.is_none() {
        return derive_error!("The #[repr(T)] attribute is required when using EnumRepr.");
    }

    let name = &input.ident;
    let data = &input.data;

    let mut try_from_repr_match_arms;

    match data {
        Data::Enum(data_enum) => {
            try_from_repr_match_arms = TokenStream2::new();

            for variant in data_enum.variants.iter() {
                let variant_name = &variant.ident;

                if !matches!(&variant.fields, Fields::Unit) {
                    return derive_error!(
                        "EnumRepr is only implemented for named unit enum fields"
                    );
                }

                try_from_repr_match_arms.extend(quote_spanned! {
                    variant.span() =>
                        x if x == #name::#variant_name as #repr => Ok(#name::#variant_name),
                });
            }
        }
        _ => return derive_error!("EnumRepr is only implemented for enums"),
    };

    let expanded = quote! {
        impl TryFrom<#repr> for #name {
            type Error = &'static str;

            fn try_from(value: #repr) -> Result<Self, Self::Error> {
                match value {
                    #try_from_repr_match_arms
                    _ => Err("Failed to convert enum to numeric value!")
                }
            }
        }

        impl From<#name> for #repr {
            fn from(value: #name) -> #repr {
                value as #repr
            }
        }
    };

    TokenStream::from(expanded)
}
