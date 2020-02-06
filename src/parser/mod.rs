use std::rc::Rc;
use nom::{IResult, InputTakeAtPosition};
use regex::Regex;
use nom::character::complete::{space0, char, alphanumeric0, space1, one_of};
use nom::bytes::complete::{take_while1, escaped, is_not, tag};
use nom::error::{ParseError, ErrorKind};
use nom::character::is_space;
use nom::Err;
use nom::combinator::opt;
use nom::multi::many0;
use nom::branch::alt;

const ELEMENT_START: u8 = 0;
const ELEMENT_END: u8 = 1;
const ELEMENT_COMPLETE: u8 = 2;

const ELEMENT_JUDGE_REGEX: &str = r#"^<[a-zA-Z0-9]{1,}\s[\s\S]*"#;

lazy_static! {
    static ref ELEMENT_JUDGE: Regex = Regex::new(ELEMENT_JUDGE_REGEX).unwrap();
}

struct DomTree<'a> {
    pre: Option<Rc<DomTree<'a>>>,
    node: Vec<DomNodeType<'a>>,
    next: Option<Rc<DomTree<'a>>>
}

impl <'a> DomTree<'a> {
    /// 解析html字符串，将其转换为DomTree对象集合
    pub fn analyze_html(html: &'a mut str) -> Vec<Rc<Self>> {
        let html = html.replace("\n", " ");
        let mut v = Vec::<Rc<DomTree>>::new();

        let mut has_tag = Vec::<&str>::new();
        let mut level = 0_u8;

        let mut input = html.as_str();
        while !input.is_empty() {

        }

        v
    }
}

enum DomNodeType<'a> {
    ElementStart(DomElement<'a>),
    ElementEnd(&'a str),
    ElementComplete(DomElement<'a>),
    Content(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
struct DomElement<'a> {
    tag: &'a str,
    attr: Vec<(&'a str, Option<&'a str>)>,
    style: Vec<(&'a str, Option<&'a str>)>
}

impl <'a> DomElement<'a> {
    fn new(tag: &'a str) -> Self {
        DomElement {
            tag,
            attr: Vec::new(),
            style: Vec::new()
        }
    }

    /// 插入属性
    fn push_attr(&mut self, k: &'a str, v: Option<&'a str>) {
        self.attr.push((k, v));
    }

    /// 插入样式
    fn push_style(&mut self, k: &'a str, v: Option<&'a str>) {
        self.style.push((k, v));
    }
}

fn while_element1(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(
        |item| is_element_char(item), ErrorKind::Space
    )
}

fn is_element_char(chr: char) -> bool {
    is_space(chr as u8) || chr == '>'
        || chr == '=' || chr == '/'
}

//    /// 一个个的处理node
//    fn parser_node(line: &'a str) -> IResult<&'a str, DomNodeType> {
//        let (input, _) = space0(input)?;
//        // 验证是否是html标签
//        if ELEMENT_JUDGE.is_match(input) {
//
//        } else {
//
//        }
//    }

/// 解析element
fn parser_element(input: &str) -> IResult<&str, (DomElement, &str)> {
    let (input, _) = char('<')(input)?;
    let (input, t) = while_element1(input)?;
    let (input, attrs) = many0(parser_attr)(input)?;
    let (input, _) = space0(input)?;
    let (input, end_tag) = alt((tag("/>"), tag(">")))(input)?;

    let mut de = DomElement::new(t);
    let attrs: Vec<(&str, Option<&str>)> = attrs;
    for (key, value) in attrs {
        de.push_attr(key, value);
    }

    Ok((input, (de, end_tag)))
}

/// 解析attribute
fn parser_attr(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let (input, _) = space1(input)?;
    let (input, key) = while_element1(input)?;
    let (input, value) = opt(parser_attr_value)(input)?;

    Ok((input, (key, value)))
}

/// 解析value的格式
/// 有可能存在也可能不存在
/// 存在的情况下有使用"或',在这种情况下，获得"和'中的文本即可
/// 还有直接一个文本，没有任何包含，这种的遇到空格，或/和>即可
fn parser_attr_value(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = space0(input)?;

    let next_char = match input.chars().next() {
        Some(c) => c,
        None => return Err(Err::Error(ParseError::from_error_kind(input, ErrorKind::NoneOf)))
    };

    if next_char == '\'' || next_char == '"' {
        let (input, _) = char(next_char)(input)?;
        let s = next_char.to_string() + "\\";
        let (input, value) = escaped(is_not(s.clone().as_str()), '\\',
                                     one_of(s.as_str()))(input)?;
        let (input, _) = char(next_char)(input)?;

        Ok(((input, value)))
    } else {
        let (input, value) = while_element1(input)?;
        Ok(((input, value)))
    }
}

#[test]
fn test_element() {
    let s = r#"<div id="id1" name="name1" >"#;

    let actual: IResult<&str, (DomElement, &str)> = parser_element(s);

    let mut expected = DomElement::new("div");
    expected.push_attr("id", Some("id1"));
    expected.push_attr("name", Some("name1"));
    assert_eq!(actual, Ok(("", (expected, ">"))));
}