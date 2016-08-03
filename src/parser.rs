#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use binaryoperation::{BinaryOperation, BinaryOp};
use ast_executor::AstExecutor;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Stmts {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use binaryoperation::{BinaryOperation, BinaryOp};
    use ast_executor::AstExecutor;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Stmts<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Box<AstExecutor>>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Stmts(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Expr(Box<AstExecutor>),
        ExprOp(BinaryOp),
        Factor(Box<AstExecutor>),
        FactorOp(BinaryOp),
        Num(i32),
        Stmt(Box<AstExecutor>),
        Stmts(Vec<Box<AstExecutor>>),
        Term(Box<AstExecutor>),
        ____Stmts(Vec<Box<AstExecutor>>),
    }

    // State 0
    //   Expr = (*) Expr ExprOp Factor ["+"]
    //   Expr = (*) Expr ExprOp Factor ["-"]
    //   Expr = (*) Expr ExprOp Factor [";"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Expr = (*) Factor [";"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Factor FactorOp Term [";"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) Term [";"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Num = (*) r#"-?[0-9]+"# [";"]
    //   Stmt = (*) Expr ";" [EOF]
    //   Stmt = (*) Expr ";" ["("]
    //   Stmt = (*) Expr ";" [r#"-?[0-9]+"#]
    //   Stmts = (*) Stmt [EOF]
    //   Stmts = (*) Stmt ["("]
    //   Stmts = (*) Stmt [r#"-?[0-9]+"#]
    //   Stmts = (*) Stmts Stmt [EOF]
    //   Stmts = (*) Stmts Stmt ["("]
    //   Stmts = (*) Stmts Stmt [r#"-?[0-9]+"#]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) Num [";"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "(" Expr ")" [";"]
    //   __Stmts = (*) Stmts [EOF]
    //
    //   "(" -> Shift(S7)
    //   r#"-?[0-9]+"# -> Shift(S8)
    //
    //   Expr -> S1
    //   Factor -> S2
    //   Num -> S3
    //   Stmt -> S4
    //   Stmts -> S5
    //   Term -> S6
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Stmt(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Stmts(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Expr = Expr (*) ExprOp Factor ["+"]
    //   Expr = Expr (*) ExprOp Factor ["-"]
    //   Expr = Expr (*) ExprOp Factor [";"]
    //   ExprOp = (*) "+" ["("]
    //   ExprOp = (*) "+" [r#"-?[0-9]+"#]
    //   ExprOp = (*) "-" ["("]
    //   ExprOp = (*) "-" [r#"-?[0-9]+"#]
    //   Stmt = Expr (*) ";" [EOF]
    //   Stmt = Expr (*) ";" ["("]
    //   Stmt = Expr (*) ";" [r#"-?[0-9]+"#]
    //
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //   ";" -> Shift(S12)
    //
    //   ExprOp -> S9
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ExprOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Expr = Factor (*) [";"]
    //   Factor = Factor (*) FactorOp Term ["%"]
    //   Factor = Factor (*) FactorOp Term ["*"]
    //   Factor = Factor (*) FactorOp Term ["+"]
    //   Factor = Factor (*) FactorOp Term ["-"]
    //   Factor = Factor (*) FactorOp Term ["/"]
    //   Factor = Factor (*) FactorOp Term [";"]
    //   FactorOp = (*) "%" ["("]
    //   FactorOp = (*) "%" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"-?[0-9]+"#]
    //
    //   "%" -> Shift(S14)
    //   "*" -> Shift(S15)
    //   "+" -> Reduce(Expr = Factor => ActionFn(5);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(5);)
    //   "/" -> Shift(S16)
    //   ";" -> Reduce(Expr = Factor => ActionFn(5);)
    //
    //   FactorOp -> S13
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FactorOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Term = Num (*) ["%"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //   Term = Num (*) [";"]
    //
    //   "%" -> Reduce(Term = Num => ActionFn(13);)
    //   "*" -> Reduce(Term = Num => ActionFn(13);)
    //   "+" -> Reduce(Term = Num => ActionFn(13);)
    //   "-" -> Reduce(Term = Num => ActionFn(13);)
    //   "/" -> Reduce(Term = Num => ActionFn(13);)
    //   ";" -> Reduce(Term = Num => ActionFn(13);)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Stmts = Stmt (*) [EOF]
    //   Stmts = Stmt (*) ["("]
    //   Stmts = Stmt (*) [r#"-?[0-9]+"#]
    //
    //   EOF -> Reduce(Stmts = Stmt => ActionFn(1);)
    //   "(" -> Reduce(Stmts = Stmt => ActionFn(1);)
    //   r#"-?[0-9]+"# -> Reduce(Stmts = Stmt => ActionFn(1);)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Stmts(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Expr = (*) Expr ExprOp Factor ["+"]
    //   Expr = (*) Expr ExprOp Factor ["-"]
    //   Expr = (*) Expr ExprOp Factor [";"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Expr = (*) Factor [";"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Factor FactorOp Term [";"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) Term [";"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Num = (*) r#"-?[0-9]+"# [";"]
    //   Stmt = (*) Expr ";" [EOF]
    //   Stmt = (*) Expr ";" ["("]
    //   Stmt = (*) Expr ";" [r#"-?[0-9]+"#]
    //   Stmts = Stmts (*) Stmt [EOF]
    //   Stmts = Stmts (*) Stmt ["("]
    //   Stmts = Stmts (*) Stmt [r#"-?[0-9]+"#]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) Num [";"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "(" Expr ")" [";"]
    //   __Stmts = Stmts (*) [EOF]
    //
    //   EOF -> Reduce(__Stmts = Stmts => ActionFn(0);)
    //   "(" -> Shift(S7)
    //   r#"-?[0-9]+"# -> Shift(S8)
    //
    //   Expr -> S1
    //   Factor -> S2
    //   Num -> S3
    //   Stmt -> S17
    //   Term -> S6
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Box<AstExecutor>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Stmts(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Stmt(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Factor = Term (*) ["%"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //   Factor = Term (*) [";"]
    //
    //   "%" -> Reduce(Factor = Term => ActionFn(9);)
    //   "*" -> Reduce(Factor = Term => ActionFn(9);)
    //   "+" -> Reduce(Factor = Term => ActionFn(9);)
    //   "-" -> Reduce(Factor = Term => ActionFn(9);)
    //   "/" -> Reduce(Factor = Term => ActionFn(9);)
    //   ";" -> Reduce(Factor = Term => ActionFn(9);)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Expr = (*) Expr ExprOp Factor [")"]
    //   Expr = (*) Expr ExprOp Factor ["+"]
    //   Expr = (*) Expr ExprOp Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term [")"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# [")"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" ["%"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [";"]
    //
    //   "(" -> Shift(S22)
    //   r#"-?[0-9]+"# -> Shift(S23)
    //
    //   Expr -> S18
    //   Factor -> S19
    //   Num -> S20
    //   Term -> S21
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Num = r#"-?[0-9]+"# (*) ["%"]
    //   Num = r#"-?[0-9]+"# (*) ["*"]
    //   Num = r#"-?[0-9]+"# (*) ["+"]
    //   Num = r#"-?[0-9]+"# (*) ["-"]
    //   Num = r#"-?[0-9]+"# (*) ["/"]
    //   Num = r#"-?[0-9]+"# (*) [";"]
    //
    //   "%" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "*" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "+" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "-" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "/" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   ";" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Expr = Expr ExprOp (*) Factor ["+"]
    //   Expr = Expr ExprOp (*) Factor ["-"]
    //   Expr = Expr ExprOp (*) Factor [";"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Factor FactorOp Term [";"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) Term [";"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Num = (*) r#"-?[0-9]+"# [";"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) Num [";"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "(" Expr ")" [";"]
    //
    //   "(" -> Shift(S7)
    //   r#"-?[0-9]+"# -> Shift(S8)
    //
    //   Factor -> S24
    //   Num -> S3
    //   Term -> S6
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   ExprOp = "+" (*) ["("]
    //   ExprOp = "+" (*) [r#"-?[0-9]+"#]
    //
    //   "(" -> Reduce(ExprOp = "+" => ActionFn(6);)
    //   r#"-?[0-9]+"# -> Reduce(ExprOp = "+" => ActionFn(6);)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ExprOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   ExprOp = "-" (*) ["("]
    //   ExprOp = "-" (*) [r#"-?[0-9]+"#]
    //
    //   "(" -> Reduce(ExprOp = "-" => ActionFn(7);)
    //   r#"-?[0-9]+"# -> Reduce(ExprOp = "-" => ActionFn(7);)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ExprOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Stmt = Expr ";" (*) [EOF]
    //   Stmt = Expr ";" (*) ["("]
    //   Stmt = Expr ";" (*) [r#"-?[0-9]+"#]
    //
    //   EOF -> Reduce(Stmt = Expr, ";" => ActionFn(3);)
    //   "(" -> Reduce(Stmt = Expr, ";" => ActionFn(3);)
    //   r#"-?[0-9]+"# -> Reduce(Stmt = Expr, ";" => ActionFn(3);)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Stmt(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Factor = Factor FactorOp (*) Term ["%"]
    //   Factor = Factor FactorOp (*) Term ["*"]
    //   Factor = Factor FactorOp (*) Term ["+"]
    //   Factor = Factor FactorOp (*) Term ["-"]
    //   Factor = Factor FactorOp (*) Term ["/"]
    //   Factor = Factor FactorOp (*) Term [";"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Num = (*) r#"-?[0-9]+"# [";"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) Num [";"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "(" Expr ")" [";"]
    //
    //   "(" -> Shift(S7)
    //   r#"-?[0-9]+"# -> Shift(S8)
    //
    //   Num -> S3
    //   Term -> S25
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   FactorOp = "%" (*) ["("]
    //   FactorOp = "%" (*) [r#"-?[0-9]+"#]
    //
    //   "(" -> Reduce(FactorOp = "%" => ActionFn(12);)
    //   r#"-?[0-9]+"# -> Reduce(FactorOp = "%" => ActionFn(12);)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FactorOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   FactorOp = "*" (*) ["("]
    //   FactorOp = "*" (*) [r#"-?[0-9]+"#]
    //
    //   "(" -> Reduce(FactorOp = "*" => ActionFn(10);)
    //   r#"-?[0-9]+"# -> Reduce(FactorOp = "*" => ActionFn(10);)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FactorOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   FactorOp = "/" (*) ["("]
    //   FactorOp = "/" (*) [r#"-?[0-9]+"#]
    //
    //   "(" -> Reduce(FactorOp = "/" => ActionFn(11);)
    //   r#"-?[0-9]+"# -> Reduce(FactorOp = "/" => ActionFn(11);)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FactorOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   Stmts = Stmts Stmt (*) [EOF]
    //   Stmts = Stmts Stmt (*) ["("]
    //   Stmts = Stmts Stmt (*) [r#"-?[0-9]+"#]
    //
    //   EOF -> Reduce(Stmts = Stmts, Stmt => ActionFn(2);)
    //   "(" -> Reduce(Stmts = Stmts, Stmt => ActionFn(2);)
    //   r#"-?[0-9]+"# -> Reduce(Stmts = Stmts, Stmt => ActionFn(2);)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Box<AstExecutor>>>,
        __sym1: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Stmts(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   Expr = Expr (*) ExprOp Factor [")"]
    //   Expr = Expr (*) ExprOp Factor ["+"]
    //   Expr = Expr (*) ExprOp Factor ["-"]
    //   ExprOp = (*) "+" ["("]
    //   ExprOp = (*) "+" [r#"-?[0-9]+"#]
    //   ExprOp = (*) "-" ["("]
    //   ExprOp = (*) "-" [r#"-?[0-9]+"#]
    //   Term = "(" Expr (*) ")" ["%"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //   Term = "(" Expr (*) ")" [";"]
    //
    //   ")" -> Shift(S27)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   ExprOp -> S26
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ExprOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 19
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) FactorOp Term ["%"]
    //   Factor = Factor (*) FactorOp Term [")"]
    //   Factor = Factor (*) FactorOp Term ["*"]
    //   Factor = Factor (*) FactorOp Term ["+"]
    //   Factor = Factor (*) FactorOp Term ["-"]
    //   Factor = Factor (*) FactorOp Term ["/"]
    //   FactorOp = (*) "%" ["("]
    //   FactorOp = (*) "%" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"-?[0-9]+"#]
    //
    //   "%" -> Shift(S14)
    //   ")" -> Reduce(Expr = Factor => ActionFn(5);)
    //   "*" -> Shift(S15)
    //   "+" -> Reduce(Expr = Factor => ActionFn(5);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(5);)
    //   "/" -> Shift(S16)
    //
    //   FactorOp -> S28
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FactorOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Term = Num (*) ["%"]
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "%" -> Reduce(Term = Num => ActionFn(13);)
    //   ")" -> Reduce(Term = Num => ActionFn(13);)
    //   "*" -> Reduce(Term = Num => ActionFn(13);)
    //   "+" -> Reduce(Term = Num => ActionFn(13);)
    //   "-" -> Reduce(Term = Num => ActionFn(13);)
    //   "/" -> Reduce(Term = Num => ActionFn(13);)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   Factor = Term (*) ["%"]
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "%" -> Reduce(Factor = Term => ActionFn(9);)
    //   ")" -> Reduce(Factor = Term => ActionFn(9);)
    //   "*" -> Reduce(Factor = Term => ActionFn(9);)
    //   "+" -> Reduce(Factor = Term => ActionFn(9);)
    //   "-" -> Reduce(Factor = Term => ActionFn(9);)
    //   "/" -> Reduce(Factor = Term => ActionFn(9);)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Expr = (*) Expr ExprOp Factor [")"]
    //   Expr = (*) Expr ExprOp Factor ["+"]
    //   Expr = (*) Expr ExprOp Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term [")"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# [")"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" ["%"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //
    //   "(" -> Shift(S22)
    //   r#"-?[0-9]+"# -> Shift(S23)
    //
    //   Expr -> S29
    //   Factor -> S19
    //   Num -> S20
    //   Term -> S21
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Num = r#"-?[0-9]+"# (*) ["%"]
    //   Num = r#"-?[0-9]+"# (*) [")"]
    //   Num = r#"-?[0-9]+"# (*) ["*"]
    //   Num = r#"-?[0-9]+"# (*) ["+"]
    //   Num = r#"-?[0-9]+"# (*) ["-"]
    //   Num = r#"-?[0-9]+"# (*) ["/"]
    //
    //   "%" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   ")" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "*" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "+" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "-" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //   "/" -> Reduce(Num = r#"-?[0-9]+"# => ActionFn(15);)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   Expr = Expr ExprOp Factor (*) ["+"]
    //   Expr = Expr ExprOp Factor (*) ["-"]
    //   Expr = Expr ExprOp Factor (*) [";"]
    //   Factor = Factor (*) FactorOp Term ["%"]
    //   Factor = Factor (*) FactorOp Term ["*"]
    //   Factor = Factor (*) FactorOp Term ["+"]
    //   Factor = Factor (*) FactorOp Term ["-"]
    //   Factor = Factor (*) FactorOp Term ["/"]
    //   Factor = Factor (*) FactorOp Term [";"]
    //   FactorOp = (*) "%" ["("]
    //   FactorOp = (*) "%" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"-?[0-9]+"#]
    //
    //   "%" -> Shift(S14)
    //   "*" -> Shift(S15)
    //   "+" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //   "-" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //   "/" -> Shift(S16)
    //   ";" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //
    //   FactorOp -> S13
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
        __sym2: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FactorOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Factor = Factor FactorOp Term (*) ["%"]
    //   Factor = Factor FactorOp Term (*) ["*"]
    //   Factor = Factor FactorOp Term (*) ["+"]
    //   Factor = Factor FactorOp Term (*) ["-"]
    //   Factor = Factor FactorOp Term (*) ["/"]
    //   Factor = Factor FactorOp Term (*) [";"]
    //
    //   "%" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "*" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "+" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "-" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "/" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   ";" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
        __sym2: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   Expr = Expr ExprOp (*) Factor [")"]
    //   Expr = Expr ExprOp (*) Factor ["+"]
    //   Expr = Expr ExprOp (*) Factor ["-"]
    //   Factor = (*) Factor FactorOp Term ["%"]
    //   Factor = (*) Factor FactorOp Term [")"]
    //   Factor = (*) Factor FactorOp Term ["*"]
    //   Factor = (*) Factor FactorOp Term ["+"]
    //   Factor = (*) Factor FactorOp Term ["-"]
    //   Factor = (*) Factor FactorOp Term ["/"]
    //   Factor = (*) Term ["%"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# [")"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S22)
    //   r#"-?[0-9]+"# -> Shift(S23)
    //
    //   Factor -> S30
    //   Num -> S20
    //   Term -> S21
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Term = "(" Expr ")" (*) ["%"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //   Term = "(" Expr ")" (*) [";"]
    //
    //   "%" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   ";" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<AstExecutor>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //   Factor = Factor FactorOp (*) Term ["%"]
    //   Factor = Factor FactorOp (*) Term [")"]
    //   Factor = Factor FactorOp (*) Term ["*"]
    //   Factor = Factor FactorOp (*) Term ["+"]
    //   Factor = Factor FactorOp (*) Term ["-"]
    //   Factor = Factor FactorOp (*) Term ["/"]
    //   Num = (*) r#"-?[0-9]+"# ["%"]
    //   Num = (*) r#"-?[0-9]+"# [")"]
    //   Num = (*) r#"-?[0-9]+"# ["*"]
    //   Num = (*) r#"-?[0-9]+"# ["+"]
    //   Num = (*) r#"-?[0-9]+"# ["-"]
    //   Num = (*) r#"-?[0-9]+"# ["/"]
    //   Term = (*) Num ["%"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" ["%"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S22)
    //   r#"-?[0-9]+"# -> Shift(S23)
    //
    //   Num -> S20
    //   Term -> S31
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Expr = Expr (*) ExprOp Factor [")"]
    //   Expr = Expr (*) ExprOp Factor ["+"]
    //   Expr = Expr (*) ExprOp Factor ["-"]
    //   ExprOp = (*) "+" ["("]
    //   ExprOp = (*) "+" [r#"-?[0-9]+"#]
    //   ExprOp = (*) "-" ["("]
    //   ExprOp = (*) "-" [r#"-?[0-9]+"#]
    //   Term = "(" Expr (*) ")" ["%"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S32)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   ExprOp -> S26
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ExprOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 30
    //   Expr = Expr ExprOp Factor (*) [")"]
    //   Expr = Expr ExprOp Factor (*) ["+"]
    //   Expr = Expr ExprOp Factor (*) ["-"]
    //   Factor = Factor (*) FactorOp Term ["%"]
    //   Factor = Factor (*) FactorOp Term [")"]
    //   Factor = Factor (*) FactorOp Term ["*"]
    //   Factor = Factor (*) FactorOp Term ["+"]
    //   Factor = Factor (*) FactorOp Term ["-"]
    //   Factor = Factor (*) FactorOp Term ["/"]
    //   FactorOp = (*) "%" ["("]
    //   FactorOp = (*) "%" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"-?[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"-?[0-9]+"#]
    //
    //   "%" -> Shift(S14)
    //   ")" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //   "*" -> Shift(S15)
    //   "+" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //   "-" -> Reduce(Expr = Expr, ExprOp, Factor => ActionFn(4);)
    //   "/" -> Shift(S16)
    //
    //   FactorOp -> S28
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
        __sym2: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FactorOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Factor = Factor FactorOp Term (*) ["%"]
    //   Factor = Factor FactorOp Term (*) [")"]
    //   Factor = Factor FactorOp Term (*) ["*"]
    //   Factor = Factor FactorOp Term (*) ["+"]
    //   Factor = Factor FactorOp Term (*) ["-"]
    //   Factor = Factor FactorOp Term (*) ["/"]
    //
    //   "%" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   ")" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "*" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "+" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "-" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //   "/" -> Reduce(Factor = Factor, FactorOp, Term => ActionFn(8);)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<AstExecutor>>,
        __sym1: &mut Option<BinaryOp>,
        __sym2: &mut Option<Box<AstExecutor>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 32
    //   Term = "(" Expr ")" (*) ["%"]
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "%" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(14);)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<AstExecutor>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Stmts::parse_Stmts;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '%' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '(' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Vec<Box<AstExecutor>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Box<AstExecutor>>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Box<AstExecutor>>
{
    vec![__0]
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    l: Vec<Box<AstExecutor>>,
    r: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Box<AstExecutor>>
{
    {
    	let mut result: Vec<Box<AstExecutor>> = vec![];
	result.extend(l);
	result.push(r);
	result
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    e: Box<AstExecutor>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    e
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: Box<AstExecutor>,
    __1: BinaryOp,
    __2: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    Box::new(BinaryOperation::new(__0, __1, __2))
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    (__0)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> BinaryOp
{
    BinaryOp::Add
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> BinaryOp
{
    BinaryOp::Sub
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: Box<AstExecutor>,
    __1: BinaryOp,
    __2: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    Box::new(BinaryOperation::new(__0, __1, __2))
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: Box<AstExecutor>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> BinaryOp
{
    BinaryOp::Mul
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> BinaryOp
{
    BinaryOp::Div
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> BinaryOp
{
    BinaryOp::Mod
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: i32,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    Box::new(__0)
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Box<AstExecutor>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Box<AstExecutor>
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> i32
{
    i32::from_str(__0).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
