// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fmt::Write;
use std::rc::Rc;

use crate::machine::rewrite_machine::term_sort;
use crate::data::{DidValue, sort::Sort::*};
use crate::machine::sorts::sort_application::ApplicationValue;
use crate::machine::sorts::sort_as::AsValue;
use crate::machine::sorts::sort_cell::CellValue;
use crate::machine::sorts::sort_cell_get::CellGetValue;
use crate::machine::sorts::sort_cell_set::CellSetValue;
use crate::machine::sorts::sort_dereference::DereferenceValue;
use crate::machine::sorts::sort_eval::EvalValue;
use crate::machine::sorts::sort_function::FunctionValue;
use crate::machine::sorts::sort_if::IfValue;
use crate::machine::sorts::sort_let::LetValue;
use crate::machine::sorts::sort_list::{ListItemValue, ListValue};
use crate::machine::sorts::sort_lookup::LookupValue;
use crate::machine::sorts::sort_map::{MapItemValue, MapValue};
use crate::machine::sorts::sort_match::{MatchValue, RuleValue};
use crate::machine::sorts::sort_policy::PolicyValue;
use crate::machine::sorts::sort_receive::ReceiveValue;
use crate::machine::sorts::sort_send::SendValue;
use crate::machine::sorts::sort_sequence::SequenceValue;
use crate::machine::sorts::sort_set::SetValue;
use crate::machine::sorts::sort_try::{ThrowValue, TryValue};
use crate::machine::sorts::sort_tuple::TupleValue;

// These modes (must) match the non-terminals in grammar.lalrpop
#[derive(PartialEq, Clone, Copy)]
enum ExportMode {
    Term,
    AtomicTerm
}

fn export_sort(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    let sort = term_sort(term);
    match sort {
        Application => export_application(w, term, mode),
        As => export_as(w, term, mode),
        Cell => export_cell(w, term, mode),
        CellGet => export_cell_get(w, term, mode),
        CellSet => export_cell_set(w, term, mode),
        Constant => export_constant(w, term, mode),
        Dereference => export_dereference(w, term, mode),
        Eval => export_eval(w, term, mode),
        Fix => unreachable!("export_sort, Fix, unreachable"),
        Function => export_function(w, term, mode),
        If => export_if(w, term, mode),
        Let => export_let(w, term, mode),
        List => export_list(w, term, mode),
        Lookup => export_lookup(w, term, mode),
        Loop => todo!(),
        Map => export_map(w, term, mode),
        Match => export_match(w, term, mode),
        Policy => export_policy(w, term, mode),
        Receive => export_receive(w, term, mode),
        Send => export_send(w, term, mode),
        Sequence => export_sequence(w, term, mode),
        Set => export_set(w, term, mode),
        Throw => export_throw(w, term, mode),
        Try => export_try(w, term, mode),
        Tuple => export_tuple(w, term, mode),
    }
}

fn export_application(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    // fun arg
    let application = ApplicationValue { value: term };
    export_sort(w, &application.fun(), mode)?;
    write!(w, " ")?;
    export_sort(w, &application.arg(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, ") ")?; }
    Ok(())
}

