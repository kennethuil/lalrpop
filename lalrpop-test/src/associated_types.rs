// auto-generated: "lalrpop 0.13.1"
use std::str::FromStr;
use associated_types_lib::ParseCallbacks;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use associated_types_lib::ParseCallbacks;
    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_Term<
        'input,
        P,
    >(
        callbacks: &mut P,
        input: &'input str,
    ) -> Result<P::Term, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
      P:  ParseCallbacks,
    {
        let __ascent = __ascent::parse_Term::<P>(
            callbacks,
            input,
        );
        let __parse_table = __parse_table::parse_Term::<P>(
            callbacks,
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Term {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::str::FromStr;
            use associated_types_lib::ParseCallbacks;
            extern crate lalrpop_util as __lalrpop_util;
            pub fn parse_Term<
                'input,
                P,
            >(
                callbacks: &mut P,
                input: &'input str,
            ) -> Result<P::Term, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(callbacks, input, &mut __tokens, __lookahead, ::std::marker::PhantomData::<(P)>)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Term((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<P>
             where P:  ParseCallbacks
             {
                Num((usize, P::Num, usize)),
                Term((usize, P::Term, usize)),
                ____Term((usize, P::Term, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Num = (*) r#"[0-9]+"# ["(", ")", r#"[0-9]+"#, EOF]
            //     Term = (*) Num ["(", ")", r#"[0-9]+"#, EOF]
            //     Term = (*) "(" Term ")" ["(", ")", r#"[0-9]+"#, EOF]
            //     __Term = (*) Term ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   "(" -> S3
            //   r#"[0-9]+"# -> S4
            //
            //     Num -> S1
            //     Term -> S2
            fn __state0<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(callbacks, input, __tokens, __sym0, ::std::marker::PhantomData::<(P)>));
                    }
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(callbacks, input, __tokens, __sym0, ::std::marker::PhantomData::<(P)>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Num(__sym0) => {
                            __result = try!(__state1(callbacks, input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(P)>));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state2(callbacks, input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(P)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   ["(", ")", r#"[0-9]+"#, EOF] -> Term = Num => ActionFn(1);
            //
            fn __state1<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, P::Num, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (0, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<P>(callbacks, input, __sym0);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"")""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 2
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(__Term)
            //
            //     __Term = Term (*) ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   ["(", ")", r#"[0-9]+"#, EOF] -> __Term = Term => ActionFn(0);
            //
            fn __state2<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, P::Term, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (0, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<P>(callbacks, input, __sym0);
                        let __nt = __Nonterminal::____Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"")""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 3
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [Term, ")"]
            //     WillProduce = Some(Term)
            //
            //     Num = (*) r#"[0-9]+"# ["(", ")", r#"[0-9]+"#, EOF]
            //     Term = (*) Num ["(", ")", r#"[0-9]+"#, EOF]
            //     Term = (*) "(" Term ")" ["(", ")", r#"[0-9]+"#, EOF]
            //     Term = "(" (*) Term ")" ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   "(" -> S3
            //   r#"[0-9]+"# -> S4
            //
            //     Num -> S1
            //     Term -> S5
            fn __state3<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(callbacks, input, __tokens, __sym1, ::std::marker::PhantomData::<(P)>));
                    }
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(callbacks, input, __tokens, __sym1, ::std::marker::PhantomData::<(P)>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Num(__sym1) => {
                            __result = try!(__state1(callbacks, input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<(P)>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state5(callbacks, input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<(P)>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 4
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Num)
            //
            //     Num = r#"[0-9]+"# (*) ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   ["(", ")", r#"[0-9]+"#, EOF] -> Num = r#"[0-9]+"# => ActionFn(3);
            //
            fn __state4<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (0, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<P>(callbacks, input, __sym0);
                        let __nt = __Nonterminal::Num((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"")""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 5
            //     AllInputs = ["(", Term]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Term]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Term (*) ")" ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   ")" -> S6
            //
            fn __state5<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, P::Term, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(callbacks, input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(P)>));
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 6
            //     AllInputs = ["(", Term, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Term, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Term ")" (*) ["(", ")", r#"[0-9]+"#, EOF]
            //
            //   ["(", ")", r#"[0-9]+"#, EOF] -> Term = "(", Term, ")" => ActionFn(2);
            //
            fn __state6<
                'input,
                P,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                callbacks: &mut P,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, P::Term, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(P)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<P>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (0, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<P>(callbacks, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###"")""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }
        }
        pub use self::__parse__Term::parse_Term;
    }
    mod __parse_table {

        mod __parse__Term {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::str::FromStr;
            use associated_types_lib::ParseCallbacks;
            extern crate lalrpop_util as __lalrpop_util;
            #[allow(dead_code)]
            pub enum __Symbol<'input, P>
             where P:  ParseCallbacks
             {
                Term_22_28_22(&'input str),
                Term_22_29_22(&'input str),
                Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
                NtNum(P::Num),
                NtTerm(P::Term),
                Nt____Term(P::Term),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Num = (*) r#"[0-9]+"# ["(", ")", r#"[0-9]+"#, EOF]
                //     Term = (*) Num ["(", ")", r#"[0-9]+"#, EOF]
                //     Term = (*) "(" Term ")" ["(", ")", r#"[0-9]+"#, EOF]
                //     __Term = (*) Term ["(", ")", r#"[0-9]+"#, EOF]
                4,  // on "(", goto 3
                0,  // on ")", error
                5,  // on r#"[0-9]+"#, goto 4

                // State 1
                //     Term = Num (*) ["(", ")", r#"[0-9]+"#, EOF]
                -2,  // on "(", reduce `Term = Num => ActionFn(1);`
                -2,  // on ")", reduce `Term = Num => ActionFn(1);`
                -2,  // on r#"[0-9]+"#, reduce `Term = Num => ActionFn(1);`

                // State 2
                //     __Term = Term (*) ["(", ")", r#"[0-9]+"#, EOF]
                -4,  // on "(", reduce `__Term = Term => ActionFn(0);`
                -4,  // on ")", reduce `__Term = Term => ActionFn(0);`
                -4,  // on r#"[0-9]+"#, reduce `__Term = Term => ActionFn(0);`

                // State 3
                //     Num = (*) r#"[0-9]+"# ["(", ")", r#"[0-9]+"#, EOF]
                //     Term = (*) Num ["(", ")", r#"[0-9]+"#, EOF]
                //     Term = (*) "(" Term ")" ["(", ")", r#"[0-9]+"#, EOF]
                //     Term = "(" (*) Term ")" ["(", ")", r#"[0-9]+"#, EOF]
                4,  // on "(", goto 3
                0,  // on ")", error
                5,  // on r#"[0-9]+"#, goto 4

                // State 4
                //     Num = r#"[0-9]+"# (*) ["(", ")", r#"[0-9]+"#, EOF]
                -1,  // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(3);`
                -1,  // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(3);`
                -1,  // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(3);`

                // State 5
                //     Term = "(" Term (*) ")" ["(", ")", r#"[0-9]+"#, EOF]
                0,  // on "(", error
                7,  // on ")", goto 6
                0,  // on r#"[0-9]+"#, error

                // State 6
                //     Term = "(" Term ")" (*) ["(", ")", r#"[0-9]+"#, EOF]
                -3,  // on "(", reduce `Term = "(", Term, ")" => ActionFn(2);`
                -3,  // on ")", reduce `Term = "(", Term, ")" => ActionFn(2);`
                -3,  // on r#"[0-9]+"#, reduce `Term = "(", Term, ")" => ActionFn(2);`

            ];
            const __EOF_ACTION: &'static [i32] = &[
                0,  // on EOF, error

                -2,  // on EOF, reduce `Term = Num => ActionFn(1);`

                -4,  // on EOF, reduce `__Term = Term => ActionFn(0);`

                0,  // on EOF, error

                -1,  // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(3);`

                0,  // on EOF, error

                -3,  // on EOF, reduce `Term = "(", Term, ")" => ActionFn(2);`

            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2,  // on Num, goto 1
                3,  // on Term, goto 2
                0,  // on __Term, error

                // State 1
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 2
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 3
                2,  // on Num, goto 1
                6,  // on Term, goto 5
                0,  // on __Term, error

                // State 4
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 5
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 6
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

            ];
            fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
                const __TERMINAL: &'static [&'static str] = &[
                    r###""(""###,
                    r###"")""###,
                    r###"r#"[0-9]+"#"###,
                ];
                __ACTION[(__state * 3)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
                    if state == 0 {
                        None
                    } else {
                        Some(terminal.to_string())
                    }
                }).collect()
            }
            pub fn parse_Term<
                'input,
                P,
            >(
                callbacks: &mut P,
                input: &'input str,
            ) -> Result<P::Term, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>> where
              P:  ParseCallbacks,
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let mut __states = vec![0_i32];
                let mut __symbols = vec![];
                let mut __integer;
                let mut __lookahead;
                let mut __last_location = Default::default();
                '__shift: loop {
                    __lookahead = match __tokens.next() {
                        Some(Ok(v)) => v,
                        None => break '__shift,
                        Some(Err(e)) => return Err(e),
                    };
                    __last_location = __lookahead.2.clone();
                    __integer = match __lookahead.1 {
                        (1, _) if true => 0,
                        (2, _) if true => 1,
                        (0, _) if true => 2,
                        _ => {
                            let __state = *__states.last().unwrap() as usize;
                            let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: __expected_tokens(__state),
                            };
                            return Err(__error);
                        }
                    };
                    '__inner: loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 3 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(_) = __reduce(callbacks, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<(P)>) {
                                return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                            }
                        } else {
                            let __state = *__states.last().unwrap() as usize;
                            let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: __expected_tokens(__state),
                            };
                            return Err(__error)
                        }
                    }
                }
                loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __EOF_ACTION[__state];
                    if __action < 0 {
                        if let Some(r) = __reduce(callbacks, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<(P)>) {
                            return r;
                        }
                    } else {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: None,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                }
            }
            pub fn __reduce<
                'input,
                P,
            >(
                callbacks: &mut P,
                input: &'input str,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>,
                _: ::std::marker::PhantomData<(P)>,
            ) -> Option<Result<P::Term,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>> where
              P:  ParseCallbacks,
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Num = r#"[0-9]+"# => ActionFn(3);
                        let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<P>(callbacks, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                        0
                    }
                    2 => {
                        // Term = Num => ActionFn(1);
                        let __sym0 = __pop_NtNum(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<P>(callbacks, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        1
                    }
                    3 => {
                        // Term = "(", Term, ")" => ActionFn(2);
                        let __sym2 = __pop_Term_22_29_22(__symbols);
                        let __sym1 = __pop_NtTerm(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<P>(callbacks, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        1
                    }
                    4 => {
                        // __Term = Term => ActionFn(0);
                        let __sym0 = __pop_NtTerm(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<P>(callbacks, input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 3 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_28_22<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, &'input str, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, &'input str, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, &'input str, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtNum<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, P::Num, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTerm<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, P::Term, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Term<
              'input,
              P,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, P>,usize)>
            ) -> (usize, P::Term, usize)
             where P:  ParseCallbacks
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Term::parse_Term;
    }
}
pub use self::__parse__Term::parse_Term;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use associated_types_lib::ParseCallbacks;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+",
                "^(?u:\\()",
                "^(?u:\\))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 3 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
    P,
>(
    callbacks: &mut P,
    input: &'input str,
    (_, __0, _): (usize, P::Term, usize),
) -> P::Term where
  P:  ParseCallbacks,
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    P,
>(
    callbacks: &mut P,
    input: &'input str,
    (_, n, _): (usize, P::Num, usize),
) -> P::Term where
  P:  ParseCallbacks,
{
    n.into()
}

#[allow(unused_variables)]
fn __action2<
    'input,
    P,
>(
    callbacks: &mut P,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, P::Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> P::Term where
  P:  ParseCallbacks,
{
    t
}

#[allow(unused_variables)]
fn __action3<
    'input,
    P,
>(
    callbacks: &mut P,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> P::Num where
  P:  ParseCallbacks,
{
    callbacks.number(i32::from_str(s).unwrap())
}

pub trait __ToTriple<'input, P, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, P, > __ToTriple<'input, P, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, P, > __ToTriple<'input, P, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
