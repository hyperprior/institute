//! The lineage tree as an inline SVG, styled by the site CSS.

use std::collections::HashMap;

use super::{escape, lab_url};
use crate::institute::{Institute, Lab};

const NODE_W: usize = 190;
const NODE_H: usize = 30;
const INDENT: usize = 28;
const ROW: usize = 42;
const PAD: usize = 2;

pub fn svg(inst: &Institute) -> String {
    // Pre-order walk from each root: each project gets a row, indented by depth.
    let mut rows: Vec<(usize, &Lab)> = Vec::new();
    fn visit<'a>(
        inst: &'a Institute,
        lab: &'a Lab,
        depth: usize,
        rows: &mut Vec<(usize, &'a Lab)>,
    ) {
        rows.push((depth, lab));
        for child in inst.children(&lab.name) {
            visit(inst, child, depth + 1, rows);
        }
    }
    for root in inst.labs.iter().filter(|lab| lab.manifest.parent.is_none()) {
        visit(inst, root, 0, &mut rows);
    }

    let row_of: HashMap<&str, usize> = rows
        .iter()
        .enumerate()
        .map(|(i, (_, l))| (l.name.as_str(), i))
        .collect();
    let max_depth = rows.iter().map(|(d, _)| *d).max().unwrap_or(0);
    let width = max_depth * INDENT + NODE_W + 2 * PAD;
    let height = rows.len().max(1) * ROW - (ROW - NODE_H) + 2 * PAD;

    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" role=\"img\" aria-label=\"Project lineage\">"
    );
    for (i, (depth, lab)) in rows.iter().enumerate() {
        if let Some(parent_row) = lab.manifest.parent.as_deref().and_then(|p| row_of.get(p)) {
            let x = PAD + (depth - 1) * INDENT + INDENT / 2;
            let y0 = PAD + parent_row * ROW + NODE_H;
            let y1 = PAD + i * ROW + NODE_H / 2;
            let x1 = PAD + depth * INDENT;
            out += &format!("<path class=\"edge\" d=\"M{x} {y0} V{y1} H{x1}\"/>");
        }
    }
    for (i, (depth, lab)) in rows.iter().enumerate() {
        let x = PAD + depth * INDENT;
        let y = PAD + i * ROW;
        let stage = lab.manifest.stage;
        out += &format!(
            "<a href=\"{href}\"><g class=\"node stage-{stage}\"><rect x=\"{x}\" y=\"{y}\" width=\"{NODE_W}\" height=\"{NODE_H}\" rx=\"4\"/><text x=\"{tx}\" y=\"{ty}\">{name}</text><text class=\"node-stage\" x=\"{sx}\" y=\"{ty}\" text-anchor=\"end\">{stage}</text></g></a>",
            href = lab_url(&lab.name),
            tx = x + 10,
            sx = x + NODE_W - 10,
            ty = y + NODE_H / 2 + 5,
            name = escape(&lab.name),
        );
    }
    out + "</svg>"
}
