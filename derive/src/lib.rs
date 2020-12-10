//! Helper macros for `web-sys-query`

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(OnEvent, attributes(unimplemented))]
pub fn derive_on_event(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    derive_on_event_enum(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn derive_on_event_enum(item: ItemEnum) -> syn::Result<TokenStream> {
    let mut on_event_handlers = vec![];
    let mut set_on_event_handlers = vec![];
    let mut off_event_handlers = vec![];
    let mut event_handlers = vec![];
    let mut collection_event_handlers = vec![];

    let doc1 = item.attrs.iter().filter(|attr| attr.path.is_ident("doc"));
    let doc2 = doc1.clone();

    let html = quote! {
        let html = self.dyn_ref::<web_sys::HtmlElement>()?;
    };
    let init_callback = quote! {
        #html
        let callback = Some(callback.as_ref().unchecked_ref());
    };
    let callback = quote! {
        callback: &Closure<dyn FnMut(web_sys::Event)>,
    };

    for variant in item.variants.iter().filter(|variant| {
        !variant
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("unimplemented"))
    }) {
        let ident = &variant.ident;
        let name = ident.to_string().to_case(Case::Snake);
        let get_ident = Ident::new(&name, Span::call_site());
        let set_ident = Ident::new(&format!("set_{}", name), Span::call_site());
        let name2 = ident.to_string().to_lowercase();
        let set_ident2 = Ident::new(&format!("set_on{}", name2), Span::call_site());
        let get_ident2 = Ident::new(&format!("on{}", name2), Span::call_site());

        on_event_handlers.push(quote! {
            Event::#ident => html.#get_ident2().ok_or(Error::EventNotHandled(Event::#ident)),
        });

        set_on_event_handlers.push(quote! {
            Event::#ident => Ok(html.#set_ident2(callback)),
        });

        off_event_handlers.push(quote! {
            Event::#ident => Ok(html.#set_ident2(None)),
        });

        event_handlers.push(quote! {
            fn #get_ident(&self) -> Result<js_sys::Function, Error> {
                #html
                html.#get_ident2().ok_or(Error::EventNotHandled(Event::#ident))
            }

            pub fn #set_ident(&self, #callback) -> Result<(), Error> {
                #init_callback
                Ok(html.#set_ident2(callback))
            }
        });

        collection_event_handlers.push(quote! {
            fn #get_ident(&self) -> Result<Vec<js_sys::Function>, Error> {
                self.0.iter().map(|elem| elem.#get_ident()).collect::<Result<Vec<_>, _>>()
            }

            pub fn #set_ident(&self, #callback) {
                self.0.iter().for_each(|elem| { elem.#set_ident(callback).ok(); })
            }
        });
    }

    Ok(quote! {
        #(#doc1)*
        impl Element {
            pub fn on(&self, event: Event) -> Result<js_sys::Function, Error> {
                #html

                match event {
                    #(#on_event_handlers)*
                    _ => Err(Error::EventNotImplemented(event)),
                }
            }

            pub fn set_on(&self, event: Event, #callback) -> Result<(), Error> {
                #init_callback

                match event {
                    #(#set_on_event_handlers)*
                    _ => Err(Error::EventNotImplemented(event)),
                }
            }

            pub fn set_off(&self, event: Event) -> Result<(), Error> {
                #html

                match event {
                    #(#off_event_handlers)*
                    _ => Err(Error::EventNotImplemented(event)),
                }
            }

            #(#event_handlers)*
        }

        #(#doc2)*
        impl Collection {
            pub fn on(&self, event: Event) -> Result<Vec<js_sys::Function>, Error> {
                self.0.iter().map(|elem| elem.on(event)).collect::<Result<Vec<_>, _>>()
            }

            pub fn set_on(&self, event: Event, #callback) {
                self.0.iter().for_each(|elem| { elem.set_on(event, callback).ok(); })
            }

            pub fn set_off(&self, event: Event) {
                self.0.iter().for_each(|elem| { elem.set_off(event).ok(); })
            }

        #(#collection_event_handlers)*
        }
    })
}
