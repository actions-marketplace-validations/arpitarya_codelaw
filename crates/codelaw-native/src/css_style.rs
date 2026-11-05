//! stylelint's notation, empty-line, unknown-name and logical-property rules
//! (W-139), reimplemented on codelaw's sheet.

use crate::css_lists::{each_value, media_features, prop, style_rules, urls};
use crate::css_syntax::{Kind, SKind, Sheet, Simple, VKind, VNode, all_simples, css_rule, dimension, finding, fixed, opt_list, opt_str, set, text, unprefixed};
use codelaw_core::TextEdit;
use std::collections::BTreeSet;

css_rule!(CommentWhitespaceInside, "css/comment-whitespace-inside", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Comment || !src[n.start..].starts_with("/*") || !src[..n.end].ends_with("*/") || n.end - n.start < 4 {
            continue;
        }
        let body = &src[n.start + 2..n.end - 2];
        if body.is_empty() || body.chars().all(|c| c == '*' || c == '!') {
            continue;
        }
        let lead = body.trim_start_matches(['*', '!']);
        let open_at = n.start + 2 + (body.len() - lead.len());
        let mut edits = Vec::new();
        if !lead.starts_with(char::is_whitespace) {
            edits.push(TextEdit::insert(open_at, " "));
        }
        let trail = body.trim_end_matches('*');
        if !trail.ends_with(char::is_whitespace) {
            edits.push(TextEdit::insert(n.start + 2 + trail.len(), " "));
        }
        if !edits.is_empty() {
            out.push(fixed(n.start, n.end, "Put a space inside the comment's delimiters", edits));
        }
    }
});

// ---------------------------------------------------------------------------
// Colours

const NAMED: &[(&str, u32)] = &[
    ("aliceblue", 0xf0f8ff), ("antiquewhite", 0xfaebd7), ("aqua", 0x00ffff), ("aquamarine", 0x7fffd4), ("azure", 0xf0ffff), ("beige", 0xf5f5dc), ("bisque", 0xffe4c4), ("black", 0x000000),
    ("blanchedalmond", 0xffebcd), ("blue", 0x0000ff), ("blueviolet", 0x8a2be2), ("brown", 0xa52a2a), ("burlywood", 0xdeb887), ("cadetblue", 0x5f9ea0), ("chartreuse", 0x7fff00),
    ("chocolate", 0xd2691e), ("coral", 0xff7f50), ("cornflowerblue", 0x6495ed), ("cornsilk", 0xfff8dc), ("crimson", 0xdc143c), ("cyan", 0x00ffff), ("darkblue", 0x00008b),
    ("darkcyan", 0x008b8b), ("darkgoldenrod", 0xb8860b), ("darkgray", 0xa9a9a9), ("darkgreen", 0x006400), ("darkgrey", 0xa9a9a9), ("darkkhaki", 0xbdb76b), ("darkmagenta", 0x8b008b),
    ("darkolivegreen", 0x556b2f), ("darkorange", 0xff8c00), ("darkorchid", 0x9932cc), ("darkred", 0x8b0000), ("darksalmon", 0xe9967a), ("darkseagreen", 0x8fbc8f), ("darkslateblue", 0x483d8b),
    ("darkslategray", 0x2f4f4f), ("darkslategrey", 0x2f4f4f), ("darkturquoise", 0x00ced1), ("darkviolet", 0x9400d3), ("deeppink", 0xff1493), ("deepskyblue", 0x00bfff), ("dimgray", 0x696969),
    ("dimgrey", 0x696969), ("dodgerblue", 0x1e90ff), ("firebrick", 0xb22222), ("floralwhite", 0xfffaf0), ("forestgreen", 0x228b22), ("fuchsia", 0xff00ff), ("gainsboro", 0xdcdcdc),
    ("ghostwhite", 0xf8f8ff), ("gold", 0xffd700), ("goldenrod", 0xdaa520), ("gray", 0x808080), ("green", 0x008000), ("greenyellow", 0xadff2f), ("grey", 0x808080), ("honeydew", 0xf0fff0),
    ("hotpink", 0xff69b4), ("indianred", 0xcd5c5c), ("indigo", 0x4b0082), ("ivory", 0xfffff0), ("khaki", 0xf0e68c), ("lavender", 0xe6e6fa), ("lavenderblush", 0xfff0f5), ("lawngreen", 0x7cfc00),
    ("lemonchiffon", 0xfffacd), ("lightblue", 0xadd8e6), ("lightcoral", 0xf08080), ("lightcyan", 0xe0ffff), ("lightgoldenrodyellow", 0xfafad2), ("lightgray", 0xd3d3d3), ("lightgreen", 0x90ee90),
    ("lightgrey", 0xd3d3d3), ("lightpink", 0xffb6c1), ("lightsalmon", 0xffa07a), ("lightseagreen", 0x20b2aa), ("lightskyblue", 0x87cefa), ("lightslategray", 0x778899), ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xb0c4de), ("lightyellow", 0xffffe0), ("lime", 0x00ff00), ("limegreen", 0x32cd32), ("linen", 0xfaf0e6), ("magenta", 0xff00ff), ("maroon", 0x800000),
    ("mediumaquamarine", 0x66cdaa), ("mediumblue", 0x0000cd), ("mediumorchid", 0xba55d3), ("mediumpurple", 0x9370db), ("mediumseagreen", 0x3cb371), ("mediumslateblue", 0x7b68ee),
    ("mediumspringgreen", 0x00fa9a), ("mediumturquoise", 0x48d1cc), ("mediumvioletred", 0xc71585), ("midnightblue", 0x191970), ("mintcream", 0xf5fffa), ("mistyrose", 0xffe4e1),
    ("moccasin", 0xffe4b5), ("navajowhite", 0xffdead), ("navy", 0x000080), ("oldlace", 0xfdf5e6), ("olive", 0x808000), ("olivedrab", 0x6b8e23), ("orange", 0xffa500), ("orangered", 0xff4500),
    ("orchid", 0xda70d6), ("palegoldenrod", 0xeee8aa), ("palegreen", 0x98fb98), ("paleturquoise", 0xafeeee), ("palevioletred", 0xdb7093), ("papayawhip", 0xffefd5), ("peachpuff", 0xffdab9),
    ("peru", 0xcd853f), ("pink", 0xffc0cb), ("plum", 0xdda0dd), ("powderblue", 0xb0e0e6), ("purple", 0x800080), ("rebeccapurple", 0x663399), ("red", 0xff0000), ("rosybrown", 0xbc8f8f),
    ("royalblue", 0x4169e1), ("saddlebrown", 0x8b4513), ("salmon", 0xfa8072), ("sandybrown", 0xf4a460), ("seagreen", 0x2e8b57), ("seashell", 0xfff5ee), ("sienna", 0xa0522d), ("silver", 0xc0c0c0),
    ("skyblue", 0x87ceeb), ("slateblue", 0x6a5acd), ("slategray", 0x708090), ("slategrey", 0x708090), ("snow", 0xfffafa), ("springgreen", 0x00ff7f), ("steelblue", 0x4682b4), ("tan", 0xd2b48c),
    ("teal", 0x008080), ("thistle", 0xd8bfd8), ("tomato", 0xff6347), ("turquoise", 0x40e0d0), ("violet", 0xee82ee), ("wheat", 0xf5deb3), ("white", 0xffffff), ("whitesmoke", 0xf5f5f5),
    ("yellow", 0xffff00), ("yellowgreen", 0x9acd32),
];

