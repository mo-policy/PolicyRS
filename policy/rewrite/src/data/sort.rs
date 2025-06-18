// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sort {
    Application,
    As,
    Cell,
    CellGet,
    CellSet,
    Constant,
    Dereference,
    Eval,
    Fix,
    Function,
    If,
    Let,
    List,
    Lookup,
    Loop,
    Map,
    Match,
    Policy,
    Receive,
    Send,
    Sequence,
    Set,
    Throw,
    Try,
    Tuple,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<String> for Sort {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Application" => Result::Ok(Sort::Application),
            "As" => Result::Ok(Sort::As),
            "Constant" => Result::Ok(Sort::Constant),
            "Cell" => Result::Ok(Sort::Cell),
            "CellGet" => Result::Ok(Sort::CellGet),
            "CellSet" => Result::Ok(Sort::CellSet),
            "Eval" => Result::Ok(Sort::Eval),
            "Fix" => Result::Ok(Sort::Fix),
            "Function" => Result::Ok(Sort::Function),
            "If" => Result::Ok(Sort::If),
            "Let" => Result::Ok(Sort::Let),
            "List" => Result::Ok(Sort::List),
            "Lookup" => Result::Ok(Sort::Lookup),
            "Loop" => Result::Ok(Sort::Loop),
            "Map" => Result::Ok(Sort::Map),
            "Match" => Result::Ok(Sort::Match),
            "Policy" => Result::Ok(Sort::Policy),
            "Receive" => Result::Ok(Sort::Receive),
            "Resolve" => Result::Ok(Sort::Dereference),
            "Send" => Result::Ok(Sort::Send),
            "Sequence" => Result::Ok(Sort::Sequence),
            "Set" => Result::Ok(Sort::Set),
            "Throw" => Result::Ok(Sort::Throw),
            "Try" => Result::Ok(Sort::Try),
            "Tuple" => Result::Ok(Sort::Tuple),
            _ => Result::Err(())
        }
    }
}
