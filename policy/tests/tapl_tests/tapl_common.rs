// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rewrite::data::DidValue;
use rewrite::machine::rewrite_machine::tapl_succ;
use rewrite::machine::sorts::*;
use sort_application::ApplicationValue;
use sort_function::FunctionValue;
use sort_if::IfValue;
use sort_lookup::LookupValue;

pub fn tapl_tru() -> Rc<DidValue> {
    // fun t -> fun f -> t
    FunctionValue::create(
        LookupValue::create("t"), 
        FunctionValue::create(
            LookupValue::create("f"), 
            LookupValue::create("t"), 
            None,
            None 
        ),
        None,
        None
    )
}

pub fn tapl_fls() -> Rc<DidValue> { 
    // fun t -> fun f -> f
    FunctionValue::create(
        LookupValue::create("t"), 
        FunctionValue::create(
            LookupValue::create("f"), 
            LookupValue::create("f"), 
            None,
            None), 
        None,
        None
    )
}

pub fn tapl_test() -> Rc<DidValue> {
    // fun l -> fun m -> fun n -> (l m) n
    FunctionValue::create(
        LookupValue::create("l"), 
        FunctionValue::create(
            LookupValue::create("m"), 
            FunctionValue::create(
                LookupValue::create("n"), 
                ApplicationValue::create(
                    ApplicationValue::create(
                        LookupValue::create("l"), 
                        LookupValue::create("m")), 
                    LookupValue::create("n")), 
                None,
                None), 
            None,
            None), 
        None,
        None
    )
}