/// The declarations whose values are colours: not names, urls or text.
fn color_decl(p: &str) -> bool {
    !matches!(p, "font-family" | "font" | "animation" | "animation-name" | "grid-area" | "grid-template-areas" | "content" | "counter-reset" | "counter-increment" | "transition" | "transition-property" | "will-change" | "list-style-type" | "quotes")
        && !p.starts_with("--")
}

/// Every word of every colour-bearing value, with owned copies when the
/// sheet was read from text.
fn each_color_word(sheet: &Sheet, src: &str, f: &mut dyn FnMut(&VNode)) {
    for i in sheet.decls() {
        if !color_decl(&prop(sheet, i, src)) {
            continue;
        }
        fn go(list: &[VNode], src: &str, f: &mut dyn FnMut(&VNode)) {
            for v in list {
                match v.kind {
                    VKind::Word => f(v),
                    VKind::Func if !matches!(v.name(src).to_ascii_lowercase().as_str(), "url" | "var" | "env" | "attr" | "format" | "local") => go(&v.nodes, src, f),
                    _ => {}
                }
            }
        }
        go(&sheet.values(i, src), src, f);
    }
}

fn hex_digits(w: &str) -> Option<&str> {
    let h = w.strip_prefix('#')?;
    h.bytes().all(|c| c.is_ascii_hexdigit()).then_some(h)
}

fn rgb_of(h: &str) -> Option<(u32, u32, u32, Option<u32>)> {
    let e = |s: &str| u32::from_str_radix(s, 16).ok();
    let d = |c: char| e(&format!("{c}{c}"));
    let c: Vec<char> = h.chars().collect();
    match c.len() {
        3 => Some((d(c[0])?, d(c[1])?, d(c[2])?, None)),
        4 => Some((d(c[0])?, d(c[1])?, d(c[2])?, Some(d(c[3])?))),
        6 => Some((e(&h[0..2])?, e(&h[2..4])?, e(&h[4..6])?, None)),
        8 => Some((e(&h[0..2])?, e(&h[2..4])?, e(&h[4..6])?, Some(e(&h[6..8])?))),
        _ => None,
    }
}

