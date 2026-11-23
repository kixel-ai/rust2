// What Unity's assemblies define that the generated C# uses, so it compiles as Unity
// compiles it without them.

namespace AOT
{
	/// <summary>Marks a method Rust calls through a delegate's address, as IL2CPP needs.</summary>
	[System.AttributeUsage(System.AttributeTargets.Method)]
	public sealed class MonoPInvokeCallbackAttribute : System.Attribute
	{
		/// <summary>For a method of the delegate type <paramref name="type"/>.</summary>
		public MonoPInvokeCallbackAttribute(System.Type type) { }
	}
}
