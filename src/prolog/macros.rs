macro_rules! internal_call_n {
    () => (
        Line::BuiltIn(BuiltInInstruction::InternalCallN)
    )
}

macro_rules! allocate {
    ($cells:expr) => (
        Line::Control(ControlInstruction::Allocate($cells))
    )
}

macro_rules! deallocate {
    () => (
        Line::Control(ControlInstruction::Deallocate)
    )
}

macro_rules! query {
    [$($x:expr),+] => (
        Line::Query(vec![$($x),+])
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

macro_rules! get_var_in_query {
    ($r:expr, $arg:expr) => (
        QueryInstruction::GetVariable($r, $arg)
    )
}

macro_rules! put_var {
    ($r:expr, $arg:expr) => (
        QueryInstruction::PutVariable($r, $arg)
    )
}

macro_rules! put_value {
    ($r:expr, $arg:expr) => (
        QueryInstruction::PutValue($r, $arg)
    )
}

macro_rules! put_unsafe_value {
    ($r:expr, $arg:expr) => (
        QueryInstruction::PutUnsafeValue($r, $arg)
    )
}

macro_rules! try_me_else {
    ($o:expr) => (
        Line::Choice(ChoiceInstruction::TryMeElse($o))
    )
}

macro_rules! is_atomic {
    () => (
        Line::BuiltIn(BuiltInInstruction::IsAtomic)
    )
}

macro_rules! is_var {
    () => (
        Line::BuiltIn(BuiltInInstruction::IsVar)
    )
}

/*
macro_rules! retry_me_else {
    ($o:expr) => (
        Line::Choice(ChoiceInstruction::RetryMeElse($o))
    )
}
 */

macro_rules! trust_me {
    () => (
        Line::Choice(ChoiceInstruction::TrustMe)
    )
}

macro_rules! call_n {
    ($arity:expr) => (
        Line::Control(ControlInstruction::CallN($arity))
    )
}

macro_rules! execute_n {
    ($arity:expr) => (
        Line::Control(ControlInstruction::ExecuteN($arity))
    )
}

macro_rules! proceed {
    () => (
        Line::Control(ControlInstruction::Proceed)
    )
}

macro_rules! non_terminal {
    () => (
        Terminal::Non
    )
}

macro_rules! cut {
    ($term:expr) => (
        Line::Cut(CutInstruction::Cut($term))
    )
}

macro_rules! get_current_block {
    () => (
        Line::BuiltIn(BuiltInInstruction::GetCurrentBlock)
    )
}

macro_rules! install_new_block {
    () => (
        Line::BuiltIn(BuiltInInstruction::InstallNewBlock)
    )
}

macro_rules! goto {
    ($line:expr, $arity:expr) => (
        Line::BuiltIn(BuiltInInstruction::Goto($line, $arity))
    )
}

macro_rules! reset_block {
    () => (
        Line::BuiltIn(BuiltInInstruction::ResetBlock)
    )
}

macro_rules! get_ball {
    () => (
        Line::BuiltIn(BuiltInInstruction::GetBall)
    )
}

macro_rules! unify {
    () => (
        Line::BuiltIn(BuiltInInstruction::Unify)
    )
}

macro_rules! unwind_stack {
    () => (
        Line::BuiltIn(BuiltInInstruction::UnwindStack)
    )
}

macro_rules! clean_up_block {
    () => (
        Line::BuiltIn(BuiltInInstruction::CleanUpBlock)
    )
}

macro_rules! set_ball {
    () => (
        Line::BuiltIn(BuiltInInstruction::SetBall)
    )
}

macro_rules! fail {
    () => (
        Line::BuiltIn(BuiltInInstruction::Fail)
    )
}

macro_rules! copy_term {
    () => (
        Line::BuiltIn(BuiltInInstruction::CopyTerm)
    )
}

macro_rules! get_level {
    () => (
        Line::Cut(CutInstruction::GetLevel)
    )
}