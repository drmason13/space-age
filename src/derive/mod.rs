use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitFloat};

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
        .find(|attr| attr.path().is_ident("orbital_period"));

    // bail if it doesn't exist
    if orbital_period_attr.is_none() {
        bail!(
            "missing orbital_period attribute, e.g. #[orbital_period = 1.0]",
            ident.span()
        )
    }

    let mut expanded: proc_macro2::TokenStream = proc_macro2::TokenStream::default();

    if let Err(err) = orbital_period_attr.unwrap().parse_nested_meta(|meta| {
        // parse the "path": `orbital_period`
        if meta.path.is_ident("orbital_period") {
            // consume the `=` sign, and return the remaining input, which should be the value
            let value = meta.value()?;

            // now we finally get to the value we want
            if let Ok(orbital_period) = value.parse::<LitFloat>() {
                // Build the output using quote
                expanded = quote! {
                    impl ::space_age::Planet for #ident {
                        const ORBITAL_PERIOD: f64 = #orbital_period;
                    }
                };
                Ok(())
            } else {
                Err(::syn::Error::new(
                    value.span(),
                    "expected a float value, e.g. #[orbital_period = 1.0]",
                ))
            }
        } else {
            unreachable!()
        }
    }) {
        return err.to_compile_error().into();
    }

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