fn export_as(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::AtomicTerm { write!(w, "( ")?; }
    // <term: AtomicTerm> "as" <name: "ident">
    let as_value = AsValue { value: term };
    export_sort(w, &as_value.term(), mode)?;
    write!(w, " as ")?;
    export_sort(w, &as_value.name(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::AtomicTerm { write!(w, ") ")?; }
    Ok(())
}

fn export_cell(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "begin ")?; }
    // "ref" <term: AtomicTerm>
    let cell_value = CellValue { value: term };
    write!(w, " ref ")?;
    export_sort(w, &cell_value.value(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_cell_get(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "begin ")?; }
    // "!" <term: AtomicTerm>
    let cell_get_value = CellGetValue { value: term };
    write!(w, " ! ")?;
    export_sort(w, &cell_get_value.cell(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_cell_set(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "begin ")?; }
    // <pattern: AtomicTerm> ":=" <term: AtomicTerm>
    let cell_set_value = CellSetValue { value: term };
    export_sort(w, &cell_set_value.cell(), ExportMode::AtomicTerm)?;
    write!(w, " := ")?;
    export_sort(w, &cell_set_value.value(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_constant(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::AtomicTerm { write!(w, "( ")?; }
    match &**term {
        DidValue::Uri(did) => {
            write!(w, "`{did:?}`")?;
        },
        v => {
            let js = serde_json::to_string_pretty(v)?;
            write!(w, "{js}")?;
        },
    }
    if mode != ExportMode::AtomicTerm { write!(w, " )")?; }
    Ok(())
}

fn export_dereference(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    let dereference_value = DereferenceValue { value: term };
    // "@" <term: AtomicTerm>
    write!(w, "@")?;
    export_sort(w, &dereference_value.term(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, " )")?; }
    Ok(())
}

fn export_function(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    let function_value = FunctionValue { value: term };
    // "fun" <pattern: AtomicTerm> "->" <term: Term>
    write!(w, "fun ")?;
    export_sort(w, &function_value.pattern(), ExportMode::AtomicTerm)?;
    write!(w, " -> ")?;
    export_sort(w, &function_value.term(), ExportMode::Term)?;
    if mode != ExportMode::Term { write!(w, " )")?; }
    Ok(())
}

fn export_eval(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    let eval_value = EvalValue { value: term };
    // "{=" <term: Term> "=}"
    write!(w, "{{= ")?;
    export_sort(w, &eval_value.term(), ExportMode::AtomicTerm)?;
    write!(w, " =}}")?;
    export_sort(w, &eval_value.term(), ExportMode::Term)?;
    Ok(())
}

fn export_if(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, " begin ")?; }
    // "if" <condition: LazyBooleanTerm> "then" <then_term: AtomicTerm> <else_term: Else?>
    let if_value = IfValue { value: term };
    write!(w, "if ")?;
    export_sort(w, &if_value.condition(), ExportMode::AtomicTerm)?;
    write!(w, " then ")?;
    export_sort(w, &if_value.then_term(), ExportMode::AtomicTerm)?;
    let else_term = if_value.else_term();
        if *else_term != DidValue::Null {
            write!(w, " else ")?;
            export_sort(w, &else_term, ExportMode::AtomicTerm)?;
        }
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_let(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, " begin ")?; }
    // "let" <pattern: AtomicTerm> "=" <term: Term> "in" <in_term: SequenceItems> "end"
    let let_value = LetValue { value: term };
    write!(w, "let ")?;
    export_sort(w, &let_value.pattern(), ExportMode::AtomicTerm)?;
    write!(w, " = ")?;
    export_sort(w, &let_value.term(), ExportMode::Term)?;
    write!(w, " in ")?;
    export_sort(w, &let_value.in_term(), ExportMode::Term)?;
    write!(w, " end ")?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_list(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    // "[" <items:SeparatedListItem*> <last:Term?> "]"
    let list_value = ListValue { value: term };
    let items = list_value.items();
    write!(w, "[ ")?;
    for item in items.try_list().ok_or("export_list, items not list")? {
        let item_value = ListItemValue { value: &item };
        export_sort(w, &item_value.term(), ExportMode::Term)?;
        let parallel = item_value.parallel().try_bool().ok_or("export_list, parallel not bool")?;
        if parallel {
            writeln!(w, "|,")?;
        } else {
            writeln!(w, ",")?;
        }
    }
    write!(w, " ]")?;
    Ok(())
}

fn export_lookup(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::AtomicTerm { write!(w, "( ")?; }
    let lookup = LookupValue { value: term };
    let name_value = lookup.name();
    let name= name_value.try_string().ok_or("export_lookup, name not string")?;
    write!(w, "{name}")?;
    if mode != ExportMode::AtomicTerm { write!(w, "( ")?; }
    Ok(())
}

fn export_map(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    // { "key": value... }
    let map_value = MapValue { value: term };
    writeln!(w, "{{")?;
    let items = map_value.items();
    for item in items.try_list().ok_or("export_map, items not list")? {
        let item_value = MapItemValue { value: &item };
        let key_value = item_value.key();
        let key = key_value.try_string().ok_or("export_map, key not string")?;
        write!(w, r#""{key}": "#)?;
        export_sort(w, &item_value.value(), ExportMode::Term)?;
        let parallel = item_value.parallel().try_bool().ok_or("export_map, parallel not bool")?;
        if parallel {
            writeln!(w, "|,")?;
        } else {
            writeln!(w, ",")?;
        }
    }
    writeln!(w, "}}")?;
    Ok(())
}

fn export_rules(w: &mut impl Write, rules: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    for rule in rules.try_list().ok_or("export_receive, rules not list")? {
        let rule_value = RuleValue { value: rule };
        writeln!(w, "")?;
        // "|" <pattern: AtomicTerm> <guard:("when" <AtomicTerm>)?> "->" <term: AtomicTerm>
        write!(w, "| ")?;
        export_sort(w, &rule_value.pattern(), ExportMode::AtomicTerm)?;
        let guard = rule_value.guard();
        if *guard != DidValue::Null {
            write!(w, " when ")?;
            export_sort(w, &guard, ExportMode::AtomicTerm)?;
        }
        write!(w, " -> ")?;
        export_sort(w, &rule_value.term(), ExportMode::AtomicTerm)?;
    }
    Ok(())
}

fn export_match(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, " begin ")?; }
    // "match" <term: Term> "with" <rules: Rules> 
    let match_value = MatchValue { value: term };
    write!(w, "match ")?;
    export_sort(w, &match_value.term(), ExportMode::Term)?;
    write!(w, " with ")?;
    export_rules(w, &match_value.rules(), mode)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_policy(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, " begin ")?; }
    // "policy" <term: Term> "with" <rules: Rules>
    let policy_value = PolicyValue { value: term };
    write!(w, "policy ")?;
    export_sort(w, &policy_value.term(), ExportMode::Term)?;
    write!(w, " with ")?;
    export_rules(w, &policy_value.rules(), mode)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}

fn export_receive(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    // receive on <channel> with <rules>
    let receive_value = ReceiveValue { value: term };
    write!(w, "receive on ")?;
    export_sort(w, &receive_value.channel(), ExportMode::Term)?;
    write!(w, " with")?;
    let rules = receive_value.rules();
    export_rules(w, &rules, mode)?;
    if mode != ExportMode::Term { write!(w, ") ")?; }
    Ok(())
}

fn export_send(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    // send <message> on <channel>
    let send_value = SendValue { value: term };
    write!(w, "send" )?;
    export_sort(w, &send_value.message(), ExportMode::Term)?;
    write!(w, " on ")?;
    export_sort(w, &send_value.channel(), ExportMode::Term)?;
    if mode != ExportMode::Term { write!(w, " )")?; }
    Ok(())
}

fn export_sequence(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    // "begin" <items: SequenceItems> "end"
    let seq_value = SequenceValue { value: term };
    let items = seq_value.items();
    write!(w, " begin ")?;
    for item in items.try_list().ok_or("export_sequence, items not list")? {
        let item_value = ListItemValue { value: &item };
        export_sort(w, &item_value.term(), ExportMode::Term)?;
        let parallel = item_value.parallel().try_bool().ok_or("export_sequence, parallel not bool")?;
        if parallel {
            writeln!(w, "|;")?;
        } else {
            writeln!(w, ";")?;
        }
    }
    write!(w, " end ")?;
    Ok(())
}

fn export_set(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    // "{[" <items:SeparatedListItem*> <last:Term?> "]}"
    let set_value = SetValue { value: term };
    let items = set_value.items();
    write!(w, "{{[")?;
    for item in items.try_list().ok_or("export_set, items not list")? {
        let item_value = ListItemValue { value: &item };
        export_sort(w, &item_value.term(), ExportMode::Term)?;
        let parallel = item_value.parallel().try_bool().ok_or("export_set, parallel not bool")?;
        if parallel {
            writeln!(w, "|,")?;
        } else {
            writeln!(w, ",")?;
        }
    }
    write!(w, "]}}")?;
    Ok(())
}

fn export_throw(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, "( ")?; }
    //"throw" <term: AtomicTerm>
    let throw_value = ThrowValue { value: term };
    write!(w, "throw ")?;
    export_sort(w, &throw_value.description(), ExportMode::AtomicTerm)?;
    if mode != ExportMode::Term { write!(w, " )")?; }
    Ok(())
}

