// Runs `tests/api.cs` in the player before its scene loads, as `<player> -- <out> [case]`:
// it prints to the file `out`, since the player's stdout is its log, and the player quits
// with the case's exit code.
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using UnityEngine;

static partial class Script
{
	[DllImport("libc.so.6", EntryPoint = "_exit")]
	static extern void Exit(int status);

	[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
	static void Boot()
	{
		// IL2CPP's `Environment.Exit` waits forever here, and nothing buffers what the test
		// printed, so a failed check ends the process at once.
		exit = code => Exit(code);
		try
		{
			var args = Environment.GetCommandLineArgs();
			var at = Array.IndexOf(args, "--");
			Check(at >= 0 && at + 1 < args.Length, "the player runs as <player> -- <out> [case]");
			// As `Main` sets up .NET's console: UTF-8 without a BOM, each line ending in LF.
			// Flushed at each write, so a check's exit keeps what came before it.
			Console.SetOut(new StreamWriter(args[at + 1], false, new UTF8Encoding(false)) { AutoFlush = true, NewLine = "\n" });
			// The test blocks on its futures as a .NET program's `Main` does, whose thread has
			// no context; Unity's would wait on this thread, which the block holds.
			SynchronizationContext.SetSynchronizationContext(null);
			Application.Quit(Run(args[(at + 2)..]));
		}
		// Unity logs what a load method throws and runs on; .NET ends a program it escapes.
		catch (Exception e)
		{
			Console.Error.WriteLine("Unhandled exception. " + e);
			Exit(1);
		}
	}
}
