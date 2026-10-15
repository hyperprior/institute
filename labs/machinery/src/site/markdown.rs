//! Markdown → HTML, with `$…$` / `$$…$$` math rendered to native MathML.

use latex2mathml::{DisplayStyle, latex_to_mathml};
use pulldown_cmark::{Event, Options, Parser, html};

use super::escape;

pub fn render(md: &str) -> String {
    let options = Options::ENABLE_MATH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH;
    let events = Parser::new_ext(md, options).map(|event| match event {
        Event::InlineMath(tex) => Event::InlineHtml(math(&tex, DisplayStyle::Inline).into()),
        Event::DisplayMath(tex) => Event::InlineHtml(math(&tex, DisplayStyle::Block).into()),
        other => other,
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
    out
}

fn math(tex: &str, style: DisplayStyle) -> String {
    latex_to_mathml(tex, style)
        .unwrap_or_else(|_| format!("<code class=\"math-error\">{}</code>", escape(tex)))
}
