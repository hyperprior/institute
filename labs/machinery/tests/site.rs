//! End to end: scaffold an institute, build the site, and check what comes out.

use std::{fs, path::Path};

use hp::{institute::Institute, scaffold, site};

fn institute() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("sparks.toml"), "").unwrap();
    fs::write(root.join("about.md"), "# Test Institute\n\nHello.\n").unwrap();
    let base = root.join("labs/base");
    fs::create_dir_all(base.join("log")).unwrap();
    fs::create_dir_all(base.join("figures")).unwrap();
    fs::write(
        base.join("lab.toml"),
        "title = \"Base\"\nstage = \"lab\"\nstarted = \"2026-10-01\"\nsummary = \"The root.\"\n",
    )
    .unwrap();
    fs::write(
        base.join("log/2026-10-02-math.md"),
        "# Some math\n\nInline $x^2$ and display:\n\n$$\\sum_i x_i$$\n\n![fig](../figures/plot.svg)\n",
    )
    .unwrap();
    fs::write(base.join("log/_2026-10-03-unfinished.md"), "# Not yet\n").unwrap();
    fs::write(
        base.join("figures/plot.svg"),
        "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
    )
    .unwrap();
    scaffold::add_spark(root, "a \"quoted\" idea", "2026-10-04").unwrap();
    scaffold::new_sketch(root, "child", Some("base"), "2026-10-05").unwrap();
    dir
}

#[test]
fn scaffold_and_load() {
    let dir = institute();
    let inst = Institute::load(dir.path(), false).unwrap();
    assert_eq!(inst.sparks[0].text, "a \"quoted\" idea");
    let child = inst.lab("child").unwrap();
    assert_eq!(child.manifest.parent.as_deref(), Some("base"));
    assert_eq!(child.entries[0].title, "Starting child");
    assert!(dir.path().join("labs/child/src/main.rs").is_file());
    assert!(scaffold::new_sketch(dir.path(), "child", None, "2026-10-05").is_err());
    assert!(scaffold::new_sketch(dir.path(), "Bad_Name", None, "2026-10-05").is_err());
    assert!(scaffold::new_sketch(dir.path(), "orphan", Some("nope"), "2026-10-05").is_err());
}

#[test]
fn drafts_only_when_asked() {
    let dir = institute();
    let titles = |drafts| -> Vec<String> {
        let inst = Institute::load(dir.path(), drafts).unwrap();
        inst.lab("base")
            .unwrap()
            .entries
            .iter()
            .map(|e| e.title.clone())
            .collect()
    };
    assert_eq!(titles(false), ["Some math"]);
    assert_eq!(titles(true), ["Not yet", "Some math"]);
}

#[test]
fn missing_parent_is_an_error() {
    let dir = institute();
    let manifest = dir.path().join("labs/child/lab.toml");
    let text = fs::read_to_string(&manifest)
        .unwrap()
        .replace("\"base\"", "\"ghost\"");
    fs::write(&manifest, text).unwrap();
    assert!(Institute::load(dir.path(), false).is_err());
}

#[test]
fn built_site_is_whole() {
    let dir = institute();
    let inst = Institute::load(dir.path(), false).unwrap();
    let out = dir.path().join("dist");
    site::build(&inst, &out).unwrap();

    let entry = fs::read_to_string(out.join("labs/base/2026-10-02-math/index.html")).unwrap();
    assert!(entry.contains("<math"), "math should render to MathML");
    assert!(!out.join("labs/base/2026-10-03-unfinished").exists());

    // Every internal link resolves, including relative figure links.
    let mut checked = 0;
    for page in html_files(&out) {
        let html = fs::read_to_string(&page).unwrap();
        for target in links(&html) {
            if target.starts_with("http") || target.starts_with('#') {
                continue;
            }
            let path = if let Some(abs) = target.strip_prefix('/') {
                out.join(abs)
            } else {
                page.parent().unwrap().join(target)
            };
            let path = if target.ends_with('/') {
                path.join("index.html")
            } else {
                path
            };
            assert!(
                path.exists(),
                "{} links to missing {target}",
                page.display()
            );
            checked += 1;
        }
    }
    assert!(checked > 20);

    // The feed is well-formed XML.
    let feed = fs::read_to_string(out.join("feed.xml")).unwrap();
    let mut reader = quick_xml::Reader::from_str(&feed);
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(_) => {}
            Err(err) => panic!("feed.xml is not well-formed: {err}"),
        }
    }
    assert!(feed.contains("Some math"));
}

fn html_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for item in fs::read_dir(dir).unwrap() {
        let path = item.unwrap().path();
        if path.is_dir() {
            files.extend(html_files(&path));
        } else if path.extension().is_some_and(|e| e == "html") {
            files.push(path);
        }
    }
    files
}

fn links(html: &str) -> Vec<&str> {
    let mut found = Vec::new();
    for attr in ["href=\"", "src=\""] {
        for (i, _) in html.match_indices(attr) {
            let rest = &html[i + attr.len()..];
            found.push(&rest[..rest.find('"').unwrap()]);
        }
    }
    found
}
