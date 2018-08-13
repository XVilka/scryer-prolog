macro_rules! clause_name {
    ($name: expr, $tbl: expr) => (
        ClauseName::User(TabledRc::new($name, $tbl.clone()))
    ) ;
    ($name: expr) => (
        ClauseName::BuiltIn($name)
    )
}

macro_rules! tabled_rc {
    ($e:expr, $tbl:expr) => (
        TabledRc::new(String::from($e), $tbl.clone())
    )
}

macro_rules! atom {
    ($e:expr, $tbl:expr) => (
        Constant::Atom(ClauseName::User(tabled_rc!($e, $tbl)))
    );
    ($e:expr) => (
        Constant::Atom(clause_name!($e))
    )
}

macro_rules! interm {
    ($n: expr) => (
        ArithmeticTerm::Interm($n)
    )
}

macro_rules! query {
    [$($x:expr),+] => (
        Line::Query(vec![$($x),+])
    )
}

macro_rules! heap_str {
    ($s:expr) => (
        HeapCellValue::Addr(Addr::Str($s))
    )
}

macro_rules! heap_integer {
    ($i:expr) => (
        HeapCellValue::Addr(Addr::Con(integer!($i)))
    )
}

macro_rules! heap_atom {
    ($name:expr) => (
        HeapCellValue::Addr(Addr::Con(atom!($name)))
    );
    ($name:expr, $tbl:expr) => (
        HeapCellValue::Addr(Addr::Con(atom!($name, $tbl)))
    )
}

macro_rules! functor {
    ($name:expr) => (
        vec![ heap_atom!($name) ]
    );
    ($name:expr, $len:expr, [$($args:expr),*]) => (
        vec![ HeapCellValue::NamedStr($len, clause_name!($name), None), $($args),* ]
    );
    ($name:expr, $len:expr, [$($args:expr),*], $fix: expr) => (
        vec![ HeapCellValue::NamedStr($len, clause_name!($name), Some($fix)), $($args),* ]
    )
}

macro_rules! temp_v {
    ($x:expr) => (
        RegType::Temp($x)
    )
}

macro_rules! perm_v {
    ($x:expr) => (
        RegType::Perm($x)
    )
}

macro_rules! is_atom {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsAtom($r)), 1, 0)
    )
}

macro_rules! is_atomic {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsAtomic($r)), 1, 0)
    )
}

macro_rules! is_integer {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsInteger($r)), 1, 0)
    )
}

macro_rules! is_compound {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsCompound($r)), 1, 0)
    )
}

macro_rules! is_float {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsFloat($r)), 1, 0)
    )
}

macro_rules! is_rational {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsRational($r)), 1, 0)
    )
}


macro_rules! is_nonvar {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsNonVar($r)), 1, 0)
    )
}

macro_rules! is_string {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsString($r)), 1, 0)
    )
}

macro_rules! is_var {
    ($r:expr) => (
        call_clause!(ClauseType::Inlined(InlinedClauseType::IsVar($r)), 1, 0)
    )
}

macro_rules! call_clause {
    ($ct:expr, $arity:expr, $pvs:expr) => (
        Line::Control(ControlInstruction::CallClause($ct, $arity, $pvs, false, false))
    );
    ($ct:expr, $arity:expr, $pvs:expr, $lco:expr) => (
        Line::Control(ControlInstruction::CallClause($ct, $arity, $pvs, $lco, false))
    )
}

macro_rules! call_clause_by_default {
    ($ct:expr, $arity:expr, $pvs:expr) => (
        Line::Control(ControlInstruction::CallClause($ct, $arity, $pvs, false, true))
    );
    ($ct:expr, $arity:expr, $pvs:expr, $lco:expr) => (
        Line::Control(ControlInstruction::CallClause($ct, $arity, $pvs, $lco, true))
    )
}

macro_rules! proceed {
    () => (
        Line::Control(ControlInstruction::Proceed)
    )
}

macro_rules! is_call {
    ($r:expr, $at:expr) => (
        call_clause!(ClauseType::BuiltIn(BuiltInClauseType::Is($r, $at)), 2, 0)
    )
}

macro_rules! is_call_by_default {
    ($r:expr, $at:expr) => (
        call_clause_by_default!(ClauseType::BuiltIn(BuiltInClauseType::Is($r, $at)), 2, 0)
    )
}

macro_rules! set_cp {
    ($r:expr) => (
        call_clause!(ClauseType::System(SystemClauseType::SetCutPoint($r)), 1, 0)
    )
}

macro_rules! integer {
    ($i:expr) => (
        Constant::Number(Number::Integer(Rc::new(BigInt::from($i))))
    )
}

macro_rules! rc_atom {
    ($e:expr) => (
        Rc::new(String::from($e))
    )
}

macro_rules! succeed {
    () => (
        call_clause!(ClauseType::System(SystemClauseType::Succeed), 0, 0)
    )
}

macro_rules! fail {
    () => (
        call_clause!(ClauseType::System(SystemClauseType::Fail), 0, 0)
    )
}

macro_rules! compare_number_instr {
    ($cmp: expr, $at_1: expr, $at_2: expr) => {{
        let ct = ClauseType::Inlined(InlinedClauseType::CompareNumber($cmp, $at_1, $at_2));
        call_clause!(ct, 2, 0)
    }}
}

macro_rules! jmp_call {
    ($arity:expr, $offset:expr, $pvs:expr) => (
        Line::Control(ControlInstruction::JmpBy($arity, $offset, $pvs, false))
    )
}

macro_rules! try_eval_session {
    ($e:expr) => (
        match $e {
            Ok(result) => result,
            Err(e) => return EvalSession::from(e)
        }
    )
}
macro_rules! return_from_clause {
    ($lco:expr, $machine_st:expr) => {{
        if $lco {
            $machine_st.p = CodePtr::Local($machine_st.cp.clone());
        } else {
            $machine_st.p += 1;
        }

        Ok(())
    }}
}

macro_rules! dir_entry {
    ($idx:expr, $module_name:expr) => (
        CodePtr::Local(LocalCodePtr::DirEntry($idx, $module_name))
    )
}

macro_rules! set_code_index {
    ($idx:expr, $ip:expr, $mod_name:expr) => {{
        let mut idx = $idx.0.borrow_mut();

        idx.0 = $ip;
        idx.1 = $mod_name.clone();
    }}
}

macro_rules! machine_code_indices {
    ($code_dir:expr, $op_dir:expr) => ( //, $modules:expr) => (
        MachineCodeIndices { code_dir: $code_dir, op_dir: $op_dir } //, modules: $modules }
    )
}

macro_rules! put_constant {
    ($lvl:expr, $cons:expr, $r:expr) => (
        QueryInstruction::PutConstant($lvl, $cons, $r)
    )
}

macro_rules! top_level_code_ptr {
    ($p:expr, $q_sz:expr) => (
        CodePtr::Local(LocalCodePtr::TopLevel($p, $q_sz))
    )
}

macro_rules! get_level_and_unify {
    ($r: expr) => (
        Line::Cut(CutInstruction::GetLevelAndUnify($r))
    )
}
