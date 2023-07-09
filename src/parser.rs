use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::IResult;
use nom::multi::separated_list1;

#[derive(Debug)]
pub struct Row {
    pub line: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub struct Rows {
    pub x: Vec<Row>,
    pub y: Vec<Row>,
}

pub fn parse(input: &str) -> IResult<&str, Rows> {
    let (input, raw_rows) = separated_list1(
        char('\n'),
        alt((
            parse_x,
            parse_y,
        )),
    )(input)?;

    let mut rows = Rows {
        x: Vec::new(),
        y: Vec::new(),
    };
    for (axis, row) in raw_rows {
        match axis {
            'x' => rows.x.push(row),
            'y' => rows.y.push(row),
            _ => panic!("Unknown axis"),
        }
    };
    Ok((input, rows))
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    let (input, number) = digit1(input)?;
    let parsed = number.parse().unwrap();
    Ok((input, parsed))
}

fn parse_x(input: &str) -> IResult<&str, (char, Row)> {
    let (input, _) = tag("x=")(input)?;
    let (input, line) = parse_number(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, from) = parse_number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, to) = parse_number(input)?;

    Ok((input, ('x', Row {
        line,
        from,
        to,
    })))
}

fn parse_y(input: &str) -> IResult<&str, (char, Row)> {
    let (input, _) = tag("y=")(input)?;
    let (input, line) = parse_number(input)?;
    let (input, _) = tag(", x=")(input)?;
    let (input, from) = parse_number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, to) = parse_number(input)?;

    Ok((input, ('y', Row {
        line,
        from,
        to,
    })))
}