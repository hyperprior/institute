//! Page templates.

use maud::{DOCTYPE, Markup, PreEscaped, html};

use super::{REPO_URL, entry_url, lab_url, lineage, markdown};
use crate::institute::{Entry, Institute, Lab, split_title};
use crate::manifest::Stage;

fn layout(title: &str, body: Markup) -> String {
    let full = if title.is_empty() {
        "Hyperprior Institute".to_string()
    } else {
        format!("{title} · Hyperprior Institute")
    };
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (full) }
                link rel="stylesheet" href="/style.css";
                link rel="icon" href="/logo.svg" type="image/svg+xml";
                link rel="alternate" type="application/atom+xml" title="Hyperprior Institute" href="/feed.xml";
            }
            body {
                header.site {
                    a.brand href="/" {
                        img src="/logo.svg" alt="" width="28" height="28";
                        span { "Hyperprior Institute" }
                    }
                    nav {
                        a href="/log/" { "Log" }
                        a href="/labs/" { "Labs" }
                        a href="/lineage/" { "Lineage" }
                        a href="/sparks/" { "Sparks" }
                    }
                }
                main { (body) }
                footer.site {
                    p {
                        "A lab that builds labs that builds labs. "
                        a href=(REPO_URL) { "Source" } " · " a href="/feed.xml" { "Feed" }
                    }
                }
            }
        }
    }
    .into_string()
}

fn stage_badge(stage: Stage) -> Markup {
    html! { span class=(format!("stage stage-{stage}")) { (stage) } }
}

fn entry_list(entries: &[(&Lab, &Entry)], show_lab: bool) -> Markup {
    html! {
        ol.entries {
            @for (lab, entry) in entries {
                li {
                    time datetime=(entry.date) { (entry.date) }
                    span {
                        a href=(entry_url(&lab.name, &entry.slug)) { (entry.title) }
                        @if entry.draft { " " span.draft { "draft" } }
                        @if show_lab { " " a.lab-tag href=(lab_url(&lab.name)) { (lab.name) } }
                    }
                }
            }
        }
    }
}

fn lab_list<'a>(labs: impl Iterator<Item = &'a Lab>) -> Markup {
    html! {
        ul.labs {
            @for lab in labs {
                li {
                    a href=(lab_url(&lab.name)) { (lab.manifest.title) }
                    " " (stage_badge(lab.manifest.stage))
                    p { (lab.manifest.summary) }
                }
            }
        }
    }
}

pub fn home(inst: &Institute) -> String {
    let (title, intro) = split_title(&inst.about);
    let recent: Vec<_> = inst.entries().into_iter().take(8).collect();
    let active = inst
        .labs
        .iter()
        .filter(|lab| matches!(lab.manifest.stage, Stage::Sketch | Stage::Lab));
    layout(
        "",
        html! {
            section.intro {
                h1 { (title.unwrap_or_else(|| "Hyperprior Institute".into())) }
                (PreEscaped(markdown::render(&intro)))
            }
            h2 { "Recently in the log" }
            (entry_list(&recent, true))
            h2 { "Active work" }
            (lab_list(active))
        },
    )
}

pub fn log(inst: &Institute) -> String {
    layout(
        "Log",
        html! {
            h1 { "Log" }
            p.lede { "Everything written at the institute, newest first." }
            (entry_list(&inst.entries(), true))
        },
    )
}

pub fn labs(inst: &Institute) -> String {
    let groups = [
        (Stage::Lab, "Labs", "Lines of work that stuck."),
        (
            Stage::Sketch,
            "Sketches",
            "Running, but not yet sure they'll last.",
        ),
        (
            Stage::Library,
            "Libraries",
            "Spun out into their own repos.",
        ),
        (
            Stage::Buried,
            "Graveyard",
            "Done or abandoned, with a write-up and a decent burial.",
        ),
    ];
    layout(
        "Labs",
        html! {
            h1 { "Labs" }
            @for (stage, heading, blurb) in groups {
                @if inst.labs_in(stage).next().is_some() {
                    h2 { (heading) }
                    p.lede { (blurb) }
                    (lab_list(inst.labs_in(stage)))
                }
            }
        },
    )
}

pub fn lab(inst: &Institute, lab: &Lab) -> String {
    let m = &lab.manifest;
    let source = m
        .repo
        .clone()
        .unwrap_or_else(|| format!("{REPO_URL}/tree/main/labs/{}", lab.name));
    let children: Vec<_> = inst.children(&lab.name).collect();
    let entries: Vec<_> = lab.entries.iter().map(|e| (lab, e)).collect();
    layout(
        &m.title,
        html! {
            h1 { (m.title) " " (stage_badge(m.stage)) }
            p.lede { (m.summary) }
            dl.meta {
                dt { "started" } dd { time datetime=(m.started) { (m.started) } }
                @if let Some(parent) = &m.parent {
                    dt { "grew from" } dd { a href=(lab_url(parent)) { (parent) } }
                }
                @if !children.is_empty() {
                    dt { "grew into" }
                    dd {
                        @for (i, child) in children.iter().enumerate() {
                            @if i > 0 { ", " }
                            a href=(lab_url(&child.name)) { (child.name) }
                        }
                    }
                }
                dt { "source" } dd { a href=(source) { (source.trim_start_matches("https://")) } }
            }
            h2 { "Log" }
            (entry_list(&entries, false))
        },
    )
}

pub fn entry(lab: &Lab, entry: &Entry) -> String {
    layout(
        &entry.title,
        html! {
            article {
                header {
                    h1 { (entry.title) }
                    p.meta {
                        time datetime=(entry.date) { (entry.date) }
                        " · " a href=(lab_url(&lab.name)) { (lab.manifest.title) }
                        " " (stage_badge(lab.manifest.stage))
                        @if entry.draft { " " span.draft { "draft" } }
                    }
                }
                (PreEscaped(markdown::render(&entry.body)))
            }
        },
    )
}

pub fn lineage(inst: &Institute) -> String {
    layout(
        "Lineage",
        html! {
            h1 { "Lineage" }
            p.lede { "What grew out of what. Every project records its parent; this is the tree they make." }
            figure.lineage { (PreEscaped(lineage::svg(inst))) }
        },
    )
}

pub fn sparks(inst: &Institute) -> String {
    layout(
        "Sparks",
        html! {
            h1 { "Sparks" }
            p.lede { "One-line ideas. Writing one down costs nothing and obliges nothing." }
            @if inst.sparks.is_empty() { p { "None right now." } }
            ol.entries {
                @for spark in &inst.sparks {
                    li { time datetime=(spark.date) { (spark.date) } span { (spark.text) } }
                }
            }
        },
    )
}

pub fn not_found() -> String {
    layout(
        "Not found",
        html! {
            h1 { "Not found" }
            p { "Nothing here. Maybe it was buried. Try the " a href="/labs/" { "labs" } "." }
        },
    )
}
