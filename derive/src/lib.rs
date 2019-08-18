#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, NestedMeta, Type};

type Result<T> = std::result::Result<T, &'static str>;

#[proc_macro_derive(RefCast)]
pub fn derive_ref_cast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = expand(input).unwrap_or_else(|error| quote! {
        compile_error! { #error }
    });
    TokenStream::from(expanded)
}

fn expand(input: DeriveInput) -> Result<TokenStream2> {
    if !has_repr_c(&input) {
        return Err("RefCast trait requires #[repr(C)] or #[repr(transparent)]");
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let from = only_field_ty(&input)?;

    Ok(quote! {
        impl #impl_generics ::ref_cast::RefCast for #name #ty_generics #where_clause {
            type From = #from;

            #[inline]
            fn ref_cast(_from: &Self::From) -> &Self {
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
                    &*(_from as *const Self::From as *const Self)
                }
            }

            #[inline]
            fn ref_cast_mut(_from: &mut Self::From) -> &mut Self {
                unsafe {
                    &mut *(_from as *mut Self::From as *mut Self)
                }
            }
        }
    })
}

fn has_repr_c(input: &DeriveInput) -> bool {
    for attr in &input.attrs {
        if let Ok(Meta::List(meta)) = attr.parse_meta() {
            if meta.path.is_ident("repr") && meta.nested.len() == 1 {
                if let NestedMeta::Meta(Meta::Path(path)) = &meta.nested[0] {
                    if path.is_ident("C") || path.is_ident("transparent") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn only_field_ty(input: &DeriveInput) -> Result<&Type> {
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => {
                return Err("RefCast does not support unit structs");
            }
        },
        Data::Enum(_) => {
            return Err("RefCast does not support enums");
        }
        Data::Union(_) => {
            return Err("RefCast does not support unions");
        }
    };

    // TODO: support structs that have trivial other fields like `()` or
    // `PhantomData`.
    if fields.len() != 1 {
        return Err("RefCast requires a struct with a single field");
    }

    Ok(&fields[0].ty)
}
