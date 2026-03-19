#[cfg(feature = "cranelift-jit")]
use crate::vm::{VmError, VmResult};
#[cfg(feature = "cranelift-jit")]
use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
#[cfg(feature = "cranelift-jit")]
use cranelift_codegen::ir::{Block, BlockArg, InstBuilder, MemFlags, SigRef, types};
#[cfg(feature = "cranelift-jit")]
use cranelift_frontend::FunctionBuilder;

#[cfg(feature = "cranelift-jit")]
use super::{
    NativeStackLayout, OP_ADD, OP_AND, OP_CEQ, OP_CGT, OP_CLT, OP_DIV, OP_DUP, OP_LDC, OP_LDLOC,
    OP_LSHR, OP_MOD, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_POP, OP_SHL, OP_SHR, OP_STLOC, OP_SUB,
    STATUS_CONTINUE, ValueLayout, checked_add_i32, helper_entry_offset,
    string_concat_helper_entry_address,
};

#[cfg(feature = "cranelift-jit")]
#[derive(Clone, Copy)]
pub(crate) struct ResolvedOffsets {
    pub(crate) stack_ptr: i32,
    pub(crate) stack_len: i32,
    pub(crate) stack_cap: i32,
    pub(crate) locals_ptr: i32,
    pub(crate) locals_len: i32,
    pub(crate) constants_ptr: i32,
    pub(crate) constants_len: i32,
    pub(crate) vm_ip: i32,
    pub(crate) drop_contract_events_enabled: i32,
    pub(crate) drop_contract_events: i32,
}

#[cfg(feature = "cranelift-jit")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeInlineStep {
    Ldc(u32),
    Ldloc(u8),
    Stloc(u8),
    Pop,
    Dup,
    Add,
    IAdd,
    FAdd,
    SConcat,
    Sub,
    ISub,
    FSub,
    Mul,
    IMul,
    FMul,
    Div,
    IDiv,
    FDiv,
    Mod,
    IMod,
    FMod,
    Shl,
    Shr,
    Lshr,
    And,
    Or,
    Not,
    Neg,
    INeg,
    FNeg,
    Ceq,
    FCeq,
    Clt,
    FClt,
    Cgt,
    FCgt,
}

#[cfg(feature = "cranelift-jit")]
#[derive(Clone, Copy)]
pub(crate) struct InlineEmitCtx {
    pub(crate) vm_ptr: cranelift_codegen::ir::Value,
    pub(crate) helper_ref: SigRef,
    pub(crate) vm_status_helper_ref: SigRef,
    pub(crate) exit_block: Block,
    pub(crate) pointer_type: cranelift_codegen::ir::Type,
    pub(crate) layout: NativeStackLayout,
    pub(crate) offsets: ResolvedOffsets,
}

#[cfg(feature = "cranelift-jit")]
enum IntBinopKind {
    Add,
    Sub,
    Mul,
}

#[cfg(feature = "cranelift-jit")]
enum FloatBinopKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[cfg(feature = "cranelift-jit")]
#[derive(Clone, Copy)]
enum ShiftKind {
    Left,
    ArithmeticRight,
    LogicalRight,
}

#[cfg(feature = "cranelift-jit")]
pub(crate) fn resolve_offsets(layout: NativeStackLayout) -> VmResult<ResolvedOffsets> {
    let stack_ptr = checked_add_i32(
        layout.vm_stack_offset,
        layout.stack_vec.ptr_offset,
        "stack ptr offset overflow",
    )?;
    let stack_len = checked_add_i32(
        layout.vm_stack_offset,
        layout.stack_vec.len_offset,
        "stack len offset overflow",
    )?;
    let stack_cap = checked_add_i32(
        layout.vm_stack_offset,
        layout.stack_vec.cap_offset,
        "stack cap offset overflow",
    )?;
    let locals_ptr = checked_add_i32(
        layout.vm_locals_offset,
        layout.stack_vec.ptr_offset,
        "locals ptr offset overflow",
    )?;
    let locals_len = checked_add_i32(
        layout.vm_locals_offset,
        layout.stack_vec.len_offset,
        "locals len offset overflow",
    )?;

    Ok(ResolvedOffsets {
        stack_ptr,
        stack_len,
        stack_cap,
        locals_ptr,
        locals_len,
        constants_ptr: layout.vm_program_constants_ptr_offset,
        constants_len: layout.vm_program_constants_len_offset,
        vm_ip: layout.vm_ip_offset,
        drop_contract_events_enabled: layout.vm_drop_contract_events_enabled_offset,
        drop_contract_events: layout.vm_drop_contract_events_offset,
    })
}

