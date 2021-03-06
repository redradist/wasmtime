mod cli_tests;
mod custom_signal_handler;
mod debug;
mod externals;
mod func;
mod fuzzing;
mod globals;
mod iloop;
mod import_calling_export;
mod import_indexes;
mod instance;
mod invoke_func_via_table;
mod linker;
mod memory_creator;
mod name;
mod stack_overflow;
mod table;
mod traps;
mod use_after_drop;
mod wast;

// TODO(#1886): Cranelift only supports reference types on x64.
#[cfg(target_arch = "x86_64")]
mod gc;
