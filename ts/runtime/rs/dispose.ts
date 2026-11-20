// `Symbol.dispose` where the engine lacks it, as stable Safari does: the registry's symbol,
// which esbuild's `using` helper falls back to too, though TypeScript's throws without one.
// `rs.ts` imports this first, so it runs before any class below keys a method on it.
if (!Symbol.dispose) Object.defineProperty(Symbol, "dispose", { value: Symbol.for("Symbol.dispose") });