#[cfg(feature = "cranelift-jit")]
pub(crate) fn emit_native_inline_step(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    step: NativeInlineStep,
) -> VmResult<()> {
    match step {
        NativeInlineStep::Ldc(index) => emit_inline_ldc(b, ctx, index, step_ip),
        NativeInlineStep::Ldloc(index) => emit_inline_ldloc(b, ctx, index, step_ip),
        NativeInlineStep::Stloc(index) => emit_inline_stloc(b, ctx, index, step_ip),
        NativeInlineStep::Pop => emit_inline_pop(b, ctx, step_ip),
        NativeInlineStep::Dup => emit_inline_dup(b, ctx, step_ip),
        NativeInlineStep::Add | NativeInlineStep::IAdd => {
            emit_inline_int_binop(b, ctx, step_ip, IntBinopKind::Add)
        }
        NativeInlineStep::FAdd => emit_inline_float_binop(b, ctx, step_ip, FloatBinopKind::Add),
        NativeInlineStep::SConcat => emit_inline_sconcat(b, ctx, step_ip),
        NativeInlineStep::Sub | NativeInlineStep::ISub => {
            emit_inline_int_binop(b, ctx, step_ip, IntBinopKind::Sub)
        }
        NativeInlineStep::FSub => emit_inline_float_binop(b, ctx, step_ip, FloatBinopKind::Sub),
        NativeInlineStep::Mul | NativeInlineStep::IMul => {
            emit_inline_int_binop(b, ctx, step_ip, IntBinopKind::Mul)
        }
        NativeInlineStep::FMul => emit_inline_float_binop(b, ctx, step_ip, FloatBinopKind::Mul),
        NativeInlineStep::Div | NativeInlineStep::IDiv => {
            emit_inline_int_divrem(b, ctx, step_ip, false)
        }
        NativeInlineStep::FDiv => emit_inline_float_binop(b, ctx, step_ip, FloatBinopKind::Div),
        NativeInlineStep::Mod | NativeInlineStep::IMod => {
            emit_inline_int_divrem(b, ctx, step_ip, true)
        }
        NativeInlineStep::FMod => emit_inline_float_binop(b, ctx, step_ip, FloatBinopKind::Mod),
        NativeInlineStep::Shl => emit_inline_shift(b, ctx, step_ip, ShiftKind::Left),
        NativeInlineStep::Shr => emit_inline_shift(b, ctx, step_ip, ShiftKind::ArithmeticRight),
        NativeInlineStep::Lshr => emit_inline_shift(b, ctx, step_ip, ShiftKind::LogicalRight),
        NativeInlineStep::And => emit_inline_bool_logic(b, ctx, step_ip, true),
        NativeInlineStep::Or => emit_inline_bool_logic(b, ctx, step_ip, false),
        NativeInlineStep::Not => emit_inline_not(b, ctx, step_ip),
        NativeInlineStep::Neg | NativeInlineStep::INeg => emit_inline_neg(b, ctx, step_ip),
        NativeInlineStep::FNeg => emit_inline_float_neg(b, ctx, step_ip),
        NativeInlineStep::Ceq => emit_inline_int_eq(b, ctx, step_ip),
        NativeInlineStep::FCeq => emit_inline_float_eq(b, ctx, step_ip),
        NativeInlineStep::Clt => emit_inline_int_compare(b, ctx, step_ip, true),
        NativeInlineStep::FClt => emit_inline_float_compare(b, ctx, step_ip, true),
        NativeInlineStep::Cgt => emit_inline_int_compare(b, ctx, step_ip, false),
        NativeInlineStep::FCgt => emit_inline_float_compare(b, ctx, step_ip, false),
    }
}

