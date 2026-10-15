//! Atom feed of the log.

use super::{SITE_URL, entry_url, escape, markdown};
use crate::institute::Institute;

pub fn atom(inst: &Institute) -> String {
    let entries: Vec<_> = inst.entries().into_iter().take(30).collect();
    let updated = entries
        .first()
        .map_or("1970-01-01", |(_, e)| e.date.as_str());
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\">\n\
         <title>Hyperprior Institute</title>\n<subtitle>A lab that builds labs that builds labs.</subtitle>\n\
         <id>{SITE_URL}/</id>\n<link href=\"{SITE_URL}/\"/>\n<link rel=\"self\" href=\"{SITE_URL}/feed.xml\"/>\n\
         <author><name>hyperprior</name></author>\n<updated>{updated}T00:00:00Z</updated>\n"
    );
    for (lab, entry) in entries {
        let url = format!("{SITE_URL}{}", entry_url(&lab.name, &entry.slug));
        out += &format!(
            "<entry>\n<title>{title}</title>\n<id>{url}</id>\n<link href=\"{url}\"/>\n\
             <updated>{date}T00:00:00Z</updated>\n<category term=\"{lab}\"/>\n\
             <content type=\"html\">{content}</content>\n</entry>\n",
            title = escape(&entry.title),
            date = entry.date,
            lab = escape(&lab.name),
            content = escape(&markdown::render(&entry.body)),
        );
    }
    out + "</feed>\n"
}
