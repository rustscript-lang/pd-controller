mod bridge;
mod codegen;
mod exec;
mod inline;
mod layout;

pub(crate) use bridge::{
    NativeInterruptMode, NativeInterruptSettings, OP_ADD, OP_AND, OP_BUILTIN_CALL, OP_CALL, OP_CEQ,
    OP_CGT, OP_CLT, OP_DIV, OP_DUP, OP_GUARD_FALSE, OP_GUARD_TRUE, OP_JUMP, OP_LDC, OP_LDLOC,
    OP_LOOP_IF_FALSE, OP_LSHR, OP_MOD, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_POP, OP_SHL, OP_SHR,
    OP_STLOC, OP_SUB, OP_TRACE_BYTES_FROM_ARRAY_U8, OP_TRACE_BYTES_TO_ARRAY_U8,
    OP_TRACE_CONCAT_BYTES, OP_TRACE_CONCAT_STRING, OP_TRACE_GET_BYTES, OP_TRACE_GET_STRING,
    OP_TRACE_HAS_BYTES, OP_TRACE_LEN_BYTES, OP_TRACE_LEN_STRING, OP_TRACE_SLICE_BYTES,
    OP_TRACE_SLICE_STRING, STATUS_CONTINUE, STATUS_ERROR, STATUS_HALTED, STATUS_OUT_OF_FUEL,
    STATUS_TRACE_EXIT, STATUS_WAITING, STATUS_YIELDED, clear_bridge_error, helper_entry_address,
    helper_entry_offset, interrupt_helper_entry_address, interrupt_helper_entry_offset,
    string_concat_helper_entry_address, take_bridge_error, typed_step_helper_entry_address,
};
#[cfg(feature = "cranelift-jit")]
pub(crate) use codegen::{
    entry_signature, helper_signature, jump_with_status, typed_step_signature,
};
pub(crate) use exec::ExecutableBuffer;
#[cfg(feature = "cranelift-jit")]
pub(crate) use inline::{
    InlineEmitCtx, NativeInlineStep, ResolvedOffsets, emit_native_inline_step, resolve_offsets,
};
pub(crate) use layout::{
    NativeStackLayout, ValueLayout, checked_add_i32, detect_native_stack_layout,
};

#[cfg(feature = "cranelift-jit")]
pub(crate) fn selected_codegen_backend() -> &'static str {
    "native"
}

#[cfg(not(feature = "cranelift-jit"))]
pub(crate) fn selected_codegen_backend() -> &'static str {
    "native-disabled"
}