#[cfg(feature = "cranelift-jit")]
#[allow(clippy::too_many_arguments)]
fn emit_helper_step_from_call_tuple(
    b: &mut FunctionBuilder,
    vm_ptr: cranelift_codegen::ir::Value,
    helper_ref: SigRef,
    exit_block: Block,
    next_block: Block,
    offsets: ResolvedOffsets,
    step_ip: usize,
    tuple: (i64, i64, i64, i64),
) {
    let (op, a, b_arg, c) = tuple;
    let op_val = b.ins().iconst(types::I64, op);
    let a_val = b.ins().iconst(types::I64, a);
    let b_val = b.ins().iconst(types::I64, b_arg);
    let c_val = b.ins().iconst(types::I64, c);
    let pointer_type = b.func.signature.params[0].value_type;
    let step_ip = i64::try_from(step_ip).expect("step ip must fit i64");
    let step_ip_val = b.ins().iconst(pointer_type, step_ip);
    b.ins()
        .store(MemFlags::new(), step_ip_val, vm_ptr, offsets.vm_ip);
    let helper_ptr = b
        .ins()
        .load(pointer_type, MemFlags::new(), vm_ptr, helper_entry_offset());
    let call = b.ins().call_indirect(
        helper_ref,
        helper_ptr,
        &[vm_ptr, op_val, a_val, b_val, c_val],
    );
    let status = b.inst_results(call)[0];
    let is_continue = b
        .ins()
        .icmp_imm(IntCC::Equal, status, STATUS_CONTINUE as i64);
    let else_args = [BlockArg::Value(status)];
    b.ins()
        .brif(is_continue, next_block, &[], exit_block, &else_args);
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_ldc(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    index: u32,
    step_ip: usize,
) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let constants_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.constants_len,
    );
    let idx = b.ins().iconst(ctx.pointer_type, i64::from(index));
    let in_bounds = b.ins().icmp(IntCC::UnsignedLessThan, idx, constants_len);
    b.ins().brif(in_bounds, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let stack_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let stack_cap = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_cap,
    );
    let has_capacity = b.ins().icmp(IntCC::UnsignedLessThan, stack_len, stack_cap);
    let cap_ok = b.create_block();
    b.ins().brif(has_capacity, cap_ok, &[], slow, &[]);

    b.switch_to_block(cap_ok);
    let constants_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.constants_ptr,
    );
    let src_addr = value_addr(
        b,
        ctx.pointer_type,
        constants_ptr,
        idx,
        ctx.layout.value.size,
    );
    let src_tag = load_tag_i32(b, ctx.layout.value, src_addr);
    let scalar = is_scalar_tag(b, ctx.layout.value, src_tag);
    let scalar_ok = b.create_block();
    b.ins().brif(scalar, scalar_ok, &[], slow, &[]);

    b.switch_to_block(scalar_ok);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let dst_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        stack_len,
        ctx.layout.value.size,
    );
    copy_value_bytes(b, src_addr, dst_addr, ctx.layout.value.size);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let new_len = b.ins().iadd(stack_len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_LDC, i64::from(index), 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_ldloc(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    index: u8,
    step_ip: usize,
) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let locals_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.locals_len,
    );
    let idx = b.ins().iconst(ctx.pointer_type, i64::from(index));
    let in_bounds = b.ins().icmp(IntCC::UnsignedLessThan, idx, locals_len);
    b.ins().brif(in_bounds, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let stack_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let stack_cap = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_cap,
    );
    let has_capacity = b.ins().icmp(IntCC::UnsignedLessThan, stack_len, stack_cap);
    let cap_ok = b.create_block();
    b.ins().brif(has_capacity, cap_ok, &[], slow, &[]);

    b.switch_to_block(cap_ok);
    let locals_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.locals_ptr,
    );
    let src_addr = value_addr(b, ctx.pointer_type, locals_ptr, idx, ctx.layout.value.size);
    let src_tag = load_tag_i32(b, ctx.layout.value, src_addr);
    let scalar = is_scalar_tag(b, ctx.layout.value, src_tag);
    let scalar_ok = b.create_block();
    b.ins().brif(scalar, scalar_ok, &[], slow, &[]);

    b.switch_to_block(scalar_ok);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let dst_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        stack_len,
        ctx.layout.value.size,
    );
    copy_value_bytes(b, src_addr, dst_addr, ctx.layout.value.size);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let new_len = b.ins().iadd(stack_len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_LDLOC, i64::from(index), 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_stloc(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    index: u8,
    step_ip: usize,
) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let stack_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let has_stack = b
        .ins()
        .icmp_imm(IntCC::UnsignedGreaterThanOrEqual, stack_len, 1);
    b.ins().brif(has_stack, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let locals_len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.locals_len,
    );
    let idx = b.ins().iconst(ctx.pointer_type, i64::from(index));
    let in_bounds = b.ins().icmp(IntCC::UnsignedLessThan, idx, locals_len);
    let bounds_ok = b.create_block();
    b.ins().brif(in_bounds, bounds_ok, &[], slow, &[]);

    b.switch_to_block(bounds_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let src_index = b.ins().isub(stack_len, one);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let src_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        src_index,
        ctx.layout.value.size,
    );
    let locals_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.locals_ptr,
    );
    let dst_addr = value_addr(b, ctx.pointer_type, locals_ptr, idx, ctx.layout.value.size);
    let dst_tag = load_tag_i32(b, ctx.layout.value, dst_addr);
    let dst_scalar = is_scalar_tag(b, ctx.layout.value, dst_tag);
    let scalar_ok = b.create_block();
    b.ins().brif(dst_scalar, scalar_ok, &[], slow, &[]);

    b.switch_to_block(scalar_ok);
    let dst_is_null = b
        .ins()
        .icmp_imm(IntCC::Equal, dst_tag, i64::from(ctx.layout.value.null_tag));
    let copy_block = b.create_block();
    let maybe_count_drop_block = b.create_block();
    b.ins()
        .brif(dst_is_null, copy_block, &[], maybe_count_drop_block, &[]);

    b.switch_to_block(maybe_count_drop_block);
    let drop_events_enabled = b.ins().load(
        types::I8,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.drop_contract_events_enabled,
    );
    let should_count_drop = b.ins().icmp_imm(IntCC::NotEqual, drop_events_enabled, 0);
    let count_drop_block = b.create_block();
    b.ins()
        .brif(should_count_drop, count_drop_block, &[], copy_block, &[]);

    b.switch_to_block(count_drop_block);
    let drop_count = b.ins().load(
        types::I64,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.drop_contract_events,
    );
    let next_drop_count = b.ins().iadd_imm(drop_count, 1);
    b.ins().store(
        MemFlags::new(),
        next_drop_count,
        ctx.vm_ptr,
        ctx.offsets.drop_contract_events,
    );
    b.ins().jump(copy_block, &[]);

    b.switch_to_block(copy_block);
    copy_value_bytes(b, src_addr, dst_addr, ctx.layout.value.size);
    let new_len = b.ins().isub(stack_len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_STLOC, i64::from(index), 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_pop(b: &mut FunctionBuilder, ctx: InlineEmitCtx, step_ip: usize) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let has_stack = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 1);
    b.ins().brif(has_stack, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let top_index = b.ins().isub(len, one);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let top_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        top_index,
        ctx.layout.value.size,
    );
    let top_tag = load_tag_i32(b, ctx.layout.value, top_addr);
    let scalar = is_scalar_tag(b, ctx.layout.value, top_tag);
    let scalar_ok = b.create_block();
    b.ins().brif(scalar, scalar_ok, &[], slow, &[]);

    b.switch_to_block(scalar_ok);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_POP, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_dup(b: &mut FunctionBuilder, ctx: InlineEmitCtx, step_ip: usize) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let has_stack = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 1);
    b.ins().brif(has_stack, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let cap = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_cap,
    );
    let has_capacity = b.ins().icmp(IntCC::UnsignedLessThan, len, cap);
    let cap_ok = b.create_block();
    b.ins().brif(has_capacity, cap_ok, &[], slow, &[]);

    b.switch_to_block(cap_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let src_index = b.ins().isub(len, one);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let src_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        src_index,
        ctx.layout.value.size,
    );
    let src_tag = load_tag_i32(b, ctx.layout.value, src_addr);
    let scalar = is_scalar_tag(b, ctx.layout.value, src_tag);
    let scalar_ok = b.create_block();
    b.ins().brif(scalar, scalar_ok, &[], slow, &[]);

    b.switch_to_block(scalar_ok);
    let dst_addr = value_addr(b, ctx.pointer_type, stack_ptr, len, ctx.layout.value.size);
    copy_value_bytes(b, src_addr, dst_addr, ctx.layout.value.size);
    let new_len = b.ins().iadd(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_DUP, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_int_binop(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    kind: IntBinopKind,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.int_tag));
    let rhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.int_tag));
    let both_int = b.ins().band(lhs_int, rhs_int);
    b.ins().brif(both_int, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let lhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let out = match kind {
        IntBinopKind::Add => b.ins().iadd(lhs, rhs),
        IntBinopKind::Sub => b.ins().isub(lhs, rhs),
        IntBinopKind::Mul => b.ins().imul(lhs, rhs),
    };
    b.ins().store(
        MemFlags::new(),
        out,
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    let op = match kind {
        IntBinopKind::Add => OP_ADD,
        IntBinopKind::Sub => OP_SUB,
        IntBinopKind::Mul => OP_MUL,
    };
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (op, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_float_binop(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    kind: FloatBinopKind,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.float_tag));
    let rhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.float_tag));
    let both_float = b.ins().band(lhs_float, rhs_float);
    b.ins().brif(both_float, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let lhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let rhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let out = match kind {
        FloatBinopKind::Add => b.ins().fadd(lhs, rhs),
        FloatBinopKind::Sub => b.ins().fsub(lhs, rhs),
        FloatBinopKind::Mul => b.ins().fmul(lhs, rhs),
        FloatBinopKind::Div => b.ins().fdiv(lhs, rhs),
        FloatBinopKind::Mod => {
            let quotient = b.ins().fdiv(lhs, rhs);
            let truncated = b.ins().trunc(quotient);
            let product = b.ins().fmul(truncated, rhs);
            b.ins().fsub(lhs, product)
        }
    };
    b.ins().store(
        MemFlags::new(),
        out,
        lhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    let op = match kind {
        FloatBinopKind::Add => OP_ADD,
        FloatBinopKind::Sub => OP_SUB,
        FloatBinopKind::Mul => OP_MUL,
        FloatBinopKind::Div => OP_DIV,
        FloatBinopKind::Mod => OP_MOD,
    };
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (op, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_sconcat(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
) -> VmResult<()> {
    let next = b.create_block();
    let step_ip = i64::try_from(step_ip)
        .map_err(|_| VmError::JitNative("step ip out of range for i64".to_string()))?;
    let step_ip_val = b.ins().iconst(ctx.pointer_type, step_ip);
    b.ins()
        .store(MemFlags::new(), step_ip_val, ctx.vm_ptr, ctx.offsets.vm_ip);

    let helper_addr = i64::try_from(string_concat_helper_entry_address())
        .map_err(|_| VmError::JitNative("sconcat helper address out of range".to_string()))?;
    let helper_ptr = b.ins().iconst(ctx.pointer_type, helper_addr);
    let call = b
        .ins()
        .call_indirect(ctx.vm_status_helper_ref, helper_ptr, &[ctx.vm_ptr]);
    let status = b.inst_results(call)[0];
    let is_continue = b
        .ins()
        .icmp_imm(IntCC::Equal, status, STATUS_CONTINUE as i64);
    let else_args = [BlockArg::Value(status)];
    b.ins()
        .brif(is_continue, next, &[], ctx.exit_block, &else_args);

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_int_divrem(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    is_mod: bool,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let type_ok = b.create_block();
    let non_zero = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.int_tag));
    let rhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.int_tag));
    let both_int = b.ins().band(lhs_int, rhs_int);
    b.ins().brif(both_int, type_ok, &[], slow, &[]);

    b.switch_to_block(type_ok);
    let lhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs_not_zero = b.ins().icmp_imm(IntCC::NotEqual, rhs, 0);
    b.ins().brif(rhs_not_zero, non_zero, &[], slow, &[]);

    b.switch_to_block(non_zero);
    let min_i64 = b.ins().iconst(types::I64, i64::MIN);
    let neg_one = b.ins().iconst(types::I64, -1);
    let lhs_is_min = b.ins().icmp(IntCC::Equal, lhs, min_i64);
    let rhs_is_neg_one = b.ins().icmp(IntCC::Equal, rhs, neg_one);
    let overflow_case = b.ins().band(lhs_is_min, rhs_is_neg_one);
    let normal_block = b.create_block();
    b.ins().brif(overflow_case, slow, &[], normal_block, &[]);

    b.switch_to_block(normal_block);
    let out = if is_mod {
        b.ins().srem(lhs, rhs)
    } else {
        b.ins().sdiv(lhs, rhs)
    };
    b.ins().store(
        MemFlags::new(),
        out,
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (if is_mod { OP_MOD } else { OP_DIV }, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_shift(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    kind: ShiftKind,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let type_ok = b.create_block();
    let shift_ok = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.int_tag));
    let rhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.int_tag));
    let both_int = b.ins().band(lhs_int, rhs_int);
    b.ins().brif(both_int, type_ok, &[], slow, &[]);

    b.switch_to_block(type_ok);
    let lhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let shift_ge_zero = b.ins().icmp_imm(IntCC::SignedGreaterThanOrEqual, rhs, 0);
    let shift_le_63 = b.ins().icmp_imm(IntCC::SignedLessThanOrEqual, rhs, 63);
    let shift_in_range = b.ins().band(shift_ge_zero, shift_le_63);
    b.ins().brif(shift_in_range, shift_ok, &[], slow, &[]);

    b.switch_to_block(shift_ok);
    let out = match kind {
        ShiftKind::Left => b.ins().ishl(lhs, rhs),
        ShiftKind::ArithmeticRight => b.ins().sshr(lhs, rhs),
        ShiftKind::LogicalRight => b.ins().ushr(lhs, rhs),
    };
    b.ins().store(
        MemFlags::new(),
        out,
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (
            match kind {
                ShiftKind::Left => OP_SHL,
                ShiftKind::ArithmeticRight => OP_SHR,
                ShiftKind::LogicalRight => OP_LSHR,
            },
            0,
            0,
            0,
        ),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_bool_logic(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    is_and: bool,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let type_ok = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_bool = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.bool_tag));
    let rhs_bool = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.bool_tag));
    let both_bool = b.ins().band(lhs_bool, rhs_bool);
    b.ins().brif(both_bool, type_ok, &[], slow, &[]);

    b.switch_to_block(type_ok);
    let lhs = b.ins().load(
        types::I8,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.bool_payload_offset,
    );
    let rhs = b.ins().load(
        types::I8,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.bool_payload_offset,
    );
    let lhs_non_zero = b.ins().icmp_imm(IntCC::NotEqual, lhs, 0);
    let rhs_non_zero = b.ins().icmp_imm(IntCC::NotEqual, rhs, 0);
    let out_bool = if is_and {
        b.ins().band(lhs_non_zero, rhs_non_zero)
    } else {
        b.ins().bor(lhs_non_zero, rhs_non_zero)
    };
    store_bool_in_value(b, ctx.layout.value, lhs_addr, out_bool);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (if is_and { OP_AND } else { OP_OR }, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_not(b: &mut FunctionBuilder, ctx: InlineEmitCtx, step_ip: usize) -> VmResult<()> {
    let next = b.create_block();
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_NOT, 0, 0, 0),
    );
    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_neg(b: &mut FunctionBuilder, ctx: InlineEmitCtx, step_ip: usize) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let has_stack = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 1);
    b.ins().brif(has_stack, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let idx = b.ins().isub(len, one);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let addr = value_addr(b, ctx.pointer_type, stack_ptr, idx, ctx.layout.value.size);
    let tag = load_tag_i32(b, ctx.layout.value, addr);
    let is_int = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(ctx.layout.value.int_tag));
    let int_ok = b.create_block();
    b.ins().brif(is_int, int_ok, &[], slow, &[]);

    b.switch_to_block(int_ok);
    let value = b.ins().load(
        types::I64,
        MemFlags::new(),
        addr,
        ctx.layout.value.int_payload_offset,
    );
    let neg = b.ins().irsub_imm(value, 0);
    b.ins().store(
        MemFlags::new(),
        neg,
        addr,
        ctx.layout.value.int_payload_offset,
    );
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_NEG, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_float_neg(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
) -> VmResult<()> {
    let slow = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let has_stack = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 1);
    b.ins().brif(has_stack, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let idx = b.ins().isub(len, one);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let addr = value_addr(b, ctx.pointer_type, stack_ptr, idx, ctx.layout.value.size);
    let tag = load_tag_i32(b, ctx.layout.value, addr);
    let is_float = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(ctx.layout.value.float_tag));
    let float_ok = b.create_block();
    b.ins().brif(is_float, float_ok, &[], slow, &[]);

    b.switch_to_block(float_ok);
    let value = b.ins().load(
        types::F64,
        MemFlags::new(),
        addr,
        ctx.layout.value.float_payload_offset,
    );
    let neg = b.ins().fneg(value);
    b.ins().store(
        MemFlags::new(),
        neg,
        addr,
        ctx.layout.value.float_payload_offset,
    );
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_NEG, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_int_compare(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    less_than: bool,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.int_tag));
    let rhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.int_tag));
    let both_int = b.ins().band(lhs_int, rhs_int);
    b.ins().brif(both_int, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let lhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let cmp = if less_than {
        b.ins().icmp(IntCC::SignedLessThan, lhs, rhs)
    } else {
        b.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs)
    };
    store_bool_in_value(b, ctx.layout.value, lhs_addr, cmp);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (if less_than { OP_CLT } else { OP_CGT }, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_float_compare(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
    less_than: bool,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.float_tag));
    let rhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.float_tag));
    let both_float = b.ins().band(lhs_float, rhs_float);
    b.ins().brif(both_float, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let lhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let rhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let cmp = if less_than {
        b.ins().fcmp(FloatCC::LessThan, lhs, rhs)
    } else {
        b.ins().fcmp(FloatCC::GreaterThan, lhs, rhs)
    };
    store_bool_in_value(b, ctx.layout.value, lhs_addr, cmp);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (if less_than { OP_CLT } else { OP_CGT }, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_int_eq(b: &mut FunctionBuilder, ctx: InlineEmitCtx, step_ip: usize) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let int_fast = b.create_block();
    let bool_check = b.create_block();
    let bool_fast = b.create_block();
    let null_check = b.create_block();
    let null_fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let tags_equal = b.ins().icmp(IntCC::Equal, lhs_tag, rhs_tag);
    let tag_eq = b.create_block();
    b.ins().brif(tags_equal, tag_eq, &[], slow, &[]);

    b.switch_to_block(tag_eq);
    let lhs_int = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.int_tag));
    b.ins().brif(lhs_int, int_fast, &[], bool_check, &[]);

    b.switch_to_block(bool_check);
    let lhs_bool = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.bool_tag));
    b.ins().brif(lhs_bool, bool_fast, &[], null_check, &[]);

    b.switch_to_block(null_check);
    let lhs_null = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.null_tag));
    b.ins().brif(lhs_null, null_fast, &[], slow, &[]);

    b.switch_to_block(int_fast);
    let lhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let rhs = b.ins().load(
        types::I64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.int_payload_offset,
    );
    let cmp = b.ins().icmp(IntCC::Equal, lhs, rhs);
    store_bool_in_value(b, ctx.layout.value, lhs_addr, cmp);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(bool_fast);
    let lhs_bool_value = b.ins().load(
        types::I8,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.bool_payload_offset,
    );
    let rhs_bool_value = b.ins().load(
        types::I8,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.bool_payload_offset,
    );
    let bool_eq = b.ins().icmp(IntCC::Equal, lhs_bool_value, rhs_bool_value);
    store_bool_in_value(b, ctx.layout.value, lhs_addr, bool_eq);
    let new_len_bool = b.ins().isub(len, one);
    b.ins().store(
        MemFlags::new(),
        new_len_bool,
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    b.ins().jump(next, &[]);

    b.switch_to_block(null_fast);
    let null_eq = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.null_tag));
    store_bool_in_value(b, ctx.layout.value, lhs_addr, null_eq);
    let new_len_null = b.ins().isub(len, one);
    b.ins().store(
        MemFlags::new(),
        new_len_null,
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_CEQ, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn emit_inline_float_eq(
    b: &mut FunctionBuilder,
    ctx: InlineEmitCtx,
    step_ip: usize,
) -> VmResult<()> {
    let slow = b.create_block();
    let len_ok = b.create_block();
    let fast = b.create_block();
    let next = b.create_block();

    let len = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_len,
    );
    let enough = b.ins().icmp_imm(IntCC::UnsignedGreaterThanOrEqual, len, 2);
    b.ins().brif(enough, len_ok, &[], slow, &[]);

    b.switch_to_block(len_ok);
    let one = b.ins().iconst(ctx.pointer_type, 1);
    let two = b.ins().iconst(ctx.pointer_type, 2);
    let rhs_index = b.ins().isub(len, one);
    let lhs_index = b.ins().isub(len, two);
    let stack_ptr = b.ins().load(
        ctx.pointer_type,
        MemFlags::new(),
        ctx.vm_ptr,
        ctx.offsets.stack_ptr,
    );
    let lhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        lhs_index,
        ctx.layout.value.size,
    );
    let rhs_addr = value_addr(
        b,
        ctx.pointer_type,
        stack_ptr,
        rhs_index,
        ctx.layout.value.size,
    );
    let lhs_tag = load_tag_i32(b, ctx.layout.value, lhs_addr);
    let rhs_tag = load_tag_i32(b, ctx.layout.value, rhs_addr);
    let lhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, lhs_tag, i64::from(ctx.layout.value.float_tag));
    let rhs_float = b
        .ins()
        .icmp_imm(IntCC::Equal, rhs_tag, i64::from(ctx.layout.value.float_tag));
    let both_float = b.ins().band(lhs_float, rhs_float);
    b.ins().brif(both_float, fast, &[], slow, &[]);

    b.switch_to_block(fast);
    let lhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        lhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let rhs = b.ins().load(
        types::F64,
        MemFlags::new(),
        rhs_addr,
        ctx.layout.value.float_payload_offset,
    );
    let cmp = b.ins().fcmp(FloatCC::Equal, lhs, rhs);
    store_bool_in_value(b, ctx.layout.value, lhs_addr, cmp);
    let new_len = b.ins().isub(len, one);
    b.ins()
        .store(MemFlags::new(), new_len, ctx.vm_ptr, ctx.offsets.stack_len);
    b.ins().jump(next, &[]);

    b.switch_to_block(slow);
    emit_helper_step_from_call_tuple(
        b,
        ctx.vm_ptr,
        ctx.helper_ref,
        ctx.exit_block,
        next,
        ctx.offsets,
        step_ip,
        (OP_CEQ, 0, 0, 0),
    );

    b.switch_to_block(next);
    Ok(())
}