fn alpha_text(a: u32) -> String {
    let v = (a as f64) / 255.0;
    let s = format!("{v:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.').to_owned();
    if s.is_empty() { "0".into() } else { s }
}

css_rule!(ColorHexAlpha, "css/color-hex-alpha", |sheet, src, ctx, out| {
    let always = opt_str(ctx, "mode", "never") == "always";
    each_color_word(sheet, src, &mut |v| {
        let Some(h) = hex_digits(v.text(src)) else { return };
        let Some((r, g, b, a)) = rgb_of(h) else { return };
        if !always && let Some(a) = a {
            out.push(fixed(v.start, v.end, format!("`#{h}` has an alpha channel; write the transparency with `rgb()`"), vec![TextEdit::new(v.start, v.end, format!("rgb({r} {g} {b} / {})", alpha_text(a)))]));
        } else if always && a.is_none() {
            let full = if h.len() == 3 { h.chars().flat_map(|c| [c, c]).collect::<String>() } else { h.to_owned() };
            out.push(fixed(v.start, v.end, format!("`#{h}` has no alpha channel; write it with one"), vec![TextEdit::new(v.start, v.end, format!("#{full}ff"))]));
        }
    });
});

css_rule!(ColorNamed, "css/color-named", |sheet, src, ctx, out| {
    each_color_word(sheet, src, &mut |v| {
        let w = v.text(src).to_ascii_lowercase();
        if let Some((name, hex)) = NAMED.iter().find(|x| x.0 == w) {
            out.push(fixed(v.start, v.end, format!("The named colour `{name}` is written as `#{hex:06x}`"), vec![TextEdit::new(v.start, v.end, format!("#{hex:06x}"))]));
        }
    });
});

css_rule!(ColorNoHex, "css/color-no-hex", |sheet, src, ctx, out| {
    each_color_word(sheet, src, &mut |v| {
        let Some(h) = hex_digits(v.text(src)) else { return };
        let Some((r, g, b, a)) = rgb_of(h) else { return };
        let new = match a {
            None => format!("rgb({r} {g} {b})"),
            Some(a) => format!("rgb({r} {g} {b} / {})", alpha_text(a)),
        };
        out.push(fixed(v.start, v.end, format!("Write `#{h}` as `{new}`"), vec![TextEdit::new(v.start, v.end, new)]));
    });
});

css_rule!(ColorNoInvalidHex, "css/color-no-invalid-hex", |sheet, src, ctx, out| {
    each_color_word(sheet, src, &mut |v| {
        let w = v.text(src);
        let Some(h) = w.strip_prefix('#') else { return };
        if h.is_empty() || !h.bytes().all(|c| c.is_ascii_alphanumeric()) {
            return;
        }
        if !h.bytes().all(|c| c.is_ascii_hexdigit()) || ![3, 4, 6, 8].contains(&h.len()) {
            out.push(finding(v.start, v.end, format!("`{w}` is not a valid hex colour")));
        }
    });
});

// ---------------------------------------------------------------------------
// Notation

const DISPLAY: &[(&str, &str)] = &[
    ("block flow", "block"), ("flow block", "block"), ("block flow-root", "flow-root"), ("flow-root block", "flow-root"), ("inline flow", "inline"), ("flow inline", "inline"),
    ("inline flow-root", "inline-block"), ("flow-root inline", "inline-block"), ("run-in flow", "run-in"), ("flow run-in", "run-in"), ("block flex", "flex"), ("flex block", "flex"),
    ("inline flex", "inline-flex"), ("flex inline", "inline-flex"), ("block grid", "grid"), ("grid block", "grid"), ("inline grid", "inline-grid"), ("grid inline", "inline-grid"),
    ("inline ruby", "ruby"), ("ruby inline", "ruby"), ("block table", "table"), ("table block", "table"), ("inline table", "inline-table"), ("table inline", "inline-table"),
    ("block flow list-item", "list-item"), ("block list-item", "list-item"), ("list-item block", "list-item"), ("flow list-item", "list-item"), ("list-item flow", "list-item"),
];

css_rule!(DisplayNotation, "css/display-notation", |sheet, src, ctx, out| {
    for i in sheet.decls() {
        if prop(sheet, i, src) != "display" {
            continue;
        }
        let n = &sheet.nodes[i];
        let v: String = text(src, n.params).split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_lowercase();
        if let Some((_, short)) = DISPLAY.iter().find(|x| x.0 == v) {
            out.push(fixed(n.params.0, n.params.1, format!("Write `display: {v}` as `{short}`"), vec![TextEdit::new(n.params.0, n.params.1, *short)]));
        }
    }
});

css_rule!(FontFamilyNameQuotes, "css/font-family-name-quotes", |sheet, src, ctx, out| {
    let keywords = set(ctx.data, "css/keywords", "font_family");
    let wide = set(ctx.data, "css/keywords", "css_wide");
    for i in sheet.decls() {
        if prop(sheet, i, src) != "font-family" {
            continue;
        }
        let n = &sheet.nodes[i];
        let v = text(src, n.params);
        if crate::css_syntax::non_standard_value(v) || v.contains("var(") {
            continue;
        }
        let mut at = n.params.0;
        for part in v.split(',') {
            let start = at + (part.len() - part.trim_start().len());
            let t = part.trim();
            at += part.len() + 1;
            if t.is_empty() {
                continue;
            }
            let quoted = (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\''));
            if quoted {
                let inner = &t[1..t.len() - 1];
                if keywords.contains(&inner.to_ascii_lowercase()) || wide.contains(&inner.to_ascii_lowercase()) {
                    out.push(fixed(start, start + t.len(), format!("`{inner}` is a keyword: quoted, it names a font called “{inner}”"), vec![TextEdit::new(start, start + t.len(), inner)]));
                }
            } else {
                let lower = t.to_ascii_lowercase();
                if keywords.contains(&lower) || wide.contains(&lower) {
                    continue;
                }
                let needs = t.contains(char::is_whitespace) && t.split_whitespace().any(|w| w.starts_with(|c: char| c.is_ascii_digit()) || !w.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'))
                    || t.split_whitespace().count() > 1;
                if needs {
                    out.push(fixed(start, start + t.len(), format!("Quote the font family name `{t}`"), vec![TextEdit::new(start, start + t.len(), format!("\"{t}\""))]));
                }
            }
        }
    }
});

css_rule!(FontWeightNotation, "css/font-weight-notation", |sheet, src, ctx, out| {
    let named = opt_str(ctx, "mode", "numeric") == "named-where-possible";
    for i in sheet.decls() {
        if prop(sheet, i, src) != "font-weight" {
            continue;
        }
        let n = &sheet.nodes[i];
        let v = text(src, n.params).to_ascii_lowercase();
        let new = match (v.as_str(), named) {
            ("normal", false) => "400",
            ("bold", false) => "700",
            ("400", true) => "normal",
            ("700", true) => "bold",
            _ => continue,
        };
        out.push(fixed(n.params.0, n.params.1, format!("Write the font weight `{v}` as `{new}`"), vec![TextEdit::new(n.params.0, n.params.1, new)]));
    }
});

const SIDES: &[&str] = &["top", "bottom", "left", "right"];

fn opposite(s: &str) -> &str {
    match s {
        "top" => "bottom",
        "bottom" => "top",
        "left" => "right",
        _ => "left",
    }
}

css_rule!(FunctionLinearGradientNoNonstandardDirection, "css/function-linear-gradient-no-nonstandard-direction", |sheet, src, ctx, out| {
    for i in sheet.decls() {
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Func || !v.name(src).eq_ignore_ascii_case("linear-gradient") {
                return;
            }
            let inner = &src[v.inner.0..v.inner.1];
            let first = inner.split(',').next().unwrap_or("");
            let words: Vec<&str> = first.split_whitespace().collect();
            let Some(&w0) = words.first() else { return };
            let s = v.inner.0 + (first.len() - first.trim_start().len());
            let e = s + first.trim().len();
            if words.iter().all(|w| SIDES.contains(&w.to_ascii_lowercase().as_str())) && !words.is_empty() {
                let new = format!("to {}", words.iter().map(|w| opposite(&w.to_ascii_lowercase()).to_owned()).collect::<Vec<_>>().join(" "));
                out.push(fixed(s, e, format!("`{}` is the legacy direction; the standard one is `{new}`", first.trim()), vec![TextEdit::new(s, e, new)]));
            } else if w0.eq_ignore_ascii_case("to") && (words.len() < 2 || !words[1..].iter().all(|w| SIDES.contains(&w.to_ascii_lowercase().as_str()))) {
                out.push(finding(s, e, format!("`{}` is not a valid gradient direction", first.trim())));
            }
        });
    }
});

