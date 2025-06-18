// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rewrite::data::DidValue;
use rewrite::machine::rewrite_machine::{run_rewrite, RewriteOk};
use rewrite::machine::sorts::*;
use sort_application::ApplicationValue;
use sort_function::FunctionValue;
use sort_if::IfValue;
use sort_lookup::LookupValue;
use crate::tapl_common::*;

#[test]
fn test_tapl_tru() {
    // tru "a" "b"
    let a: Rc<DidValue> = Rc::new("a".into());
    let b: Rc<DidValue> = Rc::new("b".into());
    let tru = tapl_tru();
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tru,
            a.clone()),
        b
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(a));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_fls() {
    // fls "a" "b"
    let a: Rc<DidValue> = Rc::new("a".into());
    let b: Rc<DidValue> = Rc::new("b".into());
    let fls = tapl_fls();
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            fls,
            a),
        b.clone()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(b));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_test() {
    let v: Rc<DidValue> = Rc::new("v".into());
    let w: Rc<DidValue> = Rc::new("w".into());
    let b = tapl_tru();
    let test = tapl_test();
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                ApplicationValue::create(
                    test,
                    b),
                v.clone()
                ),
            w
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(v));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_and_tru_tru() {
    // and tru tru
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_and(),
                tapl_tru()
                ),
            tapl_tru()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_and_tru_fls() {
    // and tru fls
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_and(),
            tapl_tru()
            ),
        tapl_fls()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_and_fls_tru() {
    // and fls tru
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_and(),
            tapl_fls()
            ),
        tapl_tru()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_and_fls_fls() {
    // and fls fls
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_and(),
            tapl_fls()
            ),
        tapl_fls()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_or_tru_fls() {
    // or tru fls
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_or(),
            tapl_tru()
            ),
        tapl_fls()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_or_tru_tru() {
    // or tru tru
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_or(),
            tapl_tru()
            ),
        tapl_tru()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_or_fls_tru() {
    // or fls tru
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_or(),
            tapl_fls()
        ),
    tapl_tru()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_or_fls_fls() {
    // or fls fls
    let program = 
        ApplicationValue::create(
        ApplicationValue::create(
            tapl_or(),
            tapl_fls()
            ),
        tapl_fls()
    );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_not_tru() {
    // not tru
    let program = 
        ApplicationValue::create(
        tapl_not(),
        tapl_tru()
    );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_not_fls() {
    // not fls
    let program = 
        ApplicationValue::create(
        tapl_not(),
        tapl_fls()
    );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_fst() {
    // fst (pair v w)
    let v:Rc<DidValue> = Rc::new("v".into());
    let w:Rc<DidValue> = Rc::new("w".into());
    let program = 
        ApplicationValue::create(
            tapl_fst(),
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_pair(),
                    v.clone()
                ),
                w
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(v));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_snd() {
    // snd (pair v w)
    let v:Rc<DidValue> = Rc::new("v".into());
    let w:Rc<DidValue> = Rc::new("w".into());
    let program = 
        ApplicationValue::create(
            tapl_snd(),
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_pair(),
                    v
                ),
                w.clone()
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(w));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_0() {
    // iszro c0
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            tapl_c0()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_1() {
    // iszro (scc c0)
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            ApplicationValue::create(
                tapl_scc(), 
                tapl_c0()
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_prd_1() {
    // iszro (prd (scc c0))
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            ApplicationValue::create(
                tapl_prd(), 
                ApplicationValue::create(
                    tapl_scc(), 
                    tapl_c0()
                )
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_subtract_00() {
    // iszro ((subtract1 c0) c0)
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_subtract1(),
                    tapl_c0()
                ), 
                tapl_c0()
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_subtract_10() {
    // iszro ((subtract1 (scc c0)) c0)
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_subtract1(),
                    ApplicationValue::create(
                        tapl_scc(), 
                        tapl_c0()
                    )
                ), 
                tapl_c0()
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_iszro_subtract_321() {
    // 0 = 3 - 2 - 1
    // iszro (subtract1 (((subtract1 3) 2) 1))
    let program = 
        ApplicationValue::create(
            tapl_iszro(),
            ApplicationValue::create(
                tapl_subtract1(),
                ApplicationValue::create(
                    ApplicationValue::create(
                        ApplicationValue::create(
                            tapl_subtract1(), 
                            tapl_c3()
                        ), 
                        tapl_c2()
                    ), 
                    tapl_c1()
                )
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_equal_00() {
    // equal 0 0
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_equal(), 
                tapl_c0()
            ),
            tapl_c0()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_equal_01() {
    // equal 0 1
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_equal(), 
                tapl_c0()
            ),
            tapl_c1()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_equal_32() {
    // equal 3 2
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_equal(), 
                tapl_c3()
            ),
            tapl_c2()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_equal_33() {
    // equal 3 3
    let program = 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_equal(), 
                tapl_c3()
            ),
            tapl_c3()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_realbool_tru() {
    // realbool tru
    let program = 
        ApplicationValue::create(
            tapl_realbool(),
            tapl_tru()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(Rc::new(true.into())));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_realbool_fls() {
    // realbool fls
    let program = 
        ApplicationValue::create(
            tapl_realbool(),
            tapl_fls()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(Rc::new(false.into())));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_churchbool_true() {
    // churchbool true
    let program = 
        ApplicationValue::create(
            tapl_churchbool(),
            Rc::new(true.into())
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_tru()));
    assert_eq!(expected, result);
}

    #[test]
fn test_tapl_churchbool_false() {
    // churchbool false
    let program = 
        ApplicationValue::create(
            tapl_churchbool(),
            Rc::new(false.into())
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(tapl_fls()));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_realnat_0() {
    // realnat c0
    let program = 
        ApplicationValue::create(
            tapl_realnat(),
            tapl_c0()
        );
    let result: Result<RewriteOk, rewrite::machine::rewrite_machine::RewriteErr> = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(Rc::new(0.into())));
    assert_eq!(expected, result);
}


#[test]
fn test_tapl_realnat_2() {
    // realnat c2
    let program = 
    ApplicationValue::create(
            tapl_realnat(),
            tapl_c2()
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_realnat_times_2_2() {
    // realnat ((times 2) 2)
    let program = 
        ApplicationValue::create(
            tapl_realnat(),
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_times(), 
                    tapl_c2()
                ), 
                tapl_c2()
            )
        );
    let result = run_rewrite(program);
    let expected = Result::Ok(RewriteOk::Term(Rc::new(4.into())));
    assert_eq!(expected, result);
}

#[test]
fn test_tapl_factorial() {
    // fun fct -> fun n -> if realeq n c0 then c1 else ((times n) (fct (prd n)))
    fn g() -> Rc<DidValue> {
        FunctionValue::create(
            LookupValue::create("fct"), 
            FunctionValue::create(
                LookupValue::create("n"), 
                IfValue::create(
                    ApplicationValue::create(
                        ApplicationValue::create(
                            tapl_realeq(), 
                            LookupValue::create("n")
                        ), 
                        tapl_c0()
                    ), 
                    tapl_c1(), 
                    ApplicationValue::create(
                        ApplicationValue::create(
                            tapl_times(), 
                            LookupValue::create("n")
                        ), 
                        ApplicationValue::create(
                            LookupValue::create("fct"), 
                            ApplicationValue::create(
                                tapl_prd(), 
                                LookupValue::create("n")
                            )
                        )
                    )
                ),
                None,
                None
            ),
            None,
            None
        )
    }
    fn factorial() -> Rc<DidValue> {
        // fix g
        ApplicationValue::create(
            tapl_fix(), 
            g()
        )
    }
    let program_factorial_0 = 
        ApplicationValue::create(
            tapl_realnat(),
            ApplicationValue::create(
                factorial(), 
                tapl_c0()
            )
        );
    let result_factorial_0 = run_rewrite(program_factorial_0);
    let expected_factorial_0 = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
    assert_eq!(expected_factorial_0, result_factorial_0);

    let program_factorial_1 = 
        ApplicationValue::create(
            tapl_realnat(), 
            ApplicationValue::create(
                factorial(), 
                tapl_c1()
            )
        );
    let result_factorial_1 = run_rewrite(program_factorial_1);
    let expected_factorial_1 = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
    assert_eq!(expected_factorial_1, result_factorial_1);

    let program_factorial_2 = 
        ApplicationValue::create(
            tapl_realnat(), 
            ApplicationValue::create(
                factorial(), 
                tapl_c2()
            )
        );
    let result_factorial_2 = run_rewrite(program_factorial_2);
    let expected_factorial_2 = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
    assert_eq!(expected_factorial_2, result_factorial_2);

    let program_factorial_3 = 
        ApplicationValue::create(
            tapl_realnat(), 
            ApplicationValue::create(
                factorial(), 
                tapl_c3()
            )
        );
    let result_factorial_3 = run_rewrite(program_factorial_3);
    let expected_factorial_3 = Result::Ok(RewriteOk::Term(Rc::new(6.into())));
    assert_eq!(expected_factorial_3, result_factorial_3);

    let program_factorial_4 = 
        ApplicationValue::create(
            tapl_realnat(), 
            ApplicationValue::create(
                factorial(), 
                tapl_c4()
            )
        );
    let result_factorial_4 = run_rewrite(program_factorial_4);
    let expected_factorial_4 = Result::Ok(RewriteOk::Term(Rc::new(24.into())));
    assert_eq!(expected_factorial_4, result_factorial_4);
}
