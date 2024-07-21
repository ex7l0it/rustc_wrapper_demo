#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;

use rustc_driver::Compilation;
use rustc_interface::{interface::Compiler, Queries};

pub struct CompilerCalls {}

impl CompilerCalls {
    pub fn new() -> Self {
        CompilerCalls {}
    }
}

impl rustc_driver::Callbacks for CompilerCalls {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let _root_crate = tcx.hir().krate();
        });

        Compilation::Stop
    }
}

pub fn run_compiler(
    args: Vec<String>,
    callbacks: &mut (dyn rustc_driver::Callbacks + Send),
) -> i32 {
    rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, callbacks).run()
    })
}
