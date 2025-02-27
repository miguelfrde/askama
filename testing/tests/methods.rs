use askama::Template;

#[derive(Template)]
#[template(source = "{{ self.get_s() }}", ext = "txt")]
struct SelfMethodTemplate<'a> {
    s: &'a str,
}

impl SelfMethodTemplate<'_> {
    fn get_s(&self) -> &str {
        self.s
    }
}

#[test]
fn test_self_method() {
    let t = SelfMethodTemplate { s: "foo" };
    assert_eq!(t.render().unwrap(), "foo");
}

#[derive(Template)]
#[template(source = "{{ self.type() }}", ext = "txt")]
struct SelfRawIdentifierMethodTemplate {}

impl SelfRawIdentifierMethodTemplate {
    fn r#type(&self) -> &str {
        "foo"
    }
}

#[test]
fn test_self_raw_identifier_method() {
    let t = SelfRawIdentifierMethodTemplate {};
    assert_eq!(t.render().unwrap(), "foo");
}

#[derive(Template)]
#[template(source = "{{ self.get_s() }} {{ t.get_s() }}", ext = "txt")]
struct NestedSelfMethodTemplate<'a> {
    t: SelfMethodTemplate<'a>,
}

impl NestedSelfMethodTemplate<'_> {
    fn get_s(&self) -> &str {
        "bar"
    }
}

#[test]
fn test_nested() {
    let t = NestedSelfMethodTemplate {
        t: SelfMethodTemplate { s: "foo" },
    };
    assert_eq!(t.render().unwrap(), "bar foo");
}
