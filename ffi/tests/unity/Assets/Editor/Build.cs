// Builds the api fixture's test as a headless Linux player with IL2CPP: the editor runs
// `Build.Player` in batch mode, `-out <path>` naming the player.
using System;
using UnityEditor;
using UnityEditor.Build;
using UnityEditor.Build.Reporting;
using UnityEditor.SceneManagement;

static class Build
{
	/// Builds the player, exiting the editor with 1 on any failure.
	static void Player()
	{
		var args = Environment.GetCommandLineArgs();
		var at = Array.IndexOf(args, "-out");
		if (at < 0 || at + 1 >= args.Length)
		{
			Fail("no -out <path>");
		}
		// The fixture's library, for Linux x64 players alone, as the binding loads it by name.
		var plugin = AssetImporter.GetAtPath("Assets/Plugins/x86_64/libapi.so") as PluginImporter;
		if (plugin == null)
		{
			Fail("no Assets/Plugins/x86_64/libapi.so");
		}
		plugin!.SetCompatibleWithAnyPlatform(false);
		plugin.SetCompatibleWithEditor(false);
		plugin.SetCompatibleWithPlatform(BuildTarget.StandaloneLinux64, true);
		plugin.SetPlatformData(BuildTarget.StandaloneLinux64, "CPU", "x86_64");
		plugin.SaveAndReimport();

		PlayerSettings.SetScriptingBackend(NamedBuildTarget.Standalone, ScriptingImplementation.IL2CPP);
		// A player needs a scene; the test runs before it loads.
		var scene = EditorSceneManager.NewScene(NewSceneSetup.EmptyScene, NewSceneMode.Single);
		if (!EditorSceneManager.SaveScene(scene, "Assets/Main.unity"))
		{
			Fail("saving the scene");
		}
		var report = BuildPipeline.BuildPlayer(new BuildPlayerOptions
		{
			scenes = new[] { "Assets/Main.unity" },
			locationPathName = args[at + 1],
			target = BuildTarget.StandaloneLinux64,
			options = BuildOptions.StrictMode,
		});
		if (report.summary.result != BuildResult.Succeeded)
		{
			Fail($"the build {report.summary.result} with {report.summary.totalErrors} errors");
		}
		EditorApplication.Exit(0);
	}

	static void Fail(string what)
	{
		Console.Error.WriteLine("Build.Player failed: " + what);
		EditorApplication.Exit(1);
	}
}