pub fn tapl_and() -> Rc<DidValue> {
    // fun b -> fun c -> (b c) fls
    FunctionValue::create(
        LookupValue::create("b"),
        FunctionValue::create(
            LookupValue::create("c"),
            ApplicationValue::create(
                ApplicationValue::create(
                    LookupValue::create("b"),
                    LookupValue::create("c")
                ),
                tapl_fls()
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_or() -> Rc<DidValue> {
    // fun b -> fun c -> b tru c
    FunctionValue::create(
        LookupValue::create("b"),
        FunctionValue::create(
            LookupValue::create("c"),
            ApplicationValue::create(
                ApplicationValue::create(
                    LookupValue::create("b"),
                    tapl_tru()
                ),
                LookupValue::create("c")
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_not() -> Rc<DidValue> {
    // fun b -> b fls tru
    FunctionValue::create(
        LookupValue::create("b"),
        ApplicationValue::create(
            ApplicationValue::create(
                LookupValue::create("b"),
                tapl_fls()
            ),
            tapl_tru()
        ),
        None,
        None
    )
}

pub fn tapl_pair() -> Rc<DidValue> {
    // fun f -> fun s -> fun b -> b f s
    FunctionValue::create(
        LookupValue::create("f"),
        FunctionValue::create(
            LookupValue::create("s"),
            FunctionValue::create(
                LookupValue::create("b"),
                ApplicationValue::create(
                    ApplicationValue::create(
                        LookupValue::create("b"),
                        LookupValue::create("f")
                    ),
                    LookupValue::create("s")
                ),
                None,
                None
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_fst() -> Rc<DidValue> {
    // fun p -> p tru
    FunctionValue::create(
        LookupValue::create("p"),
        ApplicationValue::create(
            LookupValue::create("p"),
            tapl_tru()
        ),
        None,
        None
    )
}

pub fn tapl_snd() -> Rc<DidValue> {
    // fun p -> p fls
    FunctionValue::create(
        LookupValue::create("p"),
        ApplicationValue::create(
            LookupValue::create("p"),
            tapl_fls()
        ),
        None,
        None
    )
}

pub fn tapl_scc()-> Rc<DidValue> {
    // fun n -> fun s -> fun z -> s (n s z)
    FunctionValue::create(
        LookupValue::create("n"), 
        FunctionValue::create(
            LookupValue::create("s"), 
            FunctionValue::create(
                LookupValue::create("z"), 
                ApplicationValue::create(
                    LookupValue::create("s"), 
                    ApplicationValue::create(
                        ApplicationValue::create(
                            LookupValue::create("n"), 
                            LookupValue::create("s")
                        ), 
                        LookupValue::create("z")
                    )
                ),
                None,
                None
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_c0() -> Rc<DidValue> {
    tapl_fls()
}
pub fn tapl_c1() -> Rc<DidValue> { ApplicationValue::create(tapl_scc(), tapl_c0()) }
pub fn tapl_c2() -> Rc<DidValue> { ApplicationValue::create(tapl_scc(), tapl_c1()) }
pub fn tapl_c3() -> Rc<DidValue> { ApplicationValue::create(tapl_scc(), tapl_c2()) }
pub fn tapl_c4() -> Rc<DidValue> { ApplicationValue::create(tapl_scc(), tapl_c3()) }

pub fn tapl_plus() -> Rc<DidValue> {
    // fun m -> fun n -> fun s -> fun z -> m s (n s z)
    FunctionValue::create(
        LookupValue::create("m"), 
        FunctionValue::create(
            LookupValue::create("n"), 
            FunctionValue::create(
                LookupValue::create("s"), 
                FunctionValue::create(
                    LookupValue::create("z"), 
                    ApplicationValue::create(
                        ApplicationValue::create(
                            LookupValue::create("m"), 
                            LookupValue::create("s")
                        ), 
                        ApplicationValue::create(
                            ApplicationValue::create(
                                LookupValue::create("n"), 
                                LookupValue::create("s")
                            ), 
                            LookupValue::create("z")
                        )
                    ),
                    None,
                    None
                ),
                None,
                None
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_times() -> Rc<DidValue> {
    // fun m -> fun n -> m (plus n) c0
    FunctionValue::create(
        LookupValue::create("m"), 
        FunctionValue::create(
            LookupValue::create("n"), 
            ApplicationValue::create(
                ApplicationValue::create(
                    LookupValue::create("m"), 
                    ApplicationValue::create(
                        tapl_plus(), 
                        LookupValue::create("n")
                    )
                ), 
                tapl_c0()
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_iszro() -> Rc<DidValue> {
    // fun m -> m (fun x -> fls)) tru
    FunctionValue::create(
        LookupValue::create("m"), 
        ApplicationValue::create(
            ApplicationValue::create(
                LookupValue::create("m"), 
                FunctionValue::create(
                    LookupValue::create("x"), 
                    tapl_fls(),
                    None,
                    None
                )
            ), 
            tapl_tru()
        ),
        None,
        None
    )
}

pub fn tapl_zz() -> Rc<DidValue> {
    // pair c0 c0
    ApplicationValue::create(
        ApplicationValue::create(
            tapl_pair(), 
            tapl_c0()
        ), 
        tapl_c0()
    )
}

pub fn tapl_ss() -> Rc<DidValue> {
    // fun p -> pair (snd p) (plus c1 (snd p))
    FunctionValue::create(
        LookupValue::create("p"), 
        ApplicationValue::create(
            ApplicationValue::create(
                tapl_pair(), 
                ApplicationValue::create(
                    tapl_snd(), 
                    LookupValue::create("p")
                )
            ), 
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_plus(), 
                    ApplicationValue::create(
                        tapl_scc(), 
                        tapl_c0()
                    )
                ), 
                ApplicationValue::create(
                    tapl_snd(), 
                    LookupValue::create("p")
                )
            )
        ),
        None,
        None
    )
}

pub fn tapl_prd() -> Rc<DidValue> {
    // fun m -> fst (m ss zz)
    FunctionValue::create(
        LookupValue::create("m"), 
        ApplicationValue::create(
            tapl_fst(), 
            ApplicationValue::create(
                ApplicationValue::create(
                    LookupValue::create("m"), 
                    tapl_ss()
                ), 
                tapl_zz()
            )
        ),
        None,
        None
    )
}

pub fn tapl_subtract1() -> Rc<DidValue> {
    // fun m -> fun n -> n prd m
    FunctionValue::create(
        LookupValue::create("m"), 
        FunctionValue::create(
            LookupValue::create("n"), 
            ApplicationValue::create(
                ApplicationValue::create(
                    LookupValue::create("n"), 
                    tapl_prd()
                ), 
                LookupValue::create("m")
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_equal() -> Rc<DidValue> {
    // fun m -> fun n -> and (iszro (m prd n)) (iszro (n prd m))
    FunctionValue::create(
        LookupValue::create("m"), 
        FunctionValue::create(
            LookupValue::create("n"), 
            ApplicationValue::create(
                ApplicationValue::create(
                    tapl_and(), 
                    ApplicationValue::create(
                        tapl_iszro(), 
                        ApplicationValue::create(
                            ApplicationValue::create(
                                LookupValue::create("m"), 
                                tapl_prd()
                            ), 
                            LookupValue::create("n")
                        )
                    )
                ), 
                ApplicationValue::create(
                    tapl_iszro(), 
                    ApplicationValue::create(
                        ApplicationValue::create(
                            LookupValue::create("n"), 
                            tapl_prd()
                        ), 
                        LookupValue::create("m")
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

pub fn tapl_realbool() -> Rc<DidValue> {
    // fun b -> b true false
    FunctionValue::create(
        LookupValue::create("b"), 
        ApplicationValue::create(
            ApplicationValue::create(
                LookupValue::create("b"), 
                Rc::new(true.into())
            ), 
            Rc::new(false.into())
        ),
        None,
        None
    )
}

pub fn tapl_churchbool() -> Rc<DidValue> {
    // fun b -> if b then tru else fls
    FunctionValue::create(
        LookupValue::create("b"), 
        IfValue::create(
            LookupValue::create("b"), 
            tapl_tru(), 
            tapl_fls()
        ),
        None,
        None
    )
}

pub fn tapl_realeq() -> Rc<DidValue> {
    // fun m -> fun n -> (equal m n) true false
    FunctionValue::create(
        LookupValue::create("m"), 
        FunctionValue::create(
            LookupValue::create("n"), 
            ApplicationValue::create(
                ApplicationValue::create(
                    ApplicationValue::create(
                        ApplicationValue::create(
                            tapl_equal(), 
                            LookupValue::create("m")
                        ), 
                        LookupValue::create("n")
                    ), 
                    Rc::new(true.into())
                ), 
                Rc::new(false.into())
            ),
            None,
            None
        ),
        None,
        None
    )
}

pub fn tapl_realnat() -> Rc<DidValue> {
    // fun m -> m (succ) 0
    FunctionValue::create(
        LookupValue::create("m"), 
        ApplicationValue::create(
            ApplicationValue::create(
                LookupValue::create("m"), 
                tapl_succ()
            ), 
            Rc::new(0.into())
        ),
        None,
        None
    )
}

pub fn tapl_fix() -> Rc<DidValue> {
    // fun f -> (fun x -> f (fun y -> x x y)) (fun x -> f (fun y -> x x y))
    fn xxy() -> Rc<DidValue> {
        ApplicationValue::create(
            ApplicationValue::create(
                LookupValue::create("x"), 
                LookupValue::create("x")
            ), 
            LookupValue::create("y")
        )
    }
    fn fun_y() -> Rc<DidValue> {
        FunctionValue::create(
            LookupValue::create("y"), 
            xxy(),
            None,
            None
        )
    }
    fn fun_x() -> Rc<DidValue> {
        FunctionValue::create(
            LookupValue::create("x"), 
            ApplicationValue::create(
                LookupValue::create("f"), 
                fun_y()
            ),
            None,
            None
        )
    }
    FunctionValue::create(
        LookupValue::create("f"),
        ApplicationValue::create(
            fun_x(), 
            fun_x()
        ),
        None,
        None
    )
}

