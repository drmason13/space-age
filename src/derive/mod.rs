use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

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
        .iter()
        .find(|attr| attr.path.is_ident("orbital_period"));

    // bail if it doesn't exist
    if orbital_period_attr.is_none() {
        bail!(
            "is missing its orbital_period attribute, e.g. #[orbital_period = 1.0]",
            ident.span()
        )
    }

    match orbital_period_attr.unwrap().parse_meta() {
        Ok(Meta::NameValue(value)) => {
            if let Lit::Float(orbital_period) = value.lit {
                // Build the output, possibly using quasi-quotation
                let expanded = quote! {
                    impl ::space_age::Planet for #ident {
                        const ORBITAL_PERIOD: f64 = #orbital_period;
                    }
                };

                // Hand the output tokens back to the compiler
                TokenStream::from(expanded)
            } else {
                bail!("expected a float value, e.g. #[orbital_period = 1.0]")
            }
        }
        Ok(bad) => bail!(
            "expected a NameValue style attribue, e.g. #[orbital_period = 1.0]",
            bad.path().span()
        ),
        Err(_) => bail!("expected a NameValue style attribue, e.g. #[orbital_period = 1.0]",),
    }
}
