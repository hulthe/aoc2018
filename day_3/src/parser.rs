use pest::Span;
use pest_derive::*;
use pest_ast::FromPest;

#[derive(Parser)]
#[grammar = "rectangles.pest"]
pub struct RectParser;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::int))]
pub struct Int {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub v: i32,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::id))]
pub struct ID {
    pub value: Int,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::coord))]
pub struct Coord {
    pub x: Int,
    pub y: Int,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::size))]
pub struct Size {
    pub w: Int,
    pub h: Int,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rect))]
pub struct Rectangle {
    pub id: ID,
    pub coord: Coord,
    pub size: Size,
}
