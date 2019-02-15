//! lint on using `x.get(x.len() - 1)` instead of `x.last()`

// use crate::utils::{in_macro, snippet_opt, last_path_segment, match_def_path, paths, snippet, span_lint, span_lint_and_then};
// use rustc::lint::{in_external_macro, LateContext, LateLintPass, LintArray, LintContext, LintPass};
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::{declare_tool_lint, lint_array};
// use if_chain::if_chain;
// use syntax::ast::*;
use rustc::hir::{PatKind, BindingAnnotation, Expr, ExprKind};
// use crate::utils::{opt_def_id, sugg};
// use crate::utils::{span_lint, snippet};
use crate::utils::span_lint;
use if_chain::if_chain;
// use rustc::ty::{self, Ty};
// use rustc_errors::Applicability;
// use std::borrow::Cow;
// use syntax::ast;

/// **What it does:** Checks for using `x.get(x.len() - 1)` instead of `x.last()`.
///
/// **Why is this bad?** Using `x.last()` is easier to read and has the same result.
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
    fn check_stmt(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        if_chain! {
            if let ExprKind::MethodCall(ref path, _, ref args) = expr.node;
            // check if vector
            // TODO: check if vector
            // check if calling 'get' method
            if path.ident.name.as_str() == "get";
            // if let ExprKind::MethodCall(ref method_name, ref generics, ref args) = init.node;
            // if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, _, None) = local.pat.node;
            // if name.node.as_str() == "last_element";
            then {
                span_lint(cx,
                          USE_LAST,
                          stmt.span,
                          // Todo: fix this
                          &format!("Use `x.last()` instead of `x.get(x.len() - 1)`"));
            }
        }
    }
}
