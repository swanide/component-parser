#[macro_use]
extern crate serde_derive;
extern crate swc_common;
extern crate swc_ecma_parser;

mod parser;
use neon::prelude::*;
use parser::{parse_component, parse_css_class, ComponentMeta, CssMeta};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

/// 解析单个文件
fn parse_script(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path: Handle<JsString> = cx.argument(0)?;
    if let Ok(result) = parse_component(&file_path.value(&mut cx)) {
        let result = serde_json::to_string(&result).unwrap();
        return Ok(cx.string(result));
    }
    cx.throw_error("parse script error!")
}

fn parse_script_files_with_thread(file_paths: &Vec<String>) -> HashMap<String, ComponentMeta> {
    let mut result_map: HashMap<String, ComponentMeta> = HashMap::new();
    let size = file_paths.len();
    let mut threads = vec![];
    let (sender, receiver) = channel();
    for i in 0..size {
        let sender = sender.clone();
        let file_path = file_paths.get(i).unwrap().clone();
        threads.push(thread::spawn(move || {
            if let Ok(meta) = parse_component(&file_path) {
                sender.send((file_path, meta)).unwrap();
            }
        }));
    }
    drop(sender);
    // 接受数据
    for (file_path, meta) in receiver {
        result_map.insert(file_path, meta);
    }
    // 等待进程完毕
    for t in threads {
        t.join().unwrap();
    }
    result_map
}

/// 解析一组 js 文件
fn parse_script_files(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = arg0.to_vec(&mut cx).unwrap();
    let file_paths: Vec<String> = vec
        .iter()
        .map(|&v| v.to_string(&mut cx).unwrap().value(&mut cx))
        .collect();

    let result_map = parse_script_files_with_thread(&file_paths);
    let result = serde_json::to_string(&result_map).unwrap();
    return Ok(cx.string(result));
}

/// 解析单个 css 文件
fn parse_css(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path: Handle<JsString> = cx.argument(0)?;
    if let Ok(result) = parse_css_class(&file_path.value(&mut cx)) {
        let result = serde_json::to_string(&result).unwrap();
        return Ok(cx.string(result));
    }
    cx.throw_error("parse css error!")
}

fn parse_css_files_recrusive(file_paths: &Vec<String>, result_map: &mut HashMap<String, CssMeta>) {
    let size = file_paths.len();
    let mut threads = vec![];
    let (sx, rx) = channel();

    for i in 0..size {
        let sender = sx.clone();
        let file_path = file_paths.get(i).unwrap().clone();
        threads.push(thread::spawn(move || {
            if let Ok(meta) = parse_css_class(&file_path) {
                sender.send((file_path, meta)).unwrap();
            }
        }));
    }
    drop(sx);

    let mut import_paths: Vec<String> = vec![];
    // 接受数据
    for (file_path, meta) in rx {
        if let Some(imports) = &meta.imports {
            for import_file_path in imports {
                if !result_map.contains_key(import_file_path) {
                    import_paths.push(import_file_path.to_string());
                }
            }
        }
        result_map.insert(file_path, meta);
    }

    // 等待线程完毕
    for t in threads {
        t.join().unwrap();
    }

    if import_paths.len() > 0 {
        parse_css_files_recrusive(&import_paths, result_map);
    }
}

fn parse_css_files_with_thread(file_paths: &Vec<String>) -> HashMap<String, CssMeta> {
    let mut result_map: HashMap<String, CssMeta> = HashMap::new();
    parse_css_files_recrusive(file_paths, &mut result_map);
    result_map
}

/// 解析一组 css 文件
fn parse_css_files(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = arg0.to_vec(&mut cx).unwrap();
    let file_paths: Vec<String> = vec
        .iter()
        .map(|&v| v.to_string(&mut cx).unwrap().value(&mut cx))
        .collect();

    let result_map = parse_css_files_with_thread(&file_paths);
    let result = serde_json::to_string(&result_map).unwrap();
    return Ok(cx.string(result));
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseScript", parse_script)?;
    cx.export_function("parseScriptFiles", parse_script_files)?;
    cx.export_function("parseCss", parse_css)?;
    cx.export_function("parseCssFiles", parse_css_files)?;
    Ok(())
}

#[test]
fn test_parse_script_files_with_thread() {
    let file_paths = vec![
        String::from("test/fixtures/page.js"),
        String::from("test/fixtures/component.js"),
    ];
    let results = parse_script_files_with_thread(&file_paths);
    let result = results.get("test/fixtures/component.js").unwrap();
    assert_eq!(result.data.len(), 4);
    assert_eq!(result.methods.len(), 6);
    assert_eq!(result.properties.len(), 5);

    let result = results.get("test/fixtures/page.js").unwrap();
    assert_eq!(result.data.len(), 4);
    assert_eq!(result.methods.len(), 6);
    assert_eq!(result.properties.len(), 0);

    // with not found
    let file_paths = vec![String::from("test/fixtures/component-notfound.js")];
    let results = parse_css_files_with_thread(&file_paths);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_parse_css_files_with_thread() {
    let file_paths = vec![String::from("test/fixtures/component.css")];
    let results = parse_css_files_with_thread(&file_paths);
    let result = results.get("test/fixtures/component.css").unwrap();
    assert_eq!(result.classes.len(), 10);

    // with not found
    let file_paths = vec![String::from("test/fixtures/component-notfound.css")];
    let results = parse_css_files_with_thread(&file_paths);
    assert_eq!(results.len(), 0);

    // with imported css
    let file_paths = vec![std::path::PathBuf::from("test/fixtures/import.css")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()];
    let results = parse_css_files_with_thread(&file_paths);
    assert_eq!(results.len(), 3);

    let css_path = std::path::PathBuf::from("test/fixtures/import.css")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let result = results.get(&css_path).unwrap();
    assert_eq!(result.classes.len(), 1);

    let css_path = std::path::PathBuf::from("test/fixtures/component.css")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let result = results.get(&css_path).unwrap();
    assert_eq!(result.classes.len(), 10);

    let css_path = std::path::PathBuf::from("test/fixtures/page.css")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let result = results.get(&css_path).unwrap();
    assert_eq!(result.classes.len() > 10, true);
}
