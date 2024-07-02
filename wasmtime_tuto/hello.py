from wasmtime import (
    Store,
    Module,
    Instance,
    Func,
    FuncType
)

store = Store()
module = Module.from_file(store.engine, "hello.wat")
# module is created above


def say_hello():
    "Function that prints"
    print("This is from python")


# create a variable that takes a python function
hello = Func(store, FuncType([], []), say_hello)


# instance takes the variable that takes python function
instance = Instance(store, module, [hello])
instance.exports(store)["run"](store)

watinstance = Instance(store, module, [])
wathello = watinstance.exports(store)['hello']
