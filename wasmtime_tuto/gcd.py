# import wasmtime.loader
# import gcd

# print("GCD From Wat is: ", gcd.gcd(27, 9))

# Following process works, but the above fails to find the module
from wasmtime import Store, Module, Instance

store = Store()
module = Module.from_file(store.engine, 'gcd.wat')
# module is created above
instance = Instance(store, module, [])
# above the instance executes without val in list
gcd = instance.exports(store)['gcd']
print("gcd(27, 6) = %d" % gcd(store, 27, 6))
