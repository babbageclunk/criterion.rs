//! Contains macros which together define a benchmark harness that can be used
//! in place of the standard benchmark harness. This allows the user to run
//! Criterion.rs benchmarks with `cargo bench`.

/// Macro used to define a benchmark group.
/// 
/// This is used to define a benchmark group; a collection of related benchmarks
/// which share a common configuration. Accepts two forms which can be seen
/// below.
/// 
/// # Examples:
/// 
/// Complete form:
/// 
/// ```
/// # #[macro_use]
/// # extern crate criterion;
/// # use criterion::Criterion;
/// # fn bench_method1(c: &mut Criterion) {
/// # }
/// #
/// # fn bench_method2(c: &mut Criterion) {
/// # }
/// #
/// criterion_group!{
///     name = benches;
///     config = Criterion::default();
///     targets = bench_method1, bench_method2
/// }
/// #
/// # fn main() {}
/// ```
/// 
/// In this form, all of the options are clearly spelled out. This expands to
/// a function named benches, which uses the given config expression to create
/// an instance of the Criterion struct. This is then passed by mutable
/// reference to the targets.
/// 
/// Compact Form:
/// 
/// ```
/// # #[macro_use]
/// # extern crate criterion;
/// # use criterion::Criterion;
/// # fn bench_method1(c: &mut Criterion) {
/// # }
/// #
/// # fn bench_method2(c: &mut Criterion) {
/// # }
/// #
/// criterion_group!(benches, bench_method1, bench_method2);
/// #
/// # fn main() {}
/// ```
/// In this form, the first parameter is the name of the group and subsequent
/// parameters are the target methods. The Criterion struct will be created using
/// the `Criterion::default()` function. If you wish to customize the
/// configuration, use the complete form and provide your own configuration
/// function.
#[macro_export]
macro_rules! criterion_group {
    (name = $name:ident; config = $config:expr; targets = $( $target:path ),+ ) => {
        pub fn $name() {
            $(
                let mut criterion: Criterion = $config;
                criterion.configure_from_args();
                $target(&mut criterion);
            )+
        }
    };
    ($name:ident, $( $target:path ),+ ) => {
        criterion_group!{
            name = $name;
            config = Criterion::default();
            targets = $( $target ),+
        }
    }
}

/// Macro which expands to a main function which executes all of the given
/// benchmark groups.
#[macro_export]
macro_rules! criterion_main {
    ( $( $group:path ),+ ) => {
        fn main() {
            $(
                $group();
            )+
        }
    }
}