const CAMEL_FUNCTIONS: &[&str] = &["translateX", "translateY", "translateZ", "translate3d", "scaleX", "scaleY", "scaleZ", "scale3d", "rotateX", "rotateY", "rotateZ", "rotate3d", "skewX", "skewY", "matrix3d"];

css_rule!(FunctionNameCase, "css/function-name-case", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Func || v.name.0 == v.name.1 {
                return;
            }
            let name = v.name(src);
            if CAMEL_FUNCTIONS.contains(&name) || name.starts_with("--") {
                return;
            }
            let lower = name.to_ascii_lowercase();
            let want = CAMEL_FUNCTIONS.iter().find(|c| c.to_ascii_lowercase() == lower).map(|c| c.to_string()).unwrap_or(lower);
            if want != name {
                out.push(fixed(v.name.0, v.name.1, format!("Write the function `{name}()` as `{want}()`"), vec![TextEdit::new(v.name.0, v.name.1, want)]));
            }
        });
    }
});

css_rule!(FunctionNoUnknown, "css/function-no-unknown", |sheet, src, ctx, out| {
    let known = set(ctx.data, "css/functions", "known");
    if known.is_empty() {
        return;
    }
    let ignore = opt_list(ctx, "ignore");
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) || crate::css_syntax::non_standard_value(text(src, sheet.nodes[i].params)) {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Func || v.name.0 == v.name.1 {
                return;
            }
            let name = v.name(src).to_ascii_lowercase();
            if name.starts_with("--") || known.contains(unprefixed(&name)) || known.contains(&name) || crate::css_syntax::matches_any(&ignore, &name) {
                return;
            }
            out.push(finding(v.name.0, v.name.1, format!("`{name}()` is not a known CSS function")));
        });
    }
});

