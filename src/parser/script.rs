use crate::parser::meta::{
    ComponentMeta, ComponentType, DataMeta, Location, MethodMeta, PropertyMeta, PropertyValue,
};
use regex::Regex;
use std::path::Path;
use std::{collections::HashSet, ops::Deref};
use swc_common::{
    comments::{CommentKind, Comments, SingleThreadedComments},
    sync::Lrc,
    BytePos,
};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceFile, SourceMap,
};
use swc_ecma_ast::{
    Callee, Expr, Ident, KeyValueProp, Lit, MethodProp, Module, ModuleItem, ObjectLit, Prop,
    PropName, PropOrSpread, Stmt,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use super::EventMeta;

pub fn parse_component(file_path: &str) -> Result<ComponentMeta, String> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let sf_result = cm.load_file(Path::new(file_path));
    if let Err(_) = sf_result {
        return Err(format!("No such file {}", file_path));
    }

    let sf = sf_result.unwrap();
    let comments_map: SingleThreadedComments = Default::default();
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*sf),
        Some(&comments_map),
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module_result = parser.parse_module();
    if let Err(_) = module_result {
        return Err(format!("failed to parse {}", file_path));
    }

    let module = module_result.unwrap();

    if let Ok(result) = get_component_call(&module) {
        if let Some(expr) = result.expr {
            let mut component_meta = ComponentMeta::new(result.r#type);

            if let Ok(result) = find_property_with_object_value("data", &expr) {
                if let Ok(res) = get_data_meta(&result, &comments_map, &sf) {
                    component_meta.data = res;
                }
            }
            match component_meta.r#type {
                ComponentType::Page => {
                    if let Ok(res) = get_methods_meta(&expr, &comments_map, &sf) {
                        component_meta.methods = res;
                    }
                }
                ComponentType::Component => {
                    if let Ok(result) = find_property_with_object_value("properties", &expr) {
                        if let Ok(res) = get_properties_meta(&result, &comments_map, &sf) {
                            component_meta.properties = res;
                        }
                    }
                    if let Ok(result) = find_property_with_object_value("methods", &expr) {
                        if let Ok(res) = get_methods_meta(&result, &comments_map, &sf) {
                            component_meta.methods = res;
                        }
                    }

                    if let Ok(res) = parse_trigger_event(&comments_map, &sf) {
                        component_meta.events = Some(res);
                    }
                }
            }
            return Ok(component_meta);
        }
    }

    Err("component not found!".to_string())
}

/// 转换 字节位置到字符位置
fn convert_bytepos_pos(start: BytePos, end: BytePos, sf: &SourceFile) -> Location {
    let line_index = sf.lookup_line(start).unwrap();
    let line_pos = sf.line_begin_pos(start);
    let mut loc = Location::default();
    loc.start.line = line_index + 1;
    loc.start.column = (&sf.src[line_pos.0 as usize..start.0 as usize])
        .chars()
        .count();

    let line_index = sf.lookup_line(end).unwrap();
    let line_pos = sf.line_begin_pos(end);
    loc.end.line = line_index + 1;
    loc.end.column = (&sf.src[line_pos.0 as usize..end.0 as usize])
        .chars()
        .count();
    loc
}

fn find_property_with_object_value<'a>(
    name: &str,
    properties: &'a ObjectLit,
) -> Result<&'a ObjectLit, ()> {
    if let Ok(prop_value) = find_property_by_name(name, properties) {
        if let Expr::Object(object) = &**prop_value {
            return Ok(object);
        }
    }
    Err(())
}

fn find_property_by_name<'a>(name: &str, properties: &'a ObjectLit) -> Result<&'a Box<Expr>, ()> {
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            if let Prop::KeyValue(key_value) = &**prop {
                if let PropName::Ident(prop_name) = &key_value.key {
                    if prop_name.sym.eq(name) {
                        return Ok(&key_value.value);
                    }
                }
            }
        }
    }
    Err(())
}

