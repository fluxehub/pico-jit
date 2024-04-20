use crate::{aliases::*, compiler::Scope};
use alloc::vec::Vec;
use pico_emit::{emitter::Label, instructions::*, register_list, registers::*, Emitter};

fn get_branch_target(depth: u32, scope_stack: &[Scope]) -> &Label {
    match &scope_stack[scope_stack.len() - 1 - depth as usize] {
        Scope::Block(label) => label,
        Scope::Loop(label) => label,
        Scope::If { end_label, .. } => end_label,
    }
}

pub(crate) fn block(func: &mut Emitter, scope_stack: &mut Vec<Scope>) {
    let label = func.create_label();
    scope_stack.push(Scope::Block(label));
}

pub(crate) fn r#loop(func: &mut Emitter, scope_stack: &mut Vec<Scope>) {
    let mut label = func.create_label();
    func.label(&mut label);
    scope_stack.push(Scope::Loop(label));
}

pub(crate) fn r#if(func: &mut Emitter, scope_stack: &mut Vec<Scope>) {
    let else_label = func.create_label();
    let end_label = func.create_label();
    func.pop(register_list!(A));
    func.cmp(A, 0);
    func.branch_if(Condition::EQ, else_label);
    scope_stack.push(Scope::If {
        else_label,
        end_label,
    });
}

pub(crate) fn r#else(func: &mut Emitter, scope_stack: &mut Vec<Scope>) {
    // If the WASM is well-formed, this will always be the if block
    let Scope::If {
        mut else_label,
        end_label,
    } = scope_stack.pop().expect("Scope stack should not be empty")
    else {
        unreachable!("Else block should always be preceded by an if block");
    };

    // We're technically still at the end of the then branch, so we need to jump past this block
    func.branch(end_label);
    func.label(&mut else_label);
    scope_stack.push(Scope::Block(end_label)); // Now just becomes a normal block
}

pub(crate) fn end(func: &mut Emitter, scope_stack: &mut Vec<Scope>) {
    let scope = scope_stack.pop().expect("Scope stack should not be empty");

    match scope {
        Scope::Block(mut label) => func.label(&mut label),
        Scope::If {
            mut else_label,
            mut end_label,
        } => {
            // If we have an if case, there was never an else case, so both labels are the same
            func.label(&mut else_label);
            func.label(&mut end_label);
        }
        Scope::Loop(_) => {} // Loops don't need an end label
    };
}

pub(crate) fn br(func: &mut Emitter, scope_stack: &[Scope], depth: u32) {
    let label = get_branch_target(depth, scope_stack);
    func.branch(*label);
}

pub(crate) fn br_if(func: &mut Emitter, scope_stack: &[Scope], depth: u32) {
    let label = get_branch_target(depth, scope_stack);
    func.pop(register_list!(A));
    func.cmp(A, 0);
    func.branch_if(Condition::NE, *label);
}

pub(crate) fn br_table(func: &mut Emitter, scope_stack: &[Scope], table: &[u32], default: u32) {
    func.pop(register_list!(A));
    func.cmp(A, 0);
    for target in table {
        let label = get_branch_target(*target, scope_stack);
        func.branch_if(Condition::EQ, *label);
        func.subs(A, 1);
    }
    let label = get_branch_target(default, scope_stack);
    func.branch(*label);
}

pub(crate) fn r#return(func: &mut Emitter, scope_stack: &[Scope]) {
    let Scope::Block(end) = scope_stack.get(0).unwrap() else {
        unreachable!("Top of scope stack should always be a block");
    };

    func.branch(*end);
}

pub(crate) fn call(func: &mut Emitter, call_func: &Label, function_index: u32) {
    // Setup for call
    func.mov(r0, MODULE); // First arg is &self
    func.movs(r1, function_index as u8); // Second arg is function to call
    func.mov(r2, sp); // Third arg is the stack pointer
    func.mov(sp, ARCH_SP); // Restore sp since we're calling into native code
    func.ldr(r3, *call_func); // Get address of call into r2

    func.blx(r3); // Call the function
    func.mov(sp, r0); // Load the new stack pointer
}

pub(crate) fn select(func: &mut Emitter) {
    let mut r#else = func.create_label();
    func.pop(register_list!(A, B, C));
    func.cmp(A, 0);
    func.b_if(Condition::EQ, r#else);
    func.movs(B, C);
    func.label(&mut r#else);
    func.movs(A, B);
    func.push(register_list!(A));
}
