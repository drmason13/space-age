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
        // bail with a helpful error if the required attribute isn't present at all
        bail!(
            "missing orbital_period attribute, e.g. #[orbital_period = 1.0]",
            ident.span()
        )
    };

    // confirm that the attribute is in the #[name = value] style
    let syn::Meta::NameValue(meta_name_value) = orbital_period_attr.meta else {
        bail!(
            "expected a NameValue style attribue, e.g. #[orbital_period = 1.0]",
            orbital_period_attr.meta.span()
        )
    };

    // extract the orbital period literal from the name=value pair ready to paste into our derived output
    let orbital_period = match meta_name_value.value {
        syn::Expr::Lit(lit) => match lit.lit {
            Lit::Float(value) => value,
            // bail with a helpful error if the value is a literal, but isn't a float
            _ => bail!(
                "expected a float value, e.g. #[orbital_period = 1.0]\n                                                ^^^",
                lit.span(),
            ),
        },
        // bail with a helpful error if the value isn't a literal at all
        _ => bail!("expected a literal value, e.g. #[orbital_period = 1.0]"),
    };

    // below, we can paste the tokens that make up the literal float value we extracted into orbital_period by prefixing it with a #
    // this is the code that we output in order to impl the Planet trait, which is what a derive macro does
    let expanded: proc_macro2::TokenStream = quote! {
        impl ::space_age::Planet for #ident {
            const ORBITAL_PERIOD: f64 = #orbital_period;
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)

    // Note: `TokenStream::from(exanded)` is needed simply to convert the proc_macro2 TokenStream used by quote into the proc_macro::TokenStream used by proc_macros.
    // quote simply isn't allowed to use proc_macro::TokenStream directly because the compiler forces that *only* proc macros can use  the types in proc_macro.
    // proc_macro2 is a copy of proc_macro for use by libraries that help users author proc_macros, such as quote.
}
