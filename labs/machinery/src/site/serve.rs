//! A tiny preview server: serves `dist/` and rebuilds when sources change.

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime},
};

use anyhow::Result;

use crate::institute::Institute;

pub fn serve(root: &Path, port: u16) -> Result<()> {
    let dist = root.join("dist");
    rebuild(root);
    let watched = root.to_path_buf();
    thread::spawn(move || {
        let mut last = stamp(&watched);
        loop {
            thread::sleep(Duration::from_millis(400));
            let now = stamp(&watched);
            if now != last {
                last = now;
                rebuild(&watched);
            }
        }
    });

    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("serving http://localhost:{port}  (drafts included, ctrl-c to stop)");
    for stream in listener.incoming().flatten() {
        if let Err(err) = respond(stream, &dist) {
            eprintln!("request failed: {err}");
        }
    }
    Ok(())
}

fn rebuild(root: &Path) {
    let result =
        Institute::load(root, true).and_then(|inst| super::build(&inst, &root.join("dist")));
    match result {
        Ok(()) => println!("built"),
        Err(err) => eprintln!("build failed: {err:#}"),
    }
}

/// Newest modification time and file count across the content sources.
fn stamp(root: &Path) -> (Option<SystemTime>, usize) {
    fn walk(path: &Path, acc: &mut (Option<SystemTime>, usize)) {
        if path.ends_with("target") || path.ends_with("data") {
            return;
        }
        if let Ok(meta) = fs::metadata(path) {
            acc.1 += 1;
            acc.0 = acc.0.max(meta.modified().ok());
            if meta.is_dir()
                && let Ok(items) = fs::read_dir(path)
            {
                for item in items.flatten() {
                    walk(&item.path(), acc);
                }
            }
        }
    }
    let mut acc = (None, 0);
    for rel in ["labs", "sparks.toml", "about.md"] {
        walk(&root.join(rel), &mut acc);
    }
    acc
}

fn respond(mut stream: TcpStream, dist: &Path) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut request = String::new();
    reader.read_line(&mut request)?;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? <= 2 {
            break;
        }
    }
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let path = path.split(['?', '#']).next().unwrap_or("/");

    let (status, headers, body) = if path.split('/').any(|seg| seg == "..") {
        ("400 Bad Request", String::new(), Vec::new())
    } else {
        let file: PathBuf = dist.join(path.trim_start_matches('/'));
        if file.is_dir() && !path.ends_with('/') {
            (
                "301 Moved Permanently",
                format!("Location: {path}/\r\n"),
                Vec::new(),
            )
        } else {
            let file = if file.is_dir() {
                file.join("index.html")
            } else {
                file
            };
            match fs::read(&file) {
                Ok(body) => (
                    "200 OK",
                    format!("Content-Type: {}\r\n", content_type(&file)),
                    body,
                ),
                Err(_) => (
                    "404 Not Found",
                    "Content-Type: text/html; charset=utf-8\r\n".into(),
                    fs::read(dist.join("404.html")).unwrap_or_default(),
                ),
            }
        }
    };
    write!(
        stream,
        "HTTP/1.1 {status}\r\n{headers}Content-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(&body)?;
    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("xml") => "application/atom+xml; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
