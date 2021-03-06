//! lint when items are used after statements

use rustc::lint::*;
use syntax::attr::*;
use syntax::ast::*;
use utils::in_macro;

/// **What it does:** It `Warn`s on blocks where there are items that are declared in the middle of
/// or after the statements
///
/// **Why is this bad?** Items live for the entire scope they are declared in. But statements are
/// processed in order. This might cause confusion as it's hard to figure out which item is meant
/// in a statement.
///
/// **Known problems:** None
///
/// **Example:**
/// ```rust
/// fn foo() {
///     println!("cake");
/// }
/// fn main() {
///     foo(); // prints "foo"
///     fn foo() {
///         println!("foo");
///     }
///     foo(); // prints "foo"
/// }
/// ```
declare_lint! { pub ITEMS_AFTER_STATEMENTS, Warn, "finds blocks where an item comes after a statement" }

pub struct ItemsAfterStatemets;

impl LintPass for ItemsAfterStatemets {
    fn get_lints(&self) -> LintArray {
        lint_array!(ITEMS_AFTER_STATEMENTS)
    }
}

impl EarlyLintPass for ItemsAfterStatemets {
    fn check_block(&mut self, cx: &EarlyContext, item: &Block) {
        if in_macro(cx, item.span) {
            return;
        }
        let mut stmts = item.stmts.iter().map(|stmt| &stmt.node);
        // skip initial items
        while let Some(&StmtDecl(ref decl, _)) = stmts.next() {
            if let DeclLocal(_) = decl.node {
                break;
            }
        }
        // lint on all further items
        for stmt in stmts {
            if let StmtDecl(ref decl, _) = *stmt {
                if let DeclItem(ref it) = decl.node {
                    if in_macro(cx, it.span) {
                        return;
                    }
                    cx.struct_span_lint(ITEMS_AFTER_STATEMENTS, it.span,
                                        "adding items after statements is confusing, since items exist from the start of the scope")
                      .emit();
                }
            }
        }
    }
}
