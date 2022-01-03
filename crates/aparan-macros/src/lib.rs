use proc_macro::TokenStream;

use node::into_tokio_runtime;
use utils::token_stream_with_error;
use worker::into_async_trait;

/// An initializer for an Aparan node.
#[proc_macro_attribute]
pub fn node(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(et) => return token_stream_with_error(item, et),
    };

    into_tokio_runtime(input)
}

/// A macro that initializes an Aparan worker.
#[proc_macro_attribute]
pub fn worker(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemImpl = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(et) => return token_stream_with_error(item, et),
    };

    into_async_trait(input)
}

mod utils {
    use proc_macro::TokenStream;

    pub(crate) fn token_stream_with_error(
        mut tokens: TokenStream,
        error: syn::Error,
    ) -> TokenStream {
        tokens.extend(TokenStream::from(error.into_compile_error()));
        tokens
    }
}

mod worker {
    use proc_macro::TokenStream;
    use quote::quote;
    use syn::ItemImpl;

    pub(crate) fn into_async_trait(input: ItemImpl) -> TokenStream {
	let header = quote!( #[aparan::async_trait::async_trait] );

        let result = quote!(
	    #header
            #input
        );

        result.into()
    }
}

mod node {
    use proc_macro::TokenStream;
    use proc_macro2::Span;
    use quote::{quote, quote_spanned, ToTokens};
    use syn::ItemFn;

    /// Given an [`ItemFn`], create a Tokio runtime wrapping around it.
    /// Shamelessly adapted from
    /// https://github.com/tokio-rs/tokio/blob/12dd06336d2af8c2d735d4d9e3dc0454ad7942a0/tokio-macros/src/entry.rs
    pub(crate) fn into_tokio_runtime(mut input: ItemFn) -> TokenStream {
        // Turn of the `async` keyword.
        input.sig.asyncness = None;

        // If type mismatch occurs, the current rustc points to the last statement.
        let (last_stmt_start_span, last_stmt_end_span) = {
            let mut last_stmt = input
                .block
                .stmts
                .last()
                .map(ToTokens::into_token_stream)
                .unwrap_or_default()
                .into_iter();
            // `Span` on stable Rust has a limitation that only points to the first
            // token, not the whole tokens. We can work around this limitation by
            // using the first/last span of the tokens like
            // `syn::Error::new_spanned` does.
            let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
            let end = last_stmt.last().map_or(start, |t| t.span());
            (start, end)
        };

        let rt = quote_spanned! {last_stmt_start_span=>
            aparan::tokio::runtime::Builder::new_multi_thread()
        };

        let body = &input.block;
        let brace_token = input.block.brace_token;
        let (tail_return, tail_semicolon) = match body.stmts.last() {
            Some(syn::Stmt::Semi(syn::Expr::Return(_), _)) => (quote! { return }, quote! { ; }),
            Some(syn::Stmt::Semi(..)) | Some(syn::Stmt::Local(..)) | None => {
                match &input.sig.output {
                    syn::ReturnType::Type(_, ty) if matches!(&**ty, syn::Type::Tuple(ty) if ty.elems.is_empty()) =>
                    {
                        (quote! {}, quote! { ; }) // unit
                    }
                    syn::ReturnType::Default => (quote! {}, quote! { ; }), // unit
                    syn::ReturnType::Type(..) => (quote! {}, quote! {}),   // ! or another
                }
            }
            _ => (quote! {}, quote! {}),
        };
        input.block = syn::parse2(quote_spanned! {last_stmt_end_span=>
            {
                let body = async #body;
                #[allow(clippy::expect_used)]
                #tail_return #rt
                    .enable_all()
                    .build()
                    .expect("Failed building the Runtime")
                    .block_on(body)#tail_semicolon
            }
        })
        .expect("Parsing failure");
        input.block.brace_token = brace_token;

        let result = quote!(
        #input
        );

        result.into()
    }
}
