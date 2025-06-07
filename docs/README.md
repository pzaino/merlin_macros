# merlin_macros

This file allows the use of the `#[merlin_syscall]` attribute macro.
It is used to define a syscall entry point in the kernel.

The macro takes an `id` argument, which is the syscall ID.
The macro generates a static syscall entry point with the given ID and the
function name as the syscall name.

The function name is padded to 32 bytes with null bytes.
The generated entry point is placed in the `.merlin_syscall_entries` section
and is marked as used to prevent the linker from removing it.

The generated entry point is also marked as no_mangle to prevent name
mangling, so that it can be called from user space and C or Assembly code.
The mentioning of calling from user space is because this macro is used
to map private ABI syscalls to public ABI syscalls.

SysCalls defined using this macro are automatically registered in the Public ABI context
and can be used by user space applications.

License: MPL-2.0