// 获取类型为 Ident, Str, Num 属性名，其他类型不支持
fn find_prop_name(
    prop_name: &PropName,
    comments_map: &dyn Comments,
    sf: &SourceFile,
) -> Result<PropNameMeta, ()> {
    match prop_name {
        PropName::Ident(prop_name) => Ok(PropNameMeta {
            name: prop_name.sym.to_string(),
            comment: get_comment(prop_name.span.lo(), comments_map),
            loc: convert_bytepos_pos(prop_name.span.lo(), prop_name.span.hi(), &sf),
        }),
        PropName::Str(prop_name) => Ok(PropNameMeta {
            name: prop_name.value.to_string(),
            comment: get_comment(prop_name.span.lo(), comments_map),
            loc: convert_bytepos_pos(prop_name.span.lo(), prop_name.span.hi(), &sf),
        }),
        PropName::Num(prop_name) => Ok(PropNameMeta {
            name: prop_name.value.to_string(),
            comment: get_comment(prop_name.span.lo(), comments_map),
            loc: convert_bytepos_pos(prop_name.span.lo(), prop_name.span.hi(), &sf),
        }),
        _ => Err(()),
    }
}

fn get_comment(pos: BytePos, comments_map: &dyn Comments) -> Option<String> {
    let res = comments_map.get_leading(pos);
    if let Some(comments) = res {
        if let Some(comment) = comments.get(0) {
            return match comment.kind {
                CommentKind::Line => Some(format!("//{}", comment.text)),
                CommentKind::Block => Some(format!("/*{}*/", comment.text)),
            };
        }
    }
    None
}

struct PropNameMeta {
    name: String,
    comment: Option<String>,
    loc: Location,
}

fn get_properties_meta(
    properties: &ObjectLit,
    comments_map: &dyn Comments,
    sf: &SourceFile,
) -> Result<Vec<PropertyMeta>, ()> {
    let mut result = vec![];
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            match &**prop {
                Prop::KeyValue(KeyValueProp { key, value }) => {
                    if let Ok(name) = find_prop_name(key, comments_map, sf) {
                        if let Expr::Object(expr) = &**value {
                            let mut property = PropertyMeta::new(name.name);
                            property.comment = name.comment;
                            property.loc = name.loc;
                            if let Ok(prop_value) = find_property_by_name("type", &expr) {
                                if let Expr::Ident(Ident { sym, .. }) = &**prop_value {
                                    property.r#type = sym.to_string();
                                }
                            }
                            if let Ok(prop_value) = find_property_by_name("value", &expr) {
                                match &**prop_value {
                                    Expr::Lit(Lit::Str(value)) => {
                                        property.value =
                                            Some(PropertyValue::String(value.value.to_string()))
                                    }
                                    Expr::Lit(Lit::Num(value)) => {
                                        property.value = Some(PropertyValue::Number(value.value))
                                    }
                                    Expr::Lit(Lit::Bool(value)) => {
                                        property.value = Some(PropertyValue::Boolean(value.value))
                                    }
                                    _ => (),
                                }
                            }
                            result.push(property);
                        }
                    }
                }
                _ => (),
            }
        }
    }
    Ok(result)
}

/// 获取 data 数据的子项，以便于查找 data.data.data 的情况
fn parse_data_children(
    properties: &ObjectLit,
    comments_map: &dyn Comments,
    sf: &SourceFile,
) -> Option<Vec<DataMeta>> {
    let mut output = vec![];
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            match &**prop {
                Prop::KeyValue(KeyValueProp { key, value, .. }) => {
                    if let Ok(name) = find_prop_name(key, comments_map, sf) {
                        let mut data = DataMeta::new(name.name);
                        data.comment = name.comment;
                        data.loc = name.loc;

                        if let Expr::Object(object) = &**value {
                            data.children = parse_data_children(&object, comments_map, &sf);
                        }
                        output.push(data);
                    }
                }
                _ => (),
            }
        }
    }
    Some(output)
}

fn get_data_meta(
    properties: &ObjectLit,
    comments_map: &dyn Comments,
    sf: &SourceFile,
) -> Result<Vec<DataMeta>, ()> {
    let mut result = vec![];
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            match &**prop {
                Prop::KeyValue(KeyValueProp { key, value, .. }) => {
                    if let Ok(name) = find_prop_name(key, comments_map, sf) {
                        let mut data = DataMeta::new(name.name);
                        data.comment = name.comment;
                        data.loc = name.loc;

                        if let Expr::Object(object) = &**value {
                            data.children = parse_data_children(&object, comments_map, &sf);
                        }
                        result.push(data);
                    }
                }
                _ => (),
            }
        }
    }
    Ok(result)
}

