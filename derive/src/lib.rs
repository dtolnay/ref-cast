#![doc(html_root_url = "https://docs.rs/ref-cast-impl/0.2.0")]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(RefCast)]
pub fn derive_ref_cast(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    if !has_repr_c(&ast) {
        panic!("RefCast trait requires #[repr(C)] or #[repr(transparent)]");
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let from = only_field_ty(&ast);

    let expanded = quote! {
        impl #impl_generics ::ref_cast::RefCast for #name #ty_generics #where_clause {
            type From = #from;

            #[inline]
            fn ref_cast(_from: &Self::From) -> &Self {
                extern crate core as _core;

                // TODO: assert that `Self::From` and `Self` have the same size
                // and alignment.
                //
                // Cannot do this because `Self::From` may be a generic type
                // parameter of `Self` where `transmute` is not allowed:
                //
                //     #[allow(unused)]
                //     unsafe fn assert_same_size #impl_generics #where_clause () {
                //         _core::mem::forget(
                //             _core::mem::transmute::<#from, #name #ty_generics>(
                //                 _core::mem::uninitialized()));
                //     }
                //
                // Cannot do this because `Self::From` may not be sized:
                //
                //     debug_assert_eq!(_core::mem::size_of::<Self::From>(),
                //                      _core::mem::size_of::<Self>());
                //     debug_assert_eq!(_core::mem::align_of::<Self::From>(),
                //                      _core::mem::align_of::<Self>());

                unsafe {
                    _core::mem::transmute::<&Self::From, &Self>(_from)
                }
            }

            #[inline]
            fn ref_cast_mut(_from: &mut Self::From) -> &mut Self {
                extern crate core as _core;
                unsafe {
                    _core::mem::transmute::<&mut Self::From, &mut Self>(_from)
                }
            }
        }
    };

    expanded.parse().unwrap()
}

fn has_repr_c(ast: &syn::DeriveInput) -> bool {
    for attr in &ast.attrs {
        if let syn::MetaItem::List(ref ident, ref nested) = attr.value {
            if ident == "repr" && nested.len() == 1 {
                if let syn::NestedMetaItem::MetaItem(ref inner) = nested[0] {
                    if let syn::MetaItem::Word(ref ident) = *inner {
                        if ident == "C" || ident == "transparent" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn only_field_ty(ast: &syn::DeriveInput) -> &syn::Ty {
    let fields = match ast.body {
        syn::Body::Struct(ref variant) => variant.fields(),
        syn::Body::Enum(_) => {
            panic!("RefCast does not support enums");
        }
    };

    // TODO: support structs that have trivial other fields like `()` or
    // `PhantomData`.
    if fields.len() != 1 {
        panic!("RefCast requires a struct with a single field");
    }

    &fields[0].ty
}
