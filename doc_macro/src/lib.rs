use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::Lit;
use syn::{parse_macro_input, Meta};

// 用于标记API文档
#[proc_macro_attribute]
pub fn api(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as syn::ItemFn);

    let mut doc_list = Vec::new();
    for attr_item in fn_item.attrs.iter() {
        if let Ok(meta) = attr_item.parse_meta() {
            match meta {
                Meta::Path(_val) => {
                    continue;
                }
                Meta::List(_meta_list) => {
                    continue;
                }
                Meta::NameValue(val) => {
                    let token_name = val.path.to_token_stream().to_string();
                    if token_name.as_str() == "doc" {
                        if let Lit::Str(val) = &val.lit {
                            doc_list.push(val.value());
                        }
                    }
                }
            }
        }
    }

    if doc_list.len() == 0 {
        let err = syn::Error::new(fn_item.span(), "excepted api document");
        return proc_macro::TokenStream::from(err.to_compile_error());
    }

    let api_doc = doc_def::document::parse_statement(doc_list);
    match api_doc {
        Ok(doc_obj) => {
            let result = doc_def::file::save_item(doc_def::file::ItemType::Api, &doc_obj);
            match result {
                Ok(_) => {}
                Err(err) => {
                    let err = syn::Error::new(fn_item.span(), err.as_str());
                    return proc_macro::TokenStream::from(err.to_compile_error());
                }
            }
        }
        Err(err) => {
            let err = syn::Error::new(fn_item.span(), err.as_str());
            return proc_macro::TokenStream::from(err.to_compile_error());
        }
    }

    quote!(#fn_item).into()
}

// 用于标记API文档的头部信息
#[proc_macro_attribute]
pub fn doc_header(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as syn::ItemFn);

    let mut doc_list = Vec::new();
    for attr_item in fn_item.attrs.iter() {
        if let Ok(meta) = attr_item.parse_meta() {
            match meta {
                Meta::Path(_val) => {
                    continue;
                }
                Meta::List(_meta_list) => {
                    continue;
                }
                Meta::NameValue(val) => {
                    let token_name = val.path.to_token_stream().to_string();
                    if token_name.as_str() == "doc" {
                        if let Lit::Str(val) = &val.lit {
                            doc_list.push(val.value());
                        }
                    }
                }
            }
        }
    }

    if doc_list.len() == 0 {
        let err = syn::Error::new(fn_item.span(), "excepted doc header");
        return proc_macro::TokenStream::from(err.to_compile_error());
    }

    let result: String = doc_list.join("\r\n");
    let result = doc_def::file::save_item_str(doc_def::file::ItemType::Header, &result);
    match result {
        Ok(_) => {}
        Err(err) => {
            let err = syn::Error::new(fn_item.span(), err.as_str());
            return proc_macro::TokenStream::from(err.to_compile_error());
        }
    }

    quote!(#fn_item).into()
}