#[cfg(feature = "cranelift-jit")]
fn value_addr(
    b: &mut FunctionBuilder,
    pointer_type: cranelift_codegen::ir::Type,
    base_ptr: cranelift_codegen::ir::Value,
    index: cranelift_codegen::ir::Value,
    value_size: i32,
) -> cranelift_codegen::ir::Value {
    let stride = b.ins().iconst(pointer_type, i64::from(value_size));
    let offset = b.ins().imul(index, stride);
    b.ins().iadd(base_ptr, offset)
}

#[cfg(feature = "cranelift-jit")]
fn tag_type(layout: ValueLayout) -> cranelift_codegen::ir::Type {
    match layout.tag_size {
        1 => types::I8,
        2 => types::I16,
        4 => types::I32,
        _ => types::I32,
    }
}

#[cfg(feature = "cranelift-jit")]
fn load_tag_i32(
    b: &mut FunctionBuilder,
    layout: ValueLayout,
    value_addr: cranelift_codegen::ir::Value,
) -> cranelift_codegen::ir::Value {
    let raw = b.ins().load(
        tag_type(layout),
        MemFlags::new(),
        value_addr,
        layout.tag_offset,
    );
    match layout.tag_size {
        1 | 2 => b.ins().uextend(types::I32, raw),
        _ => raw,
    }
}

