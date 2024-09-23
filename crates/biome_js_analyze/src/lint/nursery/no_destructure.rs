use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow destructuring props.
    ///
    /// In Solid, props must be used with property accesses (props.foo) to preserve reactivity. This rule only tracks destructuring in the parameter list.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const Child = ({page}) => {
    /// return (
    ///     <div>
    ///         <p>{page}</p>
    ///     </div>
    /// );
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const egg = (props) => {
    /// return (
    ///     <div>
    ///         <p>{props.page}</p>
    ///     </div>
    /// );
    /// };
    /// ```
    ///
    pub NoDestructure {
        version: "next",
        name: "noDestructure",
        language: "jsx",
        recommended: false,
    }
}

impl Rule for NoDestructure {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();


fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    let binding = ctx.query();

    return Some(())
}
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    ""
                },
            )
            .note(markup! {
                ""
            }),
        )
    }
}
