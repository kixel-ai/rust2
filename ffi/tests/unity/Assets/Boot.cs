// Runs `tests/api.cs`'s `Main` in the player before its scene loads, with the arguments after
// `--` on the command line, and quits with its exit code.
using System;
using System.Threading;
using UnityEngine;

static partial class Script
{
	[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
	static void Boot()
	{
		var args = Environment.GetCommandLineArgs();
		var at = Array.IndexOf(args, "--");
		var rest = at < 0 ? Array.Empty<string>() : args[(at + 1)..];
		// The test blocks on its futures as a .NET program's `Main` does, whose thread has no
		// context; Unity's would wait on this thread, which the block holds.
		SynchronizationContext.SetSynchronizationContext(null);
		Application.Quit(Main(rest));
	}
}
