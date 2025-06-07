use heck::ToLowerCamelCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, DeriveInput, Expr, Lit, Meta, Token, parse_macro_input, punctuated::Punctuated,
};

#[proc_macro_derive(RequestMethods)]
pub fn derive_with_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("RequestMethods works only on structs with named fields");
    };

    let args = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if is_option_type(&field.ty) {
            None
        } else {
            Some(quote! {
                #field_name: #field_type
            })
        }
    });

    let create = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        if is_option_type(&field.ty) {
            quote! {#field_name: None}
        } else {
            quote! {#field_name}
        }
    });

    let methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let method_name = format_ident!("with_{}", field_name);
        let field_type = &field.ty;
        quote! {
            pub fn #method_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = #field_name;
                self
            }
        }
    });

    let open_query = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_key = extract_serde_rename(&field.attrs)
            .unwrap_or_else(|| field_name.to_string().to_lower_camel_case());
        if is_option_type(&field.ty) {
            quote! {
                query.add_option(#field_key, self.#field_name);
            }
        } else {
            quote! {
                query.add(#field_key, self.#field_name);
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn new(#(#args),*) -> Self {
                Self {
                    #(#create),*
                }
            }

            #(#methods)*

            pub fn fill_query<const N: usize>(self, query: &mut Query<N>) {
                #(#open_query)*
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn extract_serde_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let meta = attr.parse_args::<Meta>().ok()?;

        match meta {
            Meta::NameValue(name_value) if name_value.path.is_ident("rename") => {
                if let Expr::Lit(expr_lit) = name_value.value {
                    if let Lit::Str(lit_str) = expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
            Meta::List(meta_list) if meta_list.path.is_ident("rename") => {
                for nested in meta_list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .ok()?
                {
                    if let Meta::NameValue(nv) = nested {
                        if nv.path.is_ident("serialize") || nv.path.is_ident("deserialize") {
                            if let Expr::Lit(expr_lit) = nv.value {
                                if let Lit::Str(lit_str) = expr_lit.lit {
                                    return Some(lit_str.value());
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}
