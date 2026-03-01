mod builtins;

pub mod assembler;
pub mod bytecode;
pub mod compiler;
pub mod debug_info;
#[cfg(feature = "runtime")]
pub mod debugger;
#[cfg(feature = "runtime")]
pub mod jit;
#[cfg(feature = "runtime")]
pub mod vm;
#[cfg(feature = "runtime")]
pub mod vmbc;

pub use assembler::{AsmParseError, Assembler, AssemblerError, BytecodeBuilder, assemble};
pub use bytecode::{HostImport, OpCode, Program, Value};
pub use compiler::{
    CompileError, CompiledProgram, Compiler, Expr, FunctionDecl, ParseError, SourceError,
    SourceFlavor, SourcePathError, Stmt, compile_source, compile_source_file,
    compile_source_with_flavor,
};
pub use debug_info::{ArgInfo, DebugFunction, DebugInfo, LineInfo, LocalInfo};
#[cfg(feature = "runtime")]
pub use debugger::{
    DebugCommandBridge, DebugCommandBridgeError, DebugCommandBridgeResponse,
    DebugCommandBridgeStatus, Debugger, StepMode, VmRecording, VmRecordingError, VmRecordingFrame,
    VmRecordingReplayResponse, VmRecordingReplayState, replay_recording_stdio,
    run_recording_replay_command,
};
#[cfg(feature = "runtime")]
pub use jit::{
    JitAttempt, JitConfig, JitNyiDoc, JitNyiReason, JitSnapshot, JitTrace, JitTraceTerminal,
    TraceJitEngine,
};
#[cfg(feature = "runtime")]
pub use vm::{
    CallOutcome, HostBindingPlan, HostFunction, HostFunctionRegistry, StaticHostFunction, Vm,
    VmError, VmResult, VmStatus,
};
#[cfg(feature = "runtime")]
pub use vmbc::{
    DisassembleOptions, ValidationError, WireError, decode_program, disassemble_program,
    disassemble_program_with_options, disassemble_vmbc, disassemble_vmbc_with_options,
    encode_program, infer_local_count, validate_program,
};
