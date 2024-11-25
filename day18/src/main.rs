use std::{env,fs,process};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Operator { Add, Prod, }

impl Operator {
    fn op(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Prod => a * b,
        }
    }

    // For Part 2
    fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Prod => 0,
        }
    }

    fn parse(tok: &str) -> Self {
        match tok {
            "+" => Self::Add,
            "*" => Self::Prod,
            _ => unreachable!(),
        }
    }
}

fn inner1(input: &str) -> Vec<u64> {
    let mut res = vec![];

    for (_line_num, line) in input.lines().enumerate() {
        let mut stack_nums = vec![(0, 0)];
        let mut stack_ops = vec![Operator::Add];
        let mut depth = 0;
        for mut token in line.split_whitespace() {
            if token == "+" || token == "*" {
                stack_ops.push(Operator::parse(token));
            } else if token.starts_with('(') {
                while let Some(s) = token.strip_prefix('(') {
                    depth += 1;
                    token = s;
                }
                assert!(token.chars().all(|c| c.is_ascii_digit()));
                stack_nums.push((token.parse().unwrap(), depth));
            } else {
                // We must operate with the stacked numbers to the left
                let mut operate = 1;
                while let Some(s) = token.strip_suffix(')') {
                    operate += 1;
                    token = s;
                }
                assert!(token.chars().all(|c| c.is_ascii_digit()));
                let mut n: u64 = token.parse().unwrap();

                while operate > 0 {
                    if stack_nums.last().unwrap().1 != depth { break; }
                    // We are allowed to operate
                    if let Some((left, _depth_left)) = stack_nums.pop() {
                        let op = stack_ops.pop().unwrap();
                        n = op.op(left, n);
                    }
                    operate -= 1;
                    if operate > 0 { depth -= 1; }
                }

                stack_nums.push((n, depth));
            }
        }
        assert_eq!(stack_nums.len(), 1);
        res.push(stack_nums.pop().unwrap().0);
    }

    res
}

fn run1(input: &str) -> u64 {
    inner1(input).into_iter().sum()
}

enum Token {
    Num(u64),
    Op(Operator),
    LeftParenthesis,
}

impl Token {
    fn new_num(n: u64) -> Self {
        Self::Num(n)
    }

    fn new_op(operator: Operator) -> Self {
        Self::Op(operator)
    }

    fn new_par() -> Self {
        Self::LeftParenthesis
    }
}

fn inner2(input: &str) -> Vec<u64> {
    let mut res = vec![];

    for (_line_num, line) in input.lines().enumerate() {
        // Shunting yard algorithm
        let mut output_queue = vec![];
        let mut operator_stack = vec![];
        for mut word in line.split_whitespace() {
            if word == "+" || word == "*" {
                // Push operator
                let o1 = Operator::parse(word);
                while let Some(op) = operator_stack.last() {
                    match op {
                        Token::Num(_) => unreachable!(),
                        Token::LeftParenthesis => break,
                        Token::Op(o2) => {
                            if o2.precedence() >= o1.precedence() {
                                output_queue.push(Token::new_op(*o2));
                                operator_stack.pop();
                            } else {
                                break;
                            }
                        },
                    }
                }
                operator_stack.push(Token::Op(o1));
            } else if word.starts_with('(') {
                // Push left parenthesis to op stack
                while let Some(s) = word.strip_prefix('(') {
                    word = s;
                    operator_stack.push(Token::new_par());
                }
                // Assert we have number, push to output
                assert!(word.chars().all(|c| c.is_ascii_digit()));
                output_queue.push(Token::new_num(word.parse().unwrap()));
            } else {
                // Number and possibly right parenthesis. Separate
                let mut destack = 0;
                while let Some(s) = word.strip_suffix(')') {
                    word = s;
                    destack += 1;
                }
                assert!(word.chars().all(|c| c.is_ascii_digit()));
                output_queue.push(Token::new_num(word.parse().unwrap()));
                
                while destack > 0 {
                    if let Some(op) = operator_stack.pop() {
                        match op {
                            Token::Num(_) => unreachable!(),
                            Token::LeftParenthesis => destack -= 1,
                            Token::Op(_) => output_queue.push(op),
                        }
                    } else {
                        panic!("Mismatched parenthesis in line {_line_num}");
                    }
                }
            }
        }
        // End. Append every operator remaining in the stack to the queue
        while let Some(op) = operator_stack.pop() {
            match op {
                Token::LeftParenthesis => panic!("Mismatched parenthesis in line {_line_num}"),
                _ => output_queue.push(op),
            }
        }

        let mut number_stack = vec![];
        for token in output_queue.into_iter() {
            match token {
                Token::LeftParenthesis => unreachable!(),
                Token::Num(n) => number_stack.push(n),
                Token::Op(op) => {
                    assert!(number_stack.len() > 1);
                    let right = number_stack.pop().unwrap();
                    let left = number_stack.pop().unwrap();
                    number_stack.push(op.op(left, right));
                },
            }
        }
        assert_eq!(number_stack.len(), 1);
        res.push(number_stack[0]);
    }

    res
}

fn run2(input: &str) -> u64 {
    inner2(input).into_iter().sum()
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = inner1(&input);
    assert_eq!(res, vec![71, 51, 26, 437, 12240, 13632, 208022]);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,209335026987);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = inner2(&input);
    assert_eq!(res, vec![231, 51, 46, 1445, 669060, 23340, 22985592]);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 33331817392479);
}
