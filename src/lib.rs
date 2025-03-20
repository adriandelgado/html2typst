use std::{borrow::Cow, fmt::Write as _};

use html5ever::{Attribute, ParseOpts, parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

#[derive(Debug, Default)]
struct Context {
    tag_stack: Vec<Option<Box<str>>>,
    output: String,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn parse_html(html: &str) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        // SAFETY: we are reading from a string
        .unwrap();

    let mut ctx = Context::default();

    walk(&dom.document, &mut ctx);

    cleanup(&ctx.output)
}

fn cleanup(output: &str) -> String {
    output.trim().to_owned()
}

#[allow(clippy::too_many_lines)]
fn walk(node: &Handle, ctx: &mut Context) {
    match &node.data {
        NodeData::Document
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. }
        | NodeData::Comment { .. } => walk_descendants(node, ctx, None),
        NodeData::Text { contents } => {
            // Consider:
            // - inside <pre> or <code> tags
            // - trimmed len == 0
            // - last char is a space or a newline
            // - escaping text
            // - remove excess whitespace, newlines, and carriage returns
            let text = contents.borrow();

            let escaped_text = escape_html(text.trim());
            ctx.output.push_str(&escaped_text);
        }
        NodeData::Element { name, attrs, .. } => {
            // Consider:
            // - inside <pre>
            let tag_name = name.local.as_ref();

            match tag_name {
                "hr" | "q" | "cite" | "details" | "summary" | "pre" | "code" | "sub" | "sup"
                | "table" | "iframe" => {
                    todo!("{tag_name}")
                }
                "div" | "section" | "header" | "footer" => {
                    ctx.output.push_str("\n\n");
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push_str("\n\n");
                }
                "li" => {
                    let mut tag_iter = ctx.tag_stack.iter().rev().filter_map(|t| {
                        let t = t.as_deref();
                        if matches!(t, Some("ol" | "ul" | "menu")) {
                            t
                        } else {
                            None
                        }
                    });
                    let parent_tag = tag_iter.next();
                    let tag_level = tag_iter.count();
                    match parent_tag {
                        Some("ol") => {
                            ctx.output
                                .write_fmt(format_args!("{: <width$}+ ", "", width = tag_level * 2))
                                // SAFETY: we are writing to a String
                                .unwrap();
                        }
                        Some("ul" | "menu") | None => {
                            ctx.output
                                .write_fmt(format_args!("{: <width$}- ", "", width = tag_level * 2))
                                // SAFETY: we are writing to a String
                                .unwrap();
                        }
                        _ => unreachable!(),
                    }
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push('\n');
                }
                "ol" | "ul" | "menu" => {
                    ctx.output.push('\n');
                    if ctx
                        .tag_stack
                        .iter()
                        .rev()
                        .filter_map(|t| {
                            let t = t.as_deref();
                            if matches!(t, Some("ol" | "ul" | "menu")) {
                                t
                            } else {
                                None
                            }
                        })
                        .count()
                        == 0
                    {
                        ctx.output.push('\n');
                    };
                    // TODO: extra newline if not inside a list
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push_str("\n\n");
                }
                "s" | "del" => {
                    // TODO: handle spaces
                    ctx.output.push_str("#strike[");
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push(']');
                }
                "b" | "strong" => {
                    // TODO: handle spaces
                    ctx.output.push('*');
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push('*');
                }
                "i" | "em" => {
                    // TODO: handle spaces
                    ctx.output.push('_');
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push('_');
                }
                "u" | "ins" => {
                    // TODO: handle spaces
                    ctx.output.push_str("#underline[");
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push(']');
                }
                "blockquote" => {
                    ctx.output.push_str("\n\n#quote(block: true)[\n");
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push_str("\n]\n\n");
                }
                level @ ("h1" | "h2" | "h3" | "h4" | "h5" | "h6") => {
                    let level = usize::from(level.as_bytes()[1] - b'0');
                    ctx.output
                        .write_fmt(format_args!("{:=<width$} ", "", width = level))
                        // SAFETY: we are writing to a String
                        .unwrap();
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    if let Some(id) = get_attr_value(&attrs.borrow(), "id") {
                        ctx.output
                            // TODO: escape?
                            .write_fmt(format_args!(" <{id}>\n"))
                            // SAFETY: we are writing to a String
                            .unwrap();
                    }
                }
                "html" | "head" | "body" => walk_descendants(node, ctx, Some(Box::from(tag_name))),
                "p" => {
                    walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    ctx.output.push_str("\n\n");
                }
                "br" => ctx.output.push_str("\\\n"),
                "a" => {
                    if let Some(href) = get_attr_value(&attrs.borrow(), "href") {
                        ctx.output
                            // TODO: escape href ?
                            .write_fmt(format_args!(r#"#link("{href}")["#))
                            // SAFETY: we are writing to a string
                            .unwrap();
                        walk_descendants(node, ctx, Some(Box::from(tag_name)));
                        ctx.output.push(']');
                    } else {
                        walk_descendants(node, ctx, Some(Box::from(tag_name)));
                    }
                }
                "img" => {
                    let attrs = attrs.borrow();

                    // TODO: check if the escaping is correct
                    let src = get_attr_value(&attrs, "src");
                    let alt = get_attr_value(&attrs, "alt");
                    match (src, alt) {
                        (Some(src), Some(alt)) => {
                            ctx.output
                                .write_fmt(format_args!(
                                    r#"#figure(caption: [{alt}], image(alt: "{}", "{}"))"#,
                                    escape_quotes(alt),
                                    escape_quotes(src),
                                ))
                                // SAFETY: we are writing to a string
                                .unwrap();
                        }
                        (Some(src), None) => {
                            // TODO: test the escaping
                            ctx.output
                                .write_fmt(format_args!(
                                    r#"#figure(caption: none, image("{}"))"#,
                                    escape_quotes(src),
                                ))
                                // SAFETY: we are writing to a string
                                .unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    };
}

fn get_attr_value<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|attr| attr.name.local.as_ref() == name)
        .map(|attr| attr.value.as_ref())
}

fn walk_descendants(node: &Handle, ctx: &mut Context, tag_name: Option<Box<str>>) {
    ctx.tag_stack.push(tag_name);

    for child in node.children.borrow().iter() {
        walk(child, ctx);
    }

    ctx.tag_stack.pop();
}

fn escape_quotes(html: &str) -> Cow<str> {
    if !html.contains('"') {
        return Cow::Borrowed(html);
    }

    let mut escaped = vec![];

    let bytes = html.as_bytes();

    for &ch in bytes {
        if matches!(ch, b'"') {
            escaped.push(b'\\');
        }
        escaped.push(ch);
    }

    Cow::Owned(
        String::from_utf8(escaped)
            // SAFETY: we started with valid utf8
            .unwrap(),
    )
}

fn escape_html(html: &str) -> Cow<str> {
    if !html.contains(['*', '_', '<', '>']) && !html.starts_with(['=', '-', '+']) {
        return Cow::Borrowed(html);
    }
    let mut escaped = vec![];

    let bytes = html.as_bytes();

    if matches!(bytes, [b'=' | b'-' | b'+', ..]) {
        escaped.push(b'\\');
    }

    for &ch in bytes {
        if matches!(ch, b'*' | b'_' | b'<' | b'>') {
            escaped.push(b'\\');
        }
        escaped.push(ch);
    }

    Cow::Owned(
        String::from_utf8(escaped)
            // SAFETY: we started with valid utf8
            .unwrap(),
    )
}
