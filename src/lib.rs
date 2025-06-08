/*
 * Merlin OS
 * Copyright (C) 2023-2025 by Paolo Fabio Zaino, all rights reserved.
 *
 * This file is part of the Merlin OS project and is licensed under the MPL 2.0 +
 * the following restrictions:
 *
 *   - Based on Mozilla Public License 2.0
 *   - Modifications to this file must be released under the same license
 *   - Derivative works must credit the Merlin project and its contributors
 *   - This software is provided AS IS, without warranty of any kind
 *
 * Full MPL 2.0 license text available in the root LICENSE file or at:
 * <https://github.com/pzaino/merlin_macros/LICENSE>
 */

//!
//! This file allows the use of the `#[merlin_syscall]` attribute macro.
//! It is used to define a syscall entry point in the kernel.
//!
//! The macro takes an `id` argument, which is the syscall ID.
//! The macro generates a static syscall entry point with the given ID and the
//! function name as the syscall name.
//!
//! The function name is padded to 32 bytes with null bytes.
//! The generated entry point is placed in the `.merlin_syscall_entries` section
//! and is marked as used to prevent the linker from removing it.
//!
//! The generated entry point is also marked as no_mangle to prevent name
//! mangling, so that it can be called from user space and C or Assembly code.
//! The mentioning of calling from user space is because this macro is used
//! to map private ABI syscalls to public ABI syscalls.
//!
//! SysCalls defined using this macro are automatically registered in the Public ABI context
//! and can be used by user space applications.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// # This macro is used to define a syscall entry point in the kernel.
/// The macro takes an `id` argument, which is the syscall ID.
/// Example of use:
/// ```rust,ignore
/// #[merlin_syscall(id = 42)]
/// fn my_syscall() {
///     // syscall implementation
/// }
/// ```
/// This will generate a static syscall entry point with the given ID and the
/// function name as the syscall name.
/// Declaring the function as pub is optional, given that Merlin will
/// always dispatch private API syscall via dispatcher.
/// No need to declare the function as `extern "C"` the macro will
/// take care of this. Also, no need to add the `#[no_mangle]` attribute, as
/// the macro will do this too. And finally, no need to add the `#[link_section]` or `#allow(dead_code)`
/// attributes, as the macro will do this too.
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
    let entry_ident = format_ident!("MERLIN_SYSCALL_ENTRY_{}", fn_name);
    let name_tokens = padded.iter().map(|b| quote! { #b });

    //let fn_vis = &input_fn.vis; // temporary commented out, until I update the kernel tests which still require an internal syscall to be declared as public
    let fn_vis = quote!(pub);
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_body = &fn_block;

    let gen = quote! {
        #(#fn_attrs)*
        #[allow(dead_code)]
        #[allow(unused)]
        #fn_vis extern "C" fn #fn_name(#fn_args) #fn_body

        #[link_section = ".merlin_syscall_entries"]
        #[used]
        #[no_mangle]
        pub static #entry_ident:  crate::kernel::syscalls::syscall_macro::StaticSysCallEntry =
            crate::kernel::syscalls::syscall_macro::StaticSysCallEntry {
                id: #syscall_id,
                name: &[#(#name_tokens),*],
                handler: #fn_name,
            };
    };

    gen.into()
}

// Syn parser for the macro arguments
use syn::{
    parse::{Parse, ParseStream},
    MetaNameValue, Result as SynResult,
};

/// This struct is used to parse the macro arguments.
/// So SysCallArgs is referred to the macro arguments
/// and not to the syscall arguments!
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
