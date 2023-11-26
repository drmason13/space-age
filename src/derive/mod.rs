use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Lit};

macro_rules! bail {( $err_msg:expr$(, $span:expr)? $(,)? ) => (
    {
        let mut _span = ::proc_macro2::Span::call_site();
        $( _span = $span; )?
        return ::syn::Error::new(_span, $err_msg)
                   .to_compile_error()
                   .into()
        ;
    }
)}

#[proc_macro_derive(Planet, attributes(orbital_period))]
pub fn planet(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let DeriveInput {
        ident,
        attrs,
        data: _,
        vis: _,
        generics: _,
    } = parse_macro_input!(input);

    // find the attribute we care about
    let orbital_period_attr = attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("orbital_period"));

    let Some(orbital_period_attr) = orbital_period_attr else {
        bail!(
            "missing orbital_period attribute, e.g. #[orbital_period = 1.0]",
            ident.span()
        )
    };

    let syn::Meta::NameValue(meta_name_value) = orbital_period_attr.meta else {
        bail!("expected a NameValue style attribue, e.g. #[orbital_period = 1.0]",)
    };
    let orbital_period = match meta_name_value.value {
        syn::Expr::Lit(lit) => match lit.lit {
            Lit::Float(value) => value,
            _ => bail!(
                "expected a float value, e.g. #[orbital_period = 1.0]",
                lit.span(),
            ),
        },
        _ => bail!("expected a float value, e.g. #[orbital_period = 1.0]"),
    };

    let expanded: proc_macro2::TokenStream = quote! {
        impl ::space_age::Planet for #ident {
            const ORBITAL_PERIOD: f64 = #orbital_period;
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
