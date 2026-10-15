//! The site: a static view of the repo, rendered to `dist/`. No framework, no JS.

mod feed;
mod lineage;
mod markdown;
mod pages;
pub mod serve;

use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::institute::Institute;

pub const SITE_URL: &str = "https://hyperprior.institute";
pub const REPO_URL: &str = "https://github.com/hyperprior/institute";

const STYLE: &str = include_str!("../../static/style.css");
const LOGO: &str = include_str!("../../static/logo.svg");

pub fn build(inst: &Institute, out: &Path) -> Result<()> {
    if out.exists() {
        fs::remove_dir_all(out).with_context(|| format!("clearing {}", out.display()))?;
    }
    write(out, "style.css", STYLE)?;
    write(out, "logo.svg", LOGO)?;
    write(out, "index.html", &pages::home(inst))?;
    write(out, "log/index.html", &pages::log(inst))?;
    write(out, "labs/index.html", &pages::labs(inst))?;
    write(out, "lineage/index.html", &pages::lineage(inst))?;
    write(out, "sparks/index.html", &pages::sparks(inst))?;
    write(out, "404.html", &pages::not_found())?;
    write(out, "feed.xml", &feed::atom(inst))?;

    for lab in &inst.labs {
        write(
            out,
            &format!("labs/{}/index.html", lab.name),
            &pages::lab(inst, lab),
        )?;
        for entry in &lab.entries {
            let page = pages::entry(lab, entry);
            write(
                out,
                &format!("labs/{}/{}/index.html", lab.name, entry.slug),
                &page,
            )?;
        }
        let figures = lab.dir.join("figures");
        if figures.is_dir() {
            copy_dir(&figures, &out.join("labs").join(&lab.name).join("figures"))?;
        }
    }
    Ok(())
}

fn write(out: &Path, rel: &str, contents: &str) -> Result<()> {
    let path = out.join(rel);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for item in fs::read_dir(from)? {
        let path = item?.path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

pub(crate) fn lab_url(name: &str) -> String {
    format!("/labs/{name}/")
}

pub(crate) fn entry_url(lab: &str, slug: &str) -> String {
    format!("/labs/{lab}/{slug}/")
}

pub(crate) fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