css_rule!(FunctionUrlNoSchemeRelative, "css/function-url-no-scheme-relative", |sheet, src, ctx, out| {
    for (u, s, e, _) in urls(sheet, src) {
        if u.starts_with("//") {
            out.push(finding(s, e, format!("`{u}` is scheme-relative: name `https:` or `http:`")));
        }
    }
});

css_rule!(FunctionUrlQuotes, "css/function-url-quotes", |sheet, src, ctx, out| {
    let never = opt_str(ctx, "mode", "always") == "never";
    for (u, s, e, q) in urls(sheet, src) {
        if u.is_empty() || u.starts_with("data:") && never {
            continue;
        }
        let inner = src[s..e].trim();
        let is = s + (src[s..e].len() - src[s..e].trim_start().len());
        match (q, never) {
            (None, false) if !u.contains('"') => out.push(fixed(is, is + inner.len(), format!("Quote the URL `{u}`"), vec![TextEdit::new(is, is + inner.len(), format!("\"{u}\""))])),
            (Some(_), true) if !u.contains(|c: char| c.is_whitespace() || "()'\"\\".contains(c)) => out.push(fixed(is, is + inner.len(), format!("Write the URL `{u}` without quotes"), vec![TextEdit::new(is, is + inner.len(), u.clone())])),
            _ => {}
        }
    }
});

// ---------------------------------------------------------------------------
// Unknown names

css_rule!(NoUnknownAnimations, "css/no-unknown-animations", |sheet, src, ctx, out| {
    let mut defined = BTreeSet::new();
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind == Kind::AtRule && sheet.at_name(i, src) == "keyframes" {
            defined.insert(text(src, n.params).trim_matches(['"', '\'']).to_owned());
        }
    }
    let keywords = ["none", "initial", "inherit", "unset", "revert", "revert-layer", "infinite", "linear", "ease", "ease-in", "ease-out", "ease-in-out", "step-start", "step-end", "normal", "reverse", "alternate", "alternate-reverse", "forwards", "backwards", "both", "running", "paused", "auto"];
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        if p != "animation-name" && p != "animation" {
            continue;
        }
        let vals = sheet.values(i, src);
        for v in vals.iter() {
            if v.kind != VKind::Word {
                continue;
            }
            let w = v.text(src);
            if keywords.contains(&w.to_ascii_lowercase().as_str()) || dimension(w).is_some() || w.starts_with(['$', '@', '-']) && w.starts_with("--") || crate::css_syntax::non_standard_value(w) {
                continue;
            }
            if !w.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                continue;
            }
            if !defined.contains(w) {
                out.push(finding(v.start, v.end, format!("No `@keyframes {w}` is defined in this stylesheet")));
            }
        }
    }
});

css_rule!(NoUnknownCustomMedia, "css/no-unknown-custom-media", |sheet, src, ctx, out| {
    let mut defined = BTreeSet::new();
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind == Kind::AtRule && sheet.at_name(i, src) == "custom-media"
            && let Some(name) = text(src, n.params).split_whitespace().next() {
                defined.insert(name.to_owned());
            }
    }
    for (name, r, _) in media_features(sheet, src) {
        if name.starts_with("--") && !defined.contains(&name) {
            out.push(finding(r.0, r.1, format!("No `@custom-media {name}` is defined in this stylesheet")));
        }
    }
});

css_rule!(NoUnknownCustomProperties, "css/no-unknown-custom-properties", |sheet, src, ctx, out| {
    let mut defined = BTreeSet::new();
    for i in sheet.decls() {
        let p = text(src, sheet.nodes[i].name);
        if p.starts_with("--") {
            defined.insert(p.to_owned());
        }
    }
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind == Kind::AtRule && sheet.at_name(i, src) == "property" {
            defined.insert(text(src, n.params).to_owned());
        }
    }
    for i in sheet.decls() {
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Func || !v.name(src).eq_ignore_ascii_case("var") {
                return;
            }
            let inner = &src[v.inner.0..v.inner.1];
            if inner.contains(',') {
                return; // a fallback
            }
            let name = inner.trim();
            if name.starts_with("--") && !defined.contains(name) {
                out.push(finding(v.start, v.end, format!("`{name}` is not defined in this stylesheet and has no fallback")));
            }
        });
    }
});

css_rule!(UnitNoUnknown, "css/unit-no-unknown", |sheet, src, ctx, out| {
    let units: BTreeSet<String> = set(ctx.data, "css/units", "all").into_iter().map(|u| u.to_ascii_lowercase()).collect();
    if units.is_empty() {
        return;
    }
    let ignore = opt_list(ctx, "ignore");
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) || crate::css_syntax::non_standard_value(text(src, sheet.nodes[i].params)) {
            continue;
        }
        let p = prop(sheet, i, src);
        if p.starts_with("--") {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Word {
                return;
            }
            let w = v.text(src);
            if w.starts_with('#') {
                return;
            }
            if let Some((num, u)) = dimension(w)
                && !u.is_empty()
                && !num.is_empty()
                && !units.contains(&u.to_ascii_lowercase())
                && !(u.eq_ignore_ascii_case("x") || u.starts_with('e') && u.len() > 1 && u[1..].chars().all(|c| c.is_ascii_digit()))
                && !crate::css_syntax::matches_any(&ignore, u)
            {
                out.push(finding(v.start, v.end, format!("`{u}` is not a known CSS unit")));
            }
        });
    }
});

