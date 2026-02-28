#![cfg(feature = "runtime")]
mod common;
use common::*;

#[test]
fn lua_vm_require_namespace_host_calls_are_supported() {
    let source = r#"
        local vm = require("vm")
        vm.add_one(41)
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    vm.bind_function("add_one", Box::new(AddOne));

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_vm_http_subnamespace_host_calls_are_supported() {
    let source = r#"
        local vm = require("vm")
        vm.http.request.get_header("x-client-id")
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    vm.bind_function("http::request::get_header", Box::new(EchoString));

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::String("x-client-id".to_string())]);
}

#[test]
fn lua_vm_require_member_alias_host_calls_are_supported() {
    let source = r#"
        local inc = require("vm").add_one
        inc(41)
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    vm.bind_function("add_one", Box::new(AddOne));

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn compile_source_with_lua_flavor() {
    let source = include_str!("../examples/example.lua");

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);

    for func in &compiled.functions {
        match func.name.as_str() {
            "add_one" => vm.register_function(Box::new(AddOne)),
            "print" => vm.register_function(Box::new(PrintBuiltin)),
            _ => panic!("unexpected function {}", func.name),
        };
    }

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(6)]);
}

#[test]
fn lua_assignment_updates_existing_local_without_new_slot() {
    let source = r#"
        local a = 1
        a = 2
        a
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    assert_eq!(compiled.locals, 1);

    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(2)]);
}

#[test]
fn lua_print_works_without_decl() {
    let source = r#"
        print(40 + 2)
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);

    for func in &compiled.functions {
        match func.name.as_str() {
            "print" => vm.register_function(Box::new(PrintBuiltin)),
            _ => panic!("unexpected function {}", func.name),
        };
    }

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn compile_source_file_with_lua_complex_fixture() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/example_complex.lua");
    let compiled = compile_source_file(&path).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);

    for func in &compiled.functions {
        match func.name.as_str() {
            "print" => vm.register_function(Box::new(PrintBuiltin)),
            "add_one" => vm.register_function(Box::new(AddOne)),
            _ => panic!("unexpected function {}", func.name),
        };
    }

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(12)]);
}

#[test]
fn lua_function_literal_without_return_is_rejected() {
    let source = r#"
        local base = 7
        local add = function(value) value + base end
        print(add(5))
    "#;
    let err = match compile_source_with_flavor(source, SourceFlavor::Lua) {
        Ok(_) => panic!("lua closure without return should fail"),
        Err(err) => err,
    };

    match err {
        vm::SourceError::Parse(err) => assert!(err.message.contains("return <expr>")),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn lua_require_namespace_enables_direct_host_calls() {
    let source = r#"
        local vm = require("vm")
        print(add_one(41))
    "#;
    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);

    for func in &compiled.functions {
        match func.name.as_str() {
            "add_one" => vm.register_function(Box::new(AddOne)),
            "print" => vm.register_function(Box::new(PrintBuiltin)),
            _ => panic!("unexpected function {}", func.name),
        };
    }

    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn compile_source_file_lua_supports_namespace_and_named_require_imports() {
    let unique = format!(
        "vm_lua_namespace_import_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos()
    );
    let root = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&root).expect("temp module root should be created");

    let module_path = root.join("strings.rss");
    std::fs::write(
        &module_path,
        r#"
        fn eq(lhs, rhs) {
            lhs == rhs;
        }
        pub fn is_empty(value) {
            eq(value, "");
        }
        pub fn non_empty(value) {
            eq(is_empty(value), false);
        }
    "#,
    )
    .expect("module source should write");

    let main_path = root.join("main.lua");
    std::fs::write(
        &main_path,
        r#"
        local string = require("./strings.rss")
        local is_empty = require("./strings.rss").is_empty
        print(string.non_empty("rss"))
        print(is_empty(""))
    "#,
    )
    .expect("lua source should write");

    let compiled = compile_source_file(&main_path).expect("compile should succeed");
    assert_eq!(compiled.functions.len(), 1);
    assert_eq!(compiled.functions[0].name, "print");

    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    vm.bind_function("print", Box::new(PrintBuiltin));
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Bool(true), Value::Bool(true)]);

    let _ = std::fs::remove_file(main_path);
    let _ = std::fs::remove_file(module_path);
    let _ = std::fs::remove_dir(root);
}

#[test]
fn lua_undeclared_host_call_is_rejected() {
    let source = r#"
        add_one(41)
    "#;
    let err = match compile_source_with_flavor(source, SourceFlavor::Lua) {
        Ok(_) => panic!("undeclared host call should fail"),
        Err(err) => err,
    };
    match err {
        vm::SourceError::Parse(parse) => {
            assert!(parse.message.contains("unknown function 'add_one'"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn lua_modulo_operator_works() {
    let source = r#"
        local a = 17 % 5
        local f = 100 % 7
        a + f
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(4)]);
}

#[test]
fn lua_logical_operators_work() {
    let source = r#"
        local b = true and false
        local c = true or false
        local d = (10 > 5) and (3 < 7)
        local e = (10 < 5) or (3 > 7)
        if d and c then
            42
        else
            0
        end
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_local_function_declaration_is_supported() {
    let source = r#"
        local function inc(v)
            return v + 1
        end
        inc(41)
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_repeat_until_is_supported() {
    let source = r#"
        local i = 0
        repeat
            i = i + 1
        until i > 2
        i
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(3)]);
}

#[test]
fn lua_do_end_block_is_supported() {
    let source = r#"
        local value = 1
        do
            value = value + 41
        end
        value
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_nil_and_concat_and_single_quoted_strings_are_supported() {
    let source = r#"
        local s = 'a' .. 'b'
        if type_of(nil) == "null" and s == "ab" then
            42
        else
            0
        end
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_string_method_calls_and_length_operator_are_lowered() {
    let source = r#"
        local s = "hello123world"
        local arr = { 1, 2, 3 }
        local left = s:sub(1, 5)
        local right = s:sub(-5, -1)
        local count = #s
        local arr_count = #arr
        if left == "hello" and right == "world" and count == 13 and arr_count == 3 then
            42
        else
            0
        end
    "#;

    let compiled =
        compile_source_with_flavor(source, SourceFlavor::Lua).expect("compile should succeed");
    let mut vm = Vm::with_locals(compiled.program, compiled.locals);
    let status = vm.run().expect("vm should run");
    assert_eq!(status, VmStatus::Halted);
    assert_eq!(vm.stack(), &[Value::Int(42)]);
}

#[test]
fn lua_string_pattern_methods_are_rejected() {
    let source = r##"
        local s = "v1 id=2048 done"
        s:find("%d+")
    "##;

    let err = match compile_source_with_flavor(source, SourceFlavor::Lua) {
        Ok(_) => panic!("lua pattern methods should fail in this subset"),
        Err(err) => err,
    };
    match err {
        vm::SourceError::Parse(parse) => {
            assert!(parse.message.contains("Lua pattern API"));
        }
        other => panic!("unexpected error: {other}"),
    }
}
