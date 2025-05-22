extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn merlin_syscall(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SysCallArgs);
    let syscall_id = args.id; // <-- FIXED: Extract value before quote!
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Pad name to 32 bytes
    let mut padded = [0u8; 32];
    for (i, b) in fn_name_str.bytes().enumerate().take(32) {
        padded[i] = b;
    }
    let name_tokens = padded.iter().map(|b| quote! { #b });

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

// Now include the struct at the bottom of the file
use syn::{
    parse::{Parse, ParseStream},
    MetaNameValue, Result as SynResult,
};

struct SysCallArgs {
    id: u32,
}

impl Parse for SysCallArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name_value: MetaNameValue = input.parse()?;
        if name_value.path.is_ident("id") {
            if let syn::Expr::Lit(expr_lit) = name_value.value {
                if let syn::Lit::Int(lit_int) = expr_lit.lit {
                    let id = lit_int.base10_parse::<u32>()?;
                    return Ok(SysCallArgs { id });
                }
            }
        }
        Err(input.error("expected syntax: id = <int>"))
    }
}