// ---------------------------------------------------------------------------
// Logical properties, units and keywords

const PHYSICAL: &[(&str, &str)] = &[
    ("margin-left", "margin-inline-start"), ("margin-right", "margin-inline-end"), ("margin-top", "margin-block-start"), ("margin-bottom", "margin-block-end"),
    ("padding-left", "padding-inline-start"), ("padding-right", "padding-inline-end"), ("padding-top", "padding-block-start"), ("padding-bottom", "padding-block-end"),
    ("border-left", "border-inline-start"), ("border-right", "border-inline-end"), ("border-top", "border-block-start"), ("border-bottom", "border-block-end"),
    ("border-left-color", "border-inline-start-color"), ("border-right-color", "border-inline-end-color"), ("border-top-color", "border-block-start-color"), ("border-bottom-color", "border-block-end-color"),
    ("border-left-style", "border-inline-start-style"), ("border-right-style", "border-inline-end-style"), ("border-top-style", "border-block-start-style"), ("border-bottom-style", "border-block-end-style"),
    ("border-left-width", "border-inline-start-width"), ("border-right-width", "border-inline-end-width"), ("border-top-width", "border-block-start-width"), ("border-bottom-width", "border-block-end-width"),
    ("border-top-left-radius", "border-start-start-radius"), ("border-top-right-radius", "border-start-end-radius"), ("border-bottom-left-radius", "border-end-start-radius"), ("border-bottom-right-radius", "border-end-end-radius"),
    ("left", "inset-inline-start"), ("right", "inset-inline-end"), ("top", "inset-block-start"), ("bottom", "inset-block-end"),
    ("width", "inline-size"), ("height", "block-size"), ("min-width", "min-inline-size"), ("min-height", "min-block-size"), ("max-width", "max-inline-size"), ("max-height", "max-block-size"),
    ("overflow-x", "overflow-inline"), ("overflow-y", "overflow-block"), ("scroll-margin-left", "scroll-margin-inline-start"), ("scroll-margin-right", "scroll-margin-inline-end"),
    ("scroll-margin-top", "scroll-margin-block-start"), ("scroll-margin-bottom", "scroll-margin-block-end"), ("scroll-padding-left", "scroll-padding-inline-start"),
    ("scroll-padding-right", "scroll-padding-inline-end"), ("scroll-padding-top", "scroll-padding-block-start"), ("scroll-padding-bottom", "scroll-padding-block-end"),
    ("contain-intrinsic-width", "contain-intrinsic-inline-size"), ("contain-intrinsic-height", "contain-intrinsic-block-size"),
];

css_rule!(PropertyLayoutMappings, "css/property-layout-mappings", |sheet, src, ctx, out| {
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        if let Some((_, logical)) = PHYSICAL.iter().find(|x| x.0 == p) {
            let n = &sheet.nodes[i];
            out.push(fixed(n.name.0, n.name.1, format!("Use the flow-relative `{logical}`, not the physical `{p}`"), vec![TextEdit::new(n.name.0, n.name.1, *logical)]));
        }
    }
});

const UNITS: &[(&str, &str)] = &[("vw", "vi"), ("vh", "vb"), ("svw", "svi"), ("svh", "svb"), ("lvw", "lvi"), ("lvh", "lvb"), ("dvw", "dvi"), ("dvh", "dvb"), ("cqw", "cqi"), ("cqh", "cqb")];

css_rule!(UnitLayoutMappings, "css/unit-layout-mappings", |sheet, src, ctx, out| {
    for i in sheet.decls() {
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Word {
                return;
            }
            let w = v.text(src);
            let Some((num, u)) = dimension(w) else { return };
            if let Some((_, l)) = UNITS.iter().find(|x| x.0.eq_ignore_ascii_case(u)) {
                out.push(fixed(v.start, v.end, format!("Use the flow-relative unit `{l}`, not the physical `{u}`"), vec![TextEdit::new(v.start, v.end, format!("{num}{l}"))]));
            }
        });
    }
});

css_rule!(ValueKeywordLayoutMappings, "css/value-keyword-layout-mappings", |sheet, src, ctx, out| {
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        let n = &sheet.nodes[i];
        let v = text(src, n.params).to_ascii_lowercase();
        let new = match (p.as_str(), v.as_str()) {
            ("float" | "clear", "left") => "inline-start",
            ("float" | "clear", "right") => "inline-end",
            ("text-align", "left") => "start",
            ("text-align", "right") => "end",
            ("resize", "horizontal") => "inline",
            ("resize", "vertical") => "block",
            ("caption-side", "top") => "block-start",
            ("caption-side", "bottom") => "block-end",
            _ => continue,
        };
        out.push(fixed(n.params.0, n.params.1, format!("Use the flow-relative `{p}: {new}`, not the physical `{v}`"), vec![TextEdit::new(n.params.0, n.params.1, new)]));
    }
});

