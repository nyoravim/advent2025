use std::error::Error;

use advent25::read_input;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug)]
struct Problem {
    terms: Vec<u64>,
    op: Operation,
}

impl Problem {
    fn evaluate(&self) -> u64 {
        let initial = match &self.op {
            Operation::Addition => 0,
            Operation::Multiplication => 1,
        };

        self.terms
            .iter()
            .fold(initial, |prev, current| match &self.op {
                Operation::Addition => prev + current,
                Operation::Multiplication => prev * current,
            })
    }
}

struct ProblemIterator<'a> {
    digits: Vec<&'a str>,
    operator_string: Option<&'a str>,
}

impl<'a> ProblemIterator<'a> {
    fn new(input: &'a str) -> Result<ProblemIterator<'a>, Box<dyn Error>> {
        let lines: Vec<_> = input.lines().collect();

        if let Some((operators, tokens)) = lines.split_last() {
            Ok(ProblemIterator {
                digits: Vec::from(tokens),
                operator_string: Some(operators),
            })
        } else {
            Err("No operators!".into())
        }
    }
}

fn parse_operation(op: char) -> Option<Operation> {
    match op {
        '+' => Some(Operation::Addition),
        '*' => Some(Operation::Multiplication),
        _ => None,
    }
}

fn characters_until_operator(input: &str) -> Option<usize> {
    if let Some((n, _)) = input
        .char_indices()
        .filter(|(_, c)| parse_operation(*c).is_some())
        .nth(0)
    {
        Some(n)
    } else {
        None
    }
}

fn element_at<T: Clone>(data: &[Option<T>], index: usize) -> Option<T> {
    if index >= data.len() {
        None
    } else {
        data[index].clone()
    }
}

impl<'a> Iterator for ProblemIterator<'a> {
    type Item = Problem;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(remaining_operators) = &self.operator_string else {
            return None;
        };

        let (op, remainder) = remaining_operators.split_at(1);
        let assumed_term_count = characters_until_operator(remainder);

        self.operator_string = assumed_term_count.map(|count| {
            let (_, op_string) = remainder.split_at(count);
            op_string
        });

        let mut max_length = 0;
        let digit_count = self.digits.len();
        let digits: Vec<Vec<_>> = (0..digit_count)
            .into_iter()
            .map(|digit_index| {
                let digit_line = &mut self.digits[digit_index];

                let parsable_digits = match assumed_term_count {
                    None => digit_line,
                    Some(count) => {
                        if count > digit_line.len() {
                            panic!("Unexpected end of line!");
                        }

                        let (parsable, tail) = digit_line.split_at(count);
                        let (_, remaining_digits) = tail.split_at(1);

                        *digit_line = remaining_digits;

                        parsable
                    }
                };

                max_length = parsable_digits.len().max(max_length);
                parsable_digits.chars().map(|c| c.to_digit(10)).collect()
            })
            .collect();

        let term_count = assumed_term_count.unwrap_or(max_length);
        let terms = (0..term_count)
            .map(|term_index| {
                digits.iter().fold(0, |value, line| {
                    if let Some(digit) = element_at(line, term_index) {
                        value * 10 + digit as u64
                    } else {
                        value
                    }
                })
            })
            .collect();

        let op = parse_operation(op.chars().nth(0).unwrap()).expect("Invalid operator!");
        Some(Problem { terms, op })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(6)?;
    let sum = ProblemIterator::new(&input)?
        .map(|problem| problem.evaluate())
        .sum::<u64>();

    println!("Sum: {sum}");
    Ok(())
}
