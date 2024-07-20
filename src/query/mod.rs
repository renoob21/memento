use std::{char, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

#[derive(Debug)]
pub enum Query {
    Get {
        key: String,
    },
    Add {
        key: String,
        value: String,
    }
}

impl Into<Query> for String {
    fn into(self) -> Query {
        let parse_result = parse_query().parse(&self);

        let (_, query) = parse_result.expect("Unable to parse query");

        query
    }
}

impl Query {
    pub fn from_string(bits: String) -> Self {
        bits.into()
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { key, value } => {
                writeln!(f, "<ADD: (\"{}\", \"{}\")>", key, value)
            }
            Self::Get { key } => {
                writeln!(f, "<GET: (\"{}\")>", key)
            }
        }
    }
}


pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> BoxedParser<'a, Output> 
    where
        P: Parser<'a, Output> + 'a
    {
        let parser = Box::new(parser);
        BoxedParser {
            parser,
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input:  &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input:  &'a str) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput> 
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where 
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
        {
            BoxedParser::new(pred(self, pred_fn))
        }
    
    fn and_then<F, NewOutput, NextParser>(self, f: F) -> impl Parser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NextParser + 'a,
        NextParser: Parser<'a, NewOutput> + 'a
    {
        BoxedParser::new(and_then(self, f))
    }

}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input:  &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}


// fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
//     move |input| match input.get(0..expected.len()) {
//         Some(next) if next == expected => {
//             Ok((&input[expected.len()..], ()))
//         }
//         _ => Err(input),
//     }
// }

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a,()> {
    move |input: &'a str| if input.starts_with(expected) {
        Ok((&input[expected.len()..], ()))
    } else {
        Err(input)
    }
}

fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphabetic() || next == '-' {
            matched.push(next)
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}


fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)> 
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, r1)| {
            parser2.parse(next_input)
                .map(|(final_input, r2)| {(final_input, (r1, r2))})
        })
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1> 
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B> 
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>> 
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        };

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }

}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>
{
    move |mut input| {
        let mut result = Vec::new();
        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        };

        Ok((input, result))
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

fn pred<'a, P, F, A>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
    
        };

        Err(input)
    }

}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

// fn quoted_string<'a>() -> impl Parser<'a, String> {
//     map(
//         left(
//             right(
//                 match_literal("\""),
//                 zero_or_more(pred(any_char, |c| *c != '"'))
//             ),
//             match_literal("\"")
//             ),
//         |chars| chars.into_iter().collect()
//     )
// }

fn quoted_string<'a>() -> impl Parser<'a, String> {
    right(
        match_literal("\""),
        left(zero_or_more(any_char.pred(|c| *c != '"')), match_literal("\""))
    ).map(|chars| chars.into_iter().collect())
}

pub fn parse_key<'a>() -> impl Parser<'a, String> {
    right(
        match_literal("("),
        right(
            space0(),
            left(quoted_string(), pair(space0(), match_literal(")")))
        )
    )
}

pub fn parse_key_val<'a>() -> impl Parser<'a, (String, String)> {
    right(
        match_literal("("),
        right(
            space0(),
            pair(
                left(quoted_string(), pair(pair(space0(), match_literal(",")), space0())),
                left(quoted_string(), pair(space0(), match_literal(")")))
            )
        )
    )
}

pub fn parse_get<'a>() -> impl Parser<'a, Query> {
    right(
        match_literal("<"),
        pair(
            left(
                identifier.pred(|output| output.to_lowercase() == String::from("get")),
                pair(space0(), pair(match_literal(":"), space0()))
            ),
            left(parse_key(), pair(space0(), match_literal(">")))
        )
        
    ).map(|output| {
        let (_, key) = output;
        Query::Get { key }
    })
}

pub fn parse_add<'a>() -> impl Parser<'a, Query> {
    right(
        match_literal("<"),
        pair(
            left(
                identifier.pred(|output| output.to_lowercase() == String::from("add")),
                pair(space0(), pair(match_literal(":"), space0()))
            ),
            left(parse_key_val(), pair(space0(), match_literal(">")))
        )
        
    ).map(|output| {
        let (_, (key, value)) = output;

        Query::Add { key, value }
    })
}

pub fn parse_query<'a>() -> impl Parser<'a, Query> {
    either(parse_get(), parse_add())
}


fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where 
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> NextP,
    NextP: Parser<'a, B>
{
    move |input| match parser.parse(input) {
        Ok((next_input, output)) => f(output).parse(next_input),
        Err(err) => Err(err),
    }
}

