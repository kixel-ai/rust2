import { createRequire } from "node:module";
const a = createRequire(import.meta.url)("./serve.node");
console.log("other thread, served on the caller's:", a.onThread((x) => x * 2));