#[cfg(feature = "cranelift-jit")]
fn store_tag(
    b: &mut FunctionBuilder,
    layout: ValueLayout,
    value_addr: cranelift_codegen::ir::Value,
    tag: u32,
) {
    let ty = tag_type(layout);
    let raw = b.ins().iconst(ty, i64::from(tag));
    b.ins()
        .store(MemFlags::new(), raw, value_addr, layout.tag_offset);
}

#[cfg(feature = "cranelift-jit")]
fn is_scalar_tag(
    b: &mut FunctionBuilder,
    layout: ValueLayout,
    tag: cranelift_codegen::ir::Value,
) -> cranelift_codegen::ir::Value {
    let is_null = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(layout.null_tag));
    let is_int = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(layout.int_tag));
    let is_float = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(layout.float_tag));
    let is_bool = b
        .ins()
        .icmp_imm(IntCC::Equal, tag, i64::from(layout.bool_tag));
    let scalar_a = b.ins().bor(is_null, is_int);
    let scalar_b = b.ins().bor(is_float, is_bool);
    b.ins().bor(scalar_a, scalar_b)
}

#[cfg(feature = "cranelift-jit")]
fn store_bool_in_value(
    b: &mut FunctionBuilder,
    layout: ValueLayout,
    value_addr: cranelift_codegen::ir::Value,
    bool_value: cranelift_codegen::ir::Value,
) {
    store_tag(b, layout, value_addr, layout.bool_tag);
    let one = b.ins().iconst(types::I8, 1);
    let zero = b.ins().iconst(types::I8, 0);
    let byte_value = b.ins().select(bool_value, one, zero);
    b.ins().store(
        MemFlags::new(),
        byte_value,
        value_addr,
        layout.bool_payload_offset,
    );
}

#[cfg(feature = "cranelift-jit")]
fn copy_value_bytes(
    b: &mut FunctionBuilder,
    src_addr: cranelift_codegen::ir::Value,
    dst_addr: cranelift_codegen::ir::Value,
    size: i32,
) {
    let mut offset = 0i32;
    while offset + 8 <= size {
        let chunk = b.ins().load(types::I64, MemFlags::new(), src_addr, offset);
        b.ins().store(MemFlags::new(), chunk, dst_addr, offset);
        offset += 8;
    }
    if offset + 4 <= size {
        let chunk = b.ins().load(types::I32, MemFlags::new(), src_addr, offset);
        b.ins().store(MemFlags::new(), chunk, dst_addr, offset);
        offset += 4;
    }
    while offset < size {
        let chunk = b.ins().load(types::I8, MemFlags::new(), src_addr, offset);
        b.ins().store(MemFlags::new(), chunk, dst_addr, offset);
        offset += 1;
    }
}
