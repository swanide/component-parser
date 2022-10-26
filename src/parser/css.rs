use crate::parser::meta::{CssClassMeta, CssMeta, Location};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::vec;

pub fn parse_css_class(file_path: &str) -> Result<CssMeta, String> {
    let read_result = fs::read_to_string(file_path);
    if let Err(_) = read_result {
        return Err(format!("No such file {}", file_path));
    }

    let text = read_result.unwrap();
    let regex = Regex::new(r"\.([a-z_][\w-]+)").unwrap();
    let regex_start = Regex::new(r"^[\w\s.,{}>+]$").unwrap();
    let regex_end = Regex::new(r"^[\s.:,{>+]$").unwrap();

    let mut last_line = 1;
    let mut last_offset = 0;
    let mut classes_set: HashSet<String> = HashSet::new();
    let mut classes = vec![];

    for caps in regex.captures_iter(&text) {
        let match0 = caps.get(0).unwrap();
        let start = match0.start();
        let end = match0.end();
        // front
        if start > 0 && !regex_start.is_match(text.get(start - 1..start).unwrap()) {
            continue;
        }
        // back
        if end < text.len() && !regex_end.is_match(text.get(end..end + 1).unwrap()) {
            continue;
        }

        let class_name = caps[1].to_string();
        let len = caps[1].len();
        if classes_set.contains(&class_name) {
            continue;
        }
        let pass_lines = text[last_offset..start]
            .chars()
            .filter(|c| c == &'\n')
            .count();
        let mut column = 0;
        if let Some(column_index) = text[..start].rfind('\n') {
            column += text[column_index..start].chars().count();
        }

        last_line += pass_lines;
        last_offset = end;

        classes_set.insert(class_name.to_string());

        let mut class_meta = CssClassMeta::new(class_name);
        class_meta.loc = Location::from([last_line, column], [last_line, column + len]);
        classes.push(class_meta);
    }

    // 解析 imports 地址
    let mut meta = CssMeta::new(classes);
    let regex = Regex::new(r#"@import\s+(?:url\()?["'](?P<url>[^"']+)["']"#).unwrap();
    let mut imports: Vec<String> = vec![];
    for caps in regex.captures_iter(&text) {
        let path = Path::new(file_path).parent().unwrap().join(&caps["url"]);
        let abs_path = path.canonicalize();

        if let Ok(p) = abs_path {
            imports.push(p.to_str().unwrap().to_string());
        }
    }
    meta.imports = Some(imports);

    Ok(meta)
}

#[test]
fn test_parse_css() {
    let css_meta = parse_css_class("test/fixtures/component.css").unwrap();
    assert_eq!(css_meta.classes.len(), 10);

    let class0 = css_meta.classes.get(0).unwrap();
    assert_eq!(class0.name, "class-0");
    assert_eq!(class0.loc, Location::from([1, 0], [1, 7]));

    let class2 = css_meta.classes.get(2).unwrap();
    assert_eq!(class2.name, "class-2");
    assert_eq!(class2.loc, Location::from([21, 13], [21, 20]));

    let class9 = css_meta.classes.get(9).unwrap();
    assert_eq!(class9.name, "class-9");
    assert_eq!(class9.loc, Location::from([30, 16], [30, 23]));

    let not_found = parse_css_class("test/fixtures/component-notfound.css").err();
    assert_eq!(
        Some(String::from(
            "No such file test/fixtures/component-notfound.css"
        )),
        not_found
    );

    let css_meta = parse_css_class("test/fixtures/import.css").unwrap();
    assert_eq!(css_meta.classes.len(), 1);
    assert_eq!(
        css_meta.imports,
        Some(vec![
            std::path::PathBuf::from("test/fixtures/component.css")
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            std::path::PathBuf::from("test/fixtures/page.css")
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        ])
    );
}
