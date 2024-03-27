use std::str::Split;

use include_dir::Dir;
use once_cell::sync::Lazy;

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct ContentEntry {
    pub title: &'static str,
    pub date: &'static str,
    pub image: &'static str,
    pub description: &'static str,
    pub content: &'static str, // won't get generated until requested at the blog post level
    pub archetype: &'static str,
    pub slug: &'static str,
    pub link: Option<&'static str>,
    pub weight: u8,
}

pub static BLOG_POSTS: Lazy<Vec<ContentEntry>> = Lazy::new(|| {
    static DIR: Dir<'static> = include_dir::include_dir!("content/blog");
    parse_dir(&DIR, "blog")
});

// pub static PORTFOLIO_ENTRIES: Lazy<Vec<ContentEntry>> = Lazy::new(|| {
//     static DIR: Dir<'static> = include_dir::include_dir!("content/portfolio");
//     parse_dir(&DIR, "portfolio")
// });

fn parse_dir(dir: &'static Dir<'static>, archetype: &'static str) -> Vec<ContentEntry> {
    dir.files()
        .map(|file| {
            let buf = file.contents_utf8().unwrap();
            let mut split = buf.split("+++");

            let _ = split.next().unwrap();
            let head = split.next().unwrap();
            let content = split.next().unwrap();

            let mut entry = ContentEntry {
                archetype,
                slug: file.path().to_str().unwrap().trim_end_matches(".md"),
                description: "",
                content,
                ..ContentEntry::default()
            };

            for line in head.lines() {
                let mut parts = line.splitn(2, " = ");
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    match key {
                        "date" => entry.date = value.trim_matches('"'),
                        "image" => entry.image = value.trim_matches('"'),
                        "title" => entry.title = value.trim_matches('"'),
                        "weight" => entry.weight = value.parse().unwrap(),
                        "description" => entry.description = value.trim_matches('"'),
                        "link" => entry.link = Some(value.trim_matches('"')),
                        "showonlyimage" => {}
                        "generatepage" => {}
                        _ => {}
                    }
                }
            }
            entry
        })
        .collect()
}
