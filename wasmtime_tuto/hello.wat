(module
    (func $hello (import "" "hello from wat file..."))
    (func (export "run") (call $hello))
) 