const KEYWORDS: &[&str] = &[
    "absolute", "auto", "baseline", "block", "bold", "bolder", "both", "bottom", "break-word", "capitalize", "center", "collapse", "column", "contain", "content-box", "border-box",
    "cover", "currentcolor", "dashed", "default", "dotted", "double", "ease", "ease-in", "ease-in-out", "ease-out", "ellipsis", "end", "fixed", "flex", "flex-end", "flex-start", "grid",
    "hidden", "inherit", "initial", "inline", "inline-block", "inline-flex", "inline-grid", "italic", "justify", "left", "lighter", "linear", "lowercase", "middle", "no-repeat", "none",
    "normal", "nowrap", "pointer", "pre", "pre-line", "pre-wrap", "relative", "repeat", "revert", "right", "row", "scroll", "solid", "space-around", "space-between", "space-evenly",
    "start", "static", "sticky", "stretch", "table", "top", "transparent", "underline", "unset", "uppercase", "visible", "wrap",
];

css_rule!(ValueKeywordCase, "css/value-keyword-case", |sheet, src, ctx, out| {
    let named: BTreeSet<&str> = NAMED.iter().map(|x| x.0).collect();
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        if !color_decl(&p) || p.starts_with("grid") {
            continue;
        }
        for v in sheet.values(i, src).iter() {
            if v.kind != VKind::Word {
                continue;
            }
            let w = v.text(src);
            let lower = w.to_ascii_lowercase();
            if w != lower && (KEYWORDS.contains(&lower.as_str()) || named.contains(lower.as_str())) && !(lower == "currentcolor" && w == "currentColor") {
                out.push(fixed(v.start, v.end, format!("Write the keyword `{w}` in lower case"), vec![TextEdit::new(v.start, v.end, lower)]));
            }
        }
    }
});

css_rule!(TimeMinMilliseconds, "css/time-min-milliseconds", |sheet, src, ctx, out| {
    let min = ctx.options.get("min").and_then(|v| v.as_f64()).unwrap_or(100.0);
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        if !(p.contains("transition") || p.contains("animation")) || p.ends_with("delay") {
            continue;
        }
        for v in sheet.values(i, src).iter() {
            if v.kind != VKind::Word {
                continue;
            }
            let Some((num, u)) = dimension(v.text(src)) else { continue };
            let Ok(n) = num.parse::<f64>() else { continue };
            let ms = match u.to_ascii_lowercase().as_str() {
                "ms" => n,
                "s" => n * 1000.0,
                _ => continue,
            };
            if ms > 0.0 && ms < min {
                out.push(finding(v.start, v.end, format!("`{}` is shorter than {min}ms, too short to notice", v.text(src))));
                break;
            }
        }
    }
});

// ---------------------------------------------------------------------------
// Selectors

fn simples(sheet: &Sheet, src: &str, i: usize) -> Vec<Simple> {
    let mut v = Vec::new();
    all_simples(src, &sheet.selectors(i, src), &mut v);
    v
}

css_rule!(RelativeSelectorNestingNotation, "css/relative-selector-nesting-notation", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        if !sheet.ancestors(i).any(|a| sheet.nodes[a].kind == Kind::Rule) {
            continue;
        }
        for c in sheet.selectors(i, src).iter() {
            if let Some(l) = &c.leading
                && !l.trim().is_empty()
            {
                out.push(fixed(c.start, c.end, format!("Write the relative selector with an explicit `&`: `& {}`", text(src, (c.start, c.end)).trim()), vec![TextEdit::insert(c.start, "& ")]));
            }
        }
    }
});

css_rule!(SelectorAttributeQuotes, "css/selector-attribute-quotes", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == SKind::Attribute) {
            let inner = &src[s.start + 1..s.end - 1];
            let Some(eq) = inner.find('=') else { continue };
            let raw = &inner[eq + 1..];
            let mut value = raw.trim();
            let flag_split = value.rfind(char::is_whitespace);
            if let Some(k) = flag_split
                && matches!(value[k..].trim(), "i" | "s" | "I" | "S")
            {
                value = value[..k].trim();
            }
            if value.is_empty() || value.starts_with(['"', '\'']) {
                continue;
            }
            let vs = s.start + 1 + eq + 1 + (raw.len() - raw.trim_start().len());
            out.push(fixed(vs, vs + value.len(), format!("Quote the attribute value `{value}`"), vec![TextEdit::new(vs, vs + value.len(), format!("\"{value}\""))]));
        }
    }
});

css_rule!(SelectorNoDeprecated, "css/selector-no-deprecated", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == SKind::PseudoClass) {
            let name = text(src, s.name).to_ascii_lowercase();
            if matches!(name.as_str(), "matches" | "-webkit-any" | "-moz-any" | "any") && s.args.is_some() {
                out.push(fixed(s.start, s.name.1, format!("`:{name}()` is deprecated; write `:is()`"), vec![TextEdit::new(s.name.0, s.name.1, "is")]));
            }
        }
    }
});

