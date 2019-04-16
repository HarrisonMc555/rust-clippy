//! lint on using `x.get(x.len() - 1)` instead of `x.last()`

use crate::utils::{match_type, paths, span_lint_and_sugg,
                   snippet_with_applicability, SpanlessEq};
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::{declare_tool_lint, lint_array};
use rustc::hir::{Expr, ExprKind};
use rustc_errors::Applicability;
use syntax::ast::{LitKind};
use if_chain::if_chain;

/// **What it does:** Checks for using `x.get(x.len() - 1)` instead of `x.last()`.
///
/// **Why is this bad?** Using `x.last()` is easier to read and has the same result.
///
/// Note that using `x[x.len() - 1]` is semantically different from `x.last()`.
/// Indexing into the array will panic on out-of-bounds accesses, while
/// `x.get()` and `x.last()` will return `None`.
///
/// There is another lint (get_unwrap) that covers the case of using
/// `x.get(index).unwrap()` instead of `x[index]`.
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
            // Is a method call
            if let ExprKind::MethodCall(ref path, _, ref args) = expr.node;

            // Method name is "get"
            if path.ident.name == "get";

            // Argument 0 (the struct we're calling the method on) is a vector
            if let Some(struct_calling_on) = args.get(0);
            let struct_ty = cx.tables.expr_ty(struct_calling_on);
            if match_type(cx, struct_ty, &paths::VEC);

            // Argument to "get" is a binary operation
            if let Some(get_index_arg) = args.get(1);
            if let rustc::hir::ExprKind::Binary(ref op, ref lhs, ref rhs) = get_index_arg.node;

            // Binary operation is a subtraction
            if op.node == rustc::hir::BinOpKind::Sub;

            // LHS of subtraction is "x.len()"
            if let ExprKind::MethodCall(ref arg_lhs_path, _, ref lhs_args) = lhs.node;
            if arg_lhs_path.ident.name == "len";
            if let Some(arg_lhs_struct) = lhs_args.get(0);

            if SpanlessEq::new(cx).eq_expr(struct_calling_on, arg_lhs_struct);

            // RHS of subtraction is 1
            if let ExprKind::Lit(ref rhs_lit) = rhs.node;
            if let LitKind::Int(rhs_value, ..) = rhs_lit.node;
            if rhs_value == 1;

            let mut applicability = Applicability::MachineApplicable;
            let vec_name = snippet_with_applicability(
                cx, struct_calling_on.span, "x", &mut applicability);

            then {
                span_lint_and_sugg(
                    cx,
                    USE_LAST,
                    expr.span,
                    &format!("Use `{}.last()` instead of `{}.get({}.len() - 1)`",
                             vec_name, vec_name, vec_name),
                    "try",
                    format!("{}.last()", vec_name),
                    applicability,
                );
            }
        }
    }
}
