// merlin_macros/src/lib.rs

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn merlin_syscall(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    let mut syscall_id: u32 = 0;
    for arg in attr_args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("id") {
                if let Lit::Int(litint) = nv.lit {
                    syscall_id = litint.base10_parse().unwrap_or(0);
                }
            }
        }
    }

    // Convert name string to 32-byte literal (padded with 0)
    let mut name_bytes = [0u8; 32];
    for (i, b) in fn_name_str.bytes().take(32).enumerate() {
        name_bytes[i] = b;
    }

    let name_tokens = name_bytes.iter().map(|b| quote! { #b });

    let gen = quote! {
        #input_fn

        #[link_section = ".merlin_syscall_entries"]
        #[used]
        #[no_mangle]
        pub static MERLIN_SYSCALL_ENTRY: crate::kernel::syscalls::syscall_macro::StaticSysCallEntry =
            crate::kernel::syscalls::syscall_macro::StaticSysCallEntry {
                id: #syscall_id,
                name: &[#(#name_tokens),*],
                handler: #fn_name,
            };
    };

    gen.into()
}
