#![doc(html_root_url = "https://docs.rs/ref-cast-impl/0.2.0")]

#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

use syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Type};

#[proc_macro_derive(RefCast)]
pub fn derive_ref_cast(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

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

    expanded.into()
}

fn has_repr_c(ast: &DeriveInput) -> bool {
    for attr in &ast.attrs {
        if let Some(meta) = attr.interpret_meta() {
            if let Meta::List(meta) = meta {
                if meta.ident == "repr" && meta.nested.len() == 1 {
                    if let NestedMeta::Meta(ref inner) = meta.nested[0] {
                        if let Meta::Word(ref ident) = *inner {
                            if ident == "C" || ident == "transparent" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn only_field_ty(ast: &DeriveInput) -> &Type {
    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            Fields::Unnamed(ref fields) => &fields.unnamed,
            Fields::Unit => {
                panic!("RefCast does not support unit structs");
            }
        },
        Data::Enum(_) => {
            panic!("RefCast does not support enums");
        }
        Data::Union(_) => {
            panic!("RefCast does not support unions");
        }
    };

    // TODO: support structs that have trivial other fields like `()` or
    // `PhantomData`.
    if fields.len() != 1 {
        panic!("RefCast requires a struct with a single field");
    }

    &fields[0].ty
}
