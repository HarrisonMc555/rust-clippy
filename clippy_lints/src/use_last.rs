//! lint on using `x.get(x.len() - 1)` instead of `x.last()`

use crate::utils::{match_type, paths, span_lint};
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::{declare_tool_lint, lint_array};
use rustc::hir::{Expr, ExprKind};
use syntax::ast::{LitKind};
use if_chain::if_chain;

/// **What it does:** Checks for using `x.get(x.len() - 1)` instead of `x.last()`.
///
/// **Why is this bad?** Using `x.last()` is easier to read and has the same result.
///
/// Note that using x[x.len() - 1] is semantically different from x.last(),
/// since indexing into the array will panic on out-of-bounds accesses, while
/// x.get() and x.last() will return None.
///
/// **Known problems:** None.
///
/// **Example:**
///
/// ```rust
/// // Bad
/// let x = vec![2, 3, 5];
/// let last_element = x.get(x.len() - 1);
///
/// // Good
/// let x = vec![2, 3, 5];
/// let last_element = x.last();
/// ```

declare_clippy_lint! {
    pub USE_LAST,
    complexity,
    "using `x.get(x.len() - 1)` instead of `x.last()`"
}

#[derive(Copy, Clone, Debug)]
pub struct UseLast;

impl LintPass for UseLast {
    fn get_lints(&self) -> LintArray {
        lint_array!(USE_LAST)
    }

    fn name(&self) -> &'static str {
        "UseLast"
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for UseLast {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        if_chain! {
            // let _ = println!("Starting UseLast");
            // Is a method call
            if let ExprKind::MethodCall(ref path, _, ref args) = expr.node;
            // let _ = println!("It is a MethodCall");

            // Method name is "get"
            if path.ident.name == "get";
            // let _ = println!("The name is get");

            // Argument 0 (the struct we're calling the method on) is a vector
            if let Some(struct_calling_on) = args.get(0);
            // let _ = println!("It had an argument");
            let struct_ty = cx.tables.expr_ty(struct_calling_on);
            if match_type(cx, struct_ty, &paths::VEC);
            // let _ = println!("It was a vector");

            // Argument to "get" is a binary operation
            if let Some(get_index_arg) = args.get(1);
            // let _ = println!("It had an argument");
            if let rustc::hir::ExprKind::Binary(ref op, ref lhs, ref rhs) = get_index_arg.node;
            // let _ = println!("It was a vector");

            // Binary operation is a subtraction
            if op.node == rustc::hir::BinOpKind::Sub;
            // let _ = println!("It was a subtraction");

            // LHS of subtraction is "x.len()"
            if let ExprKind::MethodCall(ref arg_lhs_path, _, ref lhs_args) = lhs.node;
            // let _ = println!("LHS of sub is a method call");
            if arg_lhs_path.ident.name == "len";
            // let _ = println!("LHS of sub was method named len");
            // if let Some(arg_lhs_struct) = lhs_args.get(0);
            // let _ = println!("LHS of sub method has an arg");
            // TODO: Is this a valid way to check if they reference the same vector?
            // if arg_lhs_struct.hir_id == struct_calling_on.hir_id;
            // let _ = println!("The vector in .get and .len were the same");

            // RHS of subtraction is 1
            if let ExprKind::Lit(ref rhs_lit) = rhs.node;
            // let _ = println!("RHS of sub was literal");
            if let LitKind::Int(rhs_value, ..) = rhs_lit.node;
            // let _ = println!("RHS of sub was int");
            if rhs_value == 1;
            // let _ = println!("RHS of sub was 1");

            // TODO: Figure out how to get name of variable for lint message
            // Can't do this (cannot move out of borrowed content (context?))
            // if let ExprKind::Struct(ref struct_calling_on_path, _, _) = struct_calling_on.node;
            // let _ = println!("It was a struct");
            // let vec_name = match struct_calling_on_path.into_inner() {
            //     rustc::hir::QPath::Resolved(_, path) =>
            //         path.segments.last().map(|path_seg| path_seg.ident.name.as_str().get()).unwrap_or("x"),
            //     rustc::hir::QPath::TypeRelative(_, path_seg) => path_seg.ident.name.as_str().get(),
            // };
            let vec_name = "x";
            // let _ = println!("About to span_lint on \"{}\"", vec_name);

            then {
                span_lint(cx,
                          USE_LAST,
                          expr.span,
                          &format!("Use `{}.last()` instead of `{}.get({}.len() - 1)`",
                                   vec_name, vec_name, vec_name));
            }
        }
    }
}
