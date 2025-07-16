use heck::ToTitleCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprLit, ItemEnum, Lit, LitStr, Result,
    parse::{Parse, ParseStream},
};

pub fn name(item: ItemEnum) -> TokenStream {
    let (vis, ident, variants) = (&item.vis, &item.ident, &item.variants);

    let mut match_arms = Vec::with_capacity(variants.len());
    let mut match_arms_plural = Vec::with_capacity(variants.len());

    for variant in variants {
        let ident = &variant.ident;

        let name = variant
            .attrs
            .iter()
            .find_map(|attr| {
                let meta = attr.meta.require_name_value().ok()?;
                if !meta.path.is_ident("name") {
                    return None;
                }
                match &meta.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) => Some(s.value()),
                    _ => None,
                }
            })
            .unwrap_or_else(|| ident.to_string().to_title_case());

        let name_plural = variant
            .attrs
            .iter()
            .find_map(|attr| {
                let meta = attr.meta.require_list().ok()?;
                if !meta.path.is_ident("name") {
                    return None;
                }
                struct Inner(String);
                impl Parse for Inner {
                    fn parse(input: ParseStream) -> Result<Self> {
                        syn::custom_keyword!(plural);

                        let _: plural = input.parse()?;
                        let _: syn::Token![=] = input.parse()?;
                        let s: LitStr = input.parse()?;
                        Ok(Self(s.value()))
                    }
                }
                meta.parse_args::<Inner>().ok().map(|i| i.0)
            })
            .unwrap_or_else(|| {
                let mut n = name.clone();
                n.push('s');
                n
            });

        match_arms.push(quote! { Self::#ident => #name });
        match_arms_plural.push(quote! { Self::#ident => #name_plural });
    }

    quote! {
        impl #ident {
            #vis const fn name(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }

            #vis const fn name_plural(&self) -> &'static str {
                match self {
                    #(#match_arms_plural),*
                }
            }

            #vis const fn name_pluralized(&self, n: usize) -> &'static str {
                if n == 1 { self.name() } else { self.name_plural() }
            }
        }
    }
}