css_rule!(SelectorNoInvalid, "css/selector-no-invalid", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        let n = &sheet.nodes[i];
        let t = text(src, n.name);
        let mut depth: i32 = 0;
        let mut bracket: i32 = 0;
        let mut bad = None;
        for c in t.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '[' => bracket += 1,
                ']' => bracket -= 1,
                _ => {}
            }
            if depth < 0 || bracket < 0 {
                bad = Some("an unmatched closing bracket");
            }
        }
        if depth != 0 || bracket != 0 {
            bad = bad.or(Some("an unclosed bracket"));
        }
        if t.split(',').any(|p| p.trim().is_empty()) {
            bad = bad.or(Some("an empty selector in the list"));
        }
        let trimmed = t.trim_end();
        if trimmed.ends_with(['>', '+', '~']) {
            bad = bad.or(Some("a combinator with nothing after it"));
        }
        if t.contains("..") || t.contains("##") {
            bad = bad.or(Some("a doubled `.` or `#`"));
        }
        if let Some(why) = bad {
            out.push(finding(n.name.0, n.name.1, format!("`{t}` is not a valid selector: {why}")));
        }
    }
});

css_rule!(SelectorNoQualifyingType, "css/selector-no-qualifying-type", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            for comp in &c.compounds {
                let Some(ty) = comp.parts.iter().find(|s| s.kind == SKind::Type) else { continue };
                if let Some(q) = comp.parts.iter().find(|s| matches!(s.kind, SKind::Class | SKind::Id | SKind::Attribute)) {
                    let what = match q.kind {
                        SKind::Class => "class",
                        SKind::Id => "id",
                        _ => "attribute",
                    };
                    out.push(fixed(ty.start, ty.end, format!("Don't qualify the {what} selector `{}` with the type `{}`", text(src, (q.start, q.end)), text(src, (ty.start, ty.end))), vec![TextEdit::delete(ty.start, ty.end)]));
                }
            }
        }
    }
});

css_rule!(SelectorNoUnmatchable, "css/selector-no-unmatchable", |sheet, src, ctx, out| {
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            for comp in &c.compounds {
                let ids: BTreeSet<&str> = comp.parts.iter().filter(|s| s.kind == SKind::Id).map(|s| text(src, s.name)).collect();
                let types: BTreeSet<String> = comp.parts.iter().filter(|s| s.kind == SKind::Type).map(|s| text(src, s.name).to_ascii_lowercase()).collect();
                let pe = comp.parts.iter().position(|s| s.kind == SKind::PseudoElement);
                let after_pe = pe.is_some_and(|k| comp.parts[k + 1..].iter().any(|s| matches!(s.kind, SKind::Class | SKind::Id | SKind::Type | SKind::Attribute)));
                let why = if ids.len() > 1 {
                    Some("an element has one id")
                } else if types.len() > 1 {
                    Some("an element has one type")
                } else if after_pe {
                    Some("nothing can follow a pseudo-element in the same compound")
                } else {
                    None
                };
                if let Some(why) = why {
                    out.push(finding(comp.start, comp.end, format!("`{}` can never match: {why}", text(src, (comp.start, comp.end)))));
                }
            }
        }
    }
});

css_rule!(SelectorTypeCase, "css/selector-type-case", |sheet, src, ctx, out| {
    let svg = set(ctx.data, "css/type-selectors", "svg");
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == SKind::Type) {
            let name = text(src, s.name);
            if name.contains('|') || svg.contains(name) {
                continue;
            }
            let lower = name.to_ascii_lowercase();
            if lower != name {
                out.push(fixed(s.name.0, s.name.1, format!("Write the type selector `{name}` in lower case"), vec![TextEdit::new(s.name.0, s.name.1, lower)]));
            }
        }
    }
});

pub static RULES: &[&dyn crate::NativeRule] = &[
    &CommentWhitespaceInside,
    &ColorHexAlpha,
    &ColorNamed,
    &ColorNoHex,
    &ColorNoInvalidHex,
    &DisplayNotation,
    &FontFamilyNameQuotes,
    &FontWeightNotation,
    &FunctionLinearGradientNoNonstandardDirection,
    &FunctionNameCase,
    &FunctionNoUnknown,
    &FunctionUrlNoSchemeRelative,
    &FunctionUrlQuotes,
    &NoUnknownAnimations,
    &NoUnknownCustomMedia,
    &NoUnknownCustomProperties,
    &UnitNoUnknown,
    &PropertyLayoutMappings,
    &UnitLayoutMappings,
    &ValueKeywordLayoutMappings,
    &ValueKeywordCase,
    &TimeMinMilliseconds,
    &RelativeSelectorNestingNotation,
    &SelectorAttributeQuotes,
    &SelectorNoDeprecated,
    &SelectorNoInvalid,
    &SelectorNoQualifyingType,
    &SelectorNoUnmatchable,
    &SelectorTypeCase,
];