fn export_tuple(w: &mut impl Write, term: &Rc<DidValue>, _mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    // "(" <items:SeparatedListItem*> <last:Term?> ")"
    let tuple_value = TupleValue { value: term };
    let items = tuple_value.items();
    write!(w, "( ")?;
    for item in items.try_list().ok_or("export_set, items not list")? {
        let item_value = ListItemValue { value: &item };
        export_sort(w, &item_value.term(), ExportMode::Term)?;
        let parallel = item_value.parallel().try_bool().ok_or("export_set, parallel not bool")?;
        if parallel {
            writeln!(w, "|,")?;
        } else {
            writeln!(w, ",")?;
        }
    }
    write!(w, " )")?;
    Ok(())
}

fn export_try(w: &mut impl Write, term: &Rc<DidValue>, mode: ExportMode) -> Result<(), Box<dyn std::error::Error>> {
    if mode != ExportMode::Term { write!(w, " begin ")?; }
    // "try" <term: Term> "with" <rules: Rules>
    let try_value = TryValue { value: term };
    write!(w, "try ")?;
    export_sort(w, &try_value.term(), ExportMode::Term)?;
    write!(w, " with ")?;
    export_rules(w, &try_value.rules(), mode)?;
    if mode != ExportMode::Term { write!(w, " end ")?; }
    Ok(())
}


pub fn export_term(w: &mut impl Write, term: &Rc<DidValue>) -> Result<(), Box<dyn std::error::Error>> {
    export_sort(w, term, ExportMode::Term)
}