# ICE when targetting WASM and re-exporting renamed proc-macro

Reproduce:

```
$ cargo check --target=wasm32-unknown-unknown

    Checking proc-macro-reexport v0.1.0 (/home/tomusdrw/workspace/rust-ice/proc-macro-reexport)
thread 'rustc' panicked at 'src/librustc_resolve/resolve_imports.rs:854: inconsistent resolution for an import', src/librustc/util/bug.rs:37:26
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  10: rustc::ty::context::tls::with_opt::{{closure}}
  11: rustc::ty::context::tls::with_context_opt
  12: rustc::ty::context::tls::with_opt
  13: rustc::util::bug::opt_span_bug_fmt
  14: rustc::util::bug::span_bug_fmt
  15: rustc_resolve::resolve_imports::ImportResolver::finalize_import
  16: rustc_resolve::resolve_imports::ImportResolver::finalize_imports
  17: rustc_resolve::Resolver::resolve_crate
  18: rustc::util::common::time
  19: rustc_interface::passes::configure_and_expand_inner
  20: rustc_interface::passes::configure_and_expand::{{closure}}
  21: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  22: rustc_interface::passes::configure_and_expand
  23: rustc_interface::queries::Query<T>::compute
  24: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::expansion
  25: rustc_interface::interface::run_compiler_in_existing_thread_pool
  26: std::thread::local::LocalKey<T>::with
  27: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0 (625451e37 2019-09-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `proc-macro-reexport`.

To learn more, run the command again with --verbose.
```
