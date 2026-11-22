// What Unity's assemblies define that the generated C# uses, so it compiles as Unity
// compiles it without them.

namespace AOT
{
	[System.AttributeUsage(System.AttributeTargets.Method)]
	public sealed class MonoPInvokeCallbackAttribute : System.Attribute
	{
		public MonoPInvokeCallbackAttribute(System.Type type) { }
	}
}