fn get_methods_meta(
    properties: &ObjectLit,
    comments_map: &dyn Comments,
    sf: &SourceFile,
) -> Result<Vec<MethodMeta>, ()> {
    let mut result = vec![];
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            match &**prop {
                Prop::KeyValue(KeyValueProp { key, value }) => {
                    if let Ok(name) = find_prop_name(key, comments_map, sf) {
                        let mut method = MethodMeta::new(name.name);
                        method.comment = name.comment;
                        method.loc = name.loc;

                        match &**value {
                            Expr::Arrow(_) => result.push(method),
                            Expr::Fn(_) => result.push(method),
                            _ => (),
                        }
                    }
                }
                Prop::Method(MethodProp { key, .. }) => {
                    if let PropName::Ident(prop_name) = key {
                        let mut method = MethodMeta::new(prop_name.sym.to_string());
                        method.comment = get_comment(prop_name.span.lo(), comments_map);
                        method.loc =
                            convert_bytepos_pos(prop_name.span.lo(), prop_name.span.hi(), &sf);
                        result.push(method);
                    }
                }
                _ => (),
            }
        }
    }
    Ok(result)
}

struct GetComponentCallResult<'a> {
    r#type: ComponentType,
    expr: Option<&'a ObjectLit>,
}

fn get_component_call(module: &Module) -> Result<GetComponentCallResult, ()> {
    for module_item in &module.body {
        if let ModuleItem::Stmt(Stmt::Expr(expr_stmt)) = module_item {
            if let Expr::Call(call_expr) = &*expr_stmt.expr {
                if  call_expr.args.len() == 0 {
                    continue;
                }

                if let Callee::Expr(expr) = &call_expr.callee {
                    if let Expr::Ident(Ident { sym, .. }) = &**expr {
                        if sym.eq("Component") {
                            let mut result = GetComponentCallResult {
                                r#type: ComponentType::Component,
                                expr: Option::None,
                            };
                            let args = &call_expr.args[0];
                            if let Expr::Object(object) = &*args.expr {
                                result.expr = Option::Some(object);
                            } else {
                                result.expr =
                                    guess_component_params(ComponentType::Component, &*args.expr);
                            }
                            return Ok(result);
                        }
                        else if sym.eq("Page") {
                            let mut result = GetComponentCallResult {
                                r#type: ComponentType::Page,
                                expr: Option::None,
                            };
                            let args = &call_expr.args[0];
                            if let Expr::Object(object) = &*args.expr {
                                result.expr = Option::Some(object);
                            } else {
                                result.expr =
                                    guess_component_params(ComponentType::Page, &*args.expr);
                            }
                            return Ok(result);
                        }
                        // guess myComponent({data: {}, ...})
                        else if sym.ends_with("Component") {
                            let args = &call_expr.args[0];
                            if let Expr::Object(_) = &*args.expr {
                                if let Some(expr) = guess_component_params(ComponentType::Component, &*expr_stmt.expr) {
                                    let result = GetComponentCallResult {
                                        r#type: ComponentType::Component,
                                        expr: Some(expr),
                                    };
                                    return Ok(result);
                                }
                            }
                        }
                        // guess myPage({data: {}, ...})
                        else if sym.ends_with("Page") {
                            let args = &call_expr.args[0];
                            if let Expr::Object(_) = &*args.expr {
                                if let Some(expr) = guess_component_params(ComponentType::Page, &*expr_stmt.expr) {
                                    let result = GetComponentCallResult {
                                        r#type: ComponentType::Page,
                                        expr: Some(expr),
                                    };
                                    return Ok(result);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Err(())
}

fn find_and_count_properties<'a>(names: &HashSet<&str>, properties: &'a ObjectLit) -> usize {
    let mut count = 0;
    for prop_or_spread in &properties.props {
        if let PropOrSpread::Prop(prop) = prop_or_spread {
            match &**prop {
                Prop::KeyValue(KeyValueProp { key, .. }) | Prop::Method(MethodProp { key, .. }) => {
                    if let PropName::Ident(prop_name) = &key {
                        let name = prop_name.sym.deref();
                        if names.contains(name) {
                            count += 1;
                        }
                    }
                }
                _ => (),
            }
        }
    }
    return count;
}

/// 解析非标准组件注册方式，例如：
///
/// Page(wrapper({ data: {} })),
///
/// Page(my.wrapper({ data: {} })),
///
/// Page(wrapper(wrapper({ data: {} })))
fn guess_component_params(r#type: ComponentType, expr: &Expr) -> Option<&ObjectLit> {
    if let Expr::Call(call_expr) = expr {
        let args = &call_expr.args[0];
        if let Expr::Object(object) = &*args.expr {
            let names = match r#type {
                // Component 检测到 properties, data, methods 则认为是配置项
                ComponentType::Component => HashSet::from(["properties", "data", "methods", "attached", "ready"]),
                // Page 检测到 data, onLoad, onShow 则认为是配置项
                ComponentType::Page => HashSet::from(["data", "onInit", "onLoad", "onReady", "onShow"]),
            };

            if find_and_count_properties(&names, object) >= 2 {
                return Some(object);
            }
        } else {
            return guess_component_params(r#type, &*args.expr);
        }
    }
    None
}

fn parse_trigger_event(comments_map: &dyn Comments, sf: &SourceFile) -> Result<Vec<EventMeta>, ()> {
    let regex = Regex::new(r#"this\.triggerEvent\s*\(\s*["'](?P<name>\w+)["']"#).unwrap();

    let mut last_line = 1;
    let mut last_offset = 0;
    let mut events_set: HashSet<String> = HashSet::new();
    let mut events = vec![];
    let text = &sf.src;

    for caps in regex.captures_iter(&text) {
        let match0 = caps.get(0).unwrap();
        let start = match0.start();
        let end = match0.end();

        let event_name = caps["name"].to_string();
        if events_set.contains(&event_name) {
            continue;
        }
        let pass_lines = text[last_offset..start]
            .chars()
            .filter(|c| c == &'\n')
            .count();
        let mut column = 0;
        if let Some(column_index) = text[..start].rfind('\n') {
            column = text[column_index..start].chars().count() - 1;
        }

        last_line += pass_lines;
        last_offset = end;

        events_set.insert(event_name.to_string());

        let mut event_meta = EventMeta::new(event_name);
        event_meta.loc = Location::from([last_line, column], [last_line, column + 17]);
        event_meta.comment = get_comment(BytePos(start as u32), comments_map);
        events.push(event_meta);
    }

    Ok(events)
}

#[test]
fn test_parse_component() {
    let result = parse_component("test/fixtures/component.js").unwrap();
    assert_eq!(matches!(result.r#type, ComponentType::Component), true);
    assert_eq!(result.data.len(), 4);
    assert_eq!(result.methods.len(), 6);
    assert_eq!(result.properties.len(), 5);
    assert_eq!(result.events.as_ref().unwrap().len(), 2);

    let data = result.data.get(0).unwrap();
    assert_eq!(data.name, "data1");
    assert_eq!(data.comment.as_ref().unwrap(), "// data1 属性");
    assert_eq!(data.loc, Location::from([8, 8], [8, 13]));

    // 子数据
    let child_data = data
        .children
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap();
    assert_eq!(child_data.name, "data111");

    // property
    let property = result.properties.get(0).unwrap();
    assert_eq!(property.name, "theme");
    assert_eq!(
        property.comment.as_ref().unwrap(),
        r"/**
         * Boolean 属性
         */"
    );
    assert_eq!(property.r#type, "Boolean");
    assert_eq!(
        property.value.as_ref().unwrap(),
        &PropertyValue::Boolean(false)
    );

    // property
    let property = result.properties.get(1).unwrap();
    assert_eq!(property.name, "color-string");
    assert_eq!(property.r#type, "String");
    assert_eq!(
        property.value.as_ref().unwrap(),
        &PropertyValue::String("#3388FF".to_string())
    );

    // property
    let property = result.properties.get(2).unwrap();
    assert_eq!(property.name, "666");
    assert_eq!(property.r#type, "Number");
    assert_eq!(
        property.value.as_ref().unwrap(),
        &PropertyValue::Number(666.0)
    );

    // method
    let method = result.methods.get(0).unwrap();
    assert_eq!(method.name, "method1");
    assert_eq!(method.comment.as_ref().unwrap(), "// method1 方法");

    let not_found = parse_component("test/fixtures/component-notfound.js").err();
    assert_eq!(
        Some(String::from(
            "No such file test/fixtures/component-notfound.js"
        )),
        not_found
    );
    // parse error
    let parse_error = parse_component("test/fixtures/component.error-js").err();
    assert_eq!(
        Some(String::from(
            "failed to parse test/fixtures/component.error-js"
        )),
        parse_error
    );

    // events
    let event = result.events.as_ref().unwrap().get(0).unwrap();
    println!("{:?}", event);
    assert_eq!(event.name, "event1");
    assert_eq!(event.comment.as_ref().unwrap(), "// event1 事件");

    let event = result.events.as_ref().unwrap().get(1).unwrap();
    println!("{:?}", event);
    assert_eq!(event.name, "event2");
    assert_eq!(matches!(event.comment, None), true);
}

#[test]
fn test_parse_page() {
    let result = parse_component("test/fixtures/page.js").unwrap();
    assert_eq!(matches!(result.r#type, ComponentType::Page), true);
    assert_eq!(result.data.len(), 4);
    assert_eq!(result.methods.len(), 6);
    assert_eq!(result.properties.len(), 0);
    assert_eq!(matches!(result.events, None), true);

    let data = result.data.get(0).unwrap();
    assert_eq!(data.name, "data1");
    assert_eq!(data.comment.as_ref().unwrap(), "// data1 属性");
    assert_eq!(data.loc, Location::from([8, 8], [8, 13]));

    let method = result.methods.get(0).unwrap();
    assert_eq!(method.name, "method1");
    assert_eq!(method.comment.as_ref().unwrap(), "// method1 方法");

    let not_found = parse_component("test/fixtures/page-notfound.js").err();
    assert_eq!(
        Some(String::from("No such file test/fixtures/page-notfound.js")),
        not_found
    );
}

#[test]
fn test_get_component_call() {
    // Page
    let source_code = r#"
Page(guess.wrapOptions({
    data: {},
    onLoad() {}
}));"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Page), true);
    assert_eq!(matches!(call_module.expr, Some(_)), true);

    // Page
    let source_code = r#"
myPage({
    data: {},
    onLoad() {}
});"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Page), true);
    assert_eq!(matches!(call_module.expr, Some(_)), true);

    // Component
    let source_code = r#"
Component(wrap(wrap1({
    properties: {},
    methods: {}
})));"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Component), true);
    assert_eq!(matches!(call_module.expr, Some(_)), true);

    // Component
    let source_code = r#"
myComponent({
    properties: {},
    methods: {}
});"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Component), true);
    assert_eq!(matches!(call_module.expr, Some(_)), true);

    // None
    let source_code = r#"
Component([{
    properties: {},
    methods: {}
}]);"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Component), true);
    assert_eq!(matches!(call_module.expr, None), true);

    // None
    let source_code = r#"
Page(wrap({
    c() {}
}));"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module).unwrap();
    assert_eq!(matches!(call_module.r#type, ComponentType::Page), true);
    assert_eq!(matches!(call_module.expr, None), true);

    // Error
    let source_code = r#"
Page();"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module);
    assert_eq!(matches!(call_module, Err(_)), true);

    // Error
    let source_code = r#"
myPage(wrap({
    data: {},
    onLoad() {}
}));"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module);
    assert_eq!(matches!(call_module, Err(_)), true);

    // Error
    let source_code = r#"
wrap({
    c() {}
});"#;
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::new(source_code, BytePos(0), BytePos(0)),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let call_module = get_component_call(&module);
    assert_eq!(matches!(call_module, Err(_)), true);
}

#[test]
fn test_guess_component_params() {
    let result = parse_component("test/fixtures/guess-page.js").unwrap();
    assert_eq!(matches!(result.r#type, ComponentType::Page), true);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.methods.len(), 1);
}
