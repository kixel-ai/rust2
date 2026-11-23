import ffi from "node:ffi";
const lib = new ffi.DynamicLibrary("./libthread.so");
const f = lib.getFunctions({ here: { arguments: ["pointer"], return: "int32" }, on_thread: { arguments: ["pointer"], return: "int32" } });
const cb = lib.registerCallback({ arguments: ["int32"], return: "int32" }, (x) => x * 2);
console.log("same thread:", f.here(cb));
console.log("other thread:", f.on_thread(cb));
