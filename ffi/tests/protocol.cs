// Runs scripts tests/protocol/model.rs writes through the api fixture's C#: each step on the
// main thread or a worker, writing what it returned a line to the script's path with `.out`
// added. Each script's transcript from the model must match it line for line.
//
// A call returning a `ValueTask` runs the fixture's futures, which wake themselves, to their
// end in place, so the model awaits each in the step after the one that made it: that step
// prints what the call returned.

#nullable enable

using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.ExceptionServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Api;

static class Protocol
{
	// What a refused check's message ends with, after the function's name, by the name the
	// model prints.
	static readonly (string Text, string Name)[] Statuses =
	{
		("a handle's value was moved out", "moved"),
		("a handle is borrowed exclusively", "exclusive"),
		("a handle is already borrowed", "borrowed"),
		("a handle is borrowed on another thread", "thread"),
		("a handle a call lent is borrowed exclusively or moved out", "lent"),
		("a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed", "outlived"),
		("a value that is not Send is used on another thread than the one that made it", "not_send"),
	};

	// The line for what a step threw: Rust's `Err`, a refused check, or a disposed handle,
	// which a moved-out one is too.
	static string Failure(Exception e)
	{
		switch (e)
		{
			case ObjectDisposedException:
				return "moved";
			case RustException:
				return "error";
			case InvalidOperationException:
				foreach (var (text, name) in Statuses)
				{
					if (e.Message.EndsWith(": " + text, StringComparison.Ordinal))
					{
						return name;
					}
				}
				break;
		}
		ExceptionDispatchInfo.Capture(e).Throw();
		throw e;
	}

	sealed class Script
	{
		readonly object?[] slots;
		// What each future's call returned, which the step awaiting it prints.
		readonly Dictionary<int, ulong> results = new Dictionary<int, ulong>();
		readonly string[] steps;

		public Script(string[] lines)
		{
			slots = new object?[lines[0].Split(' ').Length];
			steps = lines.Skip(1).ToArray();
		}

		T At<T>(string h) => (T)slots[int.Parse(h)]!;

		Counter? Opt(string h) => h == "-" ? null : At<Counter>(h);

		string Made(string h, object value)
		{
			slots[int.Parse(h)] = value;
			return "ok";
		}

		string Future(string h, Func<ValueTask<ulong>> call)
		{
			var task = call();
			if (!task.IsCompleted)
			{
				throw new InvalidOperationException("a fixture's future pended past its call");
			}
			results[int.Parse(h)] = task.Result;
			slots[int.Parse(h)] = h;
			return "ok";
		}

		static async ValueTask<ulong> Widen(ValueTask<uint> task) => await task;

		// Runs one step on this thread: what it returned.
		string Run(string op, string[] a)
		{
			try
			{
				switch (op)
				{
					case "counter_new": return Made(a[0], Counter.New("c", ulong.Parse(a[1])));
					case "counter_get": return At<Counter>(a[0]).Get().ToString();
					case "counter_add": return At<Counter>(a[0]).Add(ulong.Parse(a[1])).ToString();
					case "counter_into_sum": return At<Counter>(a[0]).IntoSum(ulong.Parse(a[1])).ToString();
					case "counter_plus": return At<Counter>(a[0]).Plus(Opt(a[1])).ToString();
					case "counter_add_to": return At<Counter>(a[0]).AddTo(Opt(a[1])) ? "1" : "0";
					case "counter_clone": return Made(a[0], At<Counter>(a[1]).Clone());
					case "plus_later":
					{
						var (counter, other) = (At<Counter>(a[1]), Opt(a[2]));
						return Future(a[0], () => counter.PlusLater(other));
					}
					case "ledger_new": return Made(a[0], Ledger.New());
					case "ledger_edit":
						Made(a[0], At<Ledger>(a[1]).Edit());
						return "1";
					case "ledger_published": return At<Ledger>(a[0]).Published().ToString();
					case "ledger_seal":
						At<Ledger>(a[0]).Seal();
						return "ok";
					case "ledger_receipt": return Made(a[0], At<Ledger>(a[1]).Receipt());
					case "draft_write":
						At<Draft>(a[0]).Write("line");
						return "ok";
					case "draft_lines": return At<Draft>(a[0]).Lines().ToString();
					case "draft_commit": return At<Draft>(a[0]).Commit().ToString();
					case "tallied": return Lib.Tallied(At<Receipt>(a[0])).ToString();
					case "stored": return Lib.Stored(At<Receipt>(a[0])).ToString();
					case "receipt_count": return At<Receipt>(a[0]).Count().ToString();
					case "receipt_clone": return Made(a[0], At<Receipt>(a[1]).Clone());
					case "tally_new": return Made(a[0], Tally.New());
					case "tally_bump": return At<Tally>(a[0]).Bump().ToString();
					case "bump_later":
					{
						var tally = At<Tally>(a[1]);
						return Future(a[0], () => Widen(tally.BumpLater()));
					}
					case "local_new":
						if (a[1] != "0")
						{
							throw new NotSupportedException("C# makes no local on a home");
						}
						return Made(a[0], Local.New());
					case "local_bump": return At<Local>(a[0]).Bump().ToString();
					case "local_share": return Made(a[0], At<Local>(a[1]).Share());
					case "local_later":
					{
						var local = At<Local>(a[1]);
						return Future(a[0], () => Widen(local.Later()));
					}
					case "meters_new": return Made(a[0], Meters.New());
					case "meters_first": return Made(a[0], At<Meters>(a[1]).First());
					case "meters_clear":
						At<Meters>(a[0]).Clear();
						return "ok";
					case "meter_new": return Made(a[0], Meter.Default());
					case "meter_add": return At<Meter>(a[0]).Add(ulong.Parse(a[1])).ToString();
					case "meter_reset":
						At<Meter>(a[0]).Reset();
						return "ok";
					case "drop":
						At<IDisposable>(a[0]).Dispose();
						return "ok";
					case "await": return results[int.Parse(a[0])].ToString();
					default: throw new NotSupportedException("an op C# does not run: " + op);
				}
			}
			catch (Exception e)
			{
				return Failure(e);
			}
		}

		public string Run()
		{
			var out_ = new StringBuilder();
			// The worker: a thread of its own per script, which exits once it ends. A background
			// thread, so a script that throws ends the run rather than leaving it waiting.
			using var jobs = new BlockingCollection<Action>();
			var worker = new Thread(() =>
			{
				foreach (var job in jobs.GetConsumingEnumerable())
				{
					job();
				}
			})
			{
				IsBackground = true,
			};
			worker.Start();
			try
			{
				foreach (var line in steps)
				{
					var words = line.Split(' ');
					var (op, a) = (words[1], words.Skip(2).ToArray());
					string got;
					if (words[0] == "0")
					{
						got = Run(op, a);
					}
					else
					{
						using var done = new ManualResetEventSlim();
						string? result = null;
						ExceptionDispatchInfo? thrown = null;
						jobs.Add(() =>
						{
							try
							{
								result = Run(op, a);
							}
							catch (Exception e)
							{
								thrown = ExceptionDispatchInfo.Capture(e);
							}
							done.Set();
						});
						done.Wait();
						thrown?.Throw();
						got = result!;
					}
					out_.Append(got).Append('\n');
				}
			}
			finally
			{
				// Joined on failure too, since `jobs` is disposed next; a worker still in it
				// would throw over the step's own exception.
				jobs.CompleteAdding();
				if (!worker.Join(TimeSpan.FromMinutes(1)))
				{
					Environment.FailFast("the worker never exited");
				}
			}
			return out_.ToString();
		}
	}

	static void Main(string[] args)
	{
		foreach (var path in args)
		{
			File.WriteAllText(path + ".out", new Script(File.ReadAllLines(path)).Run());
		}
	}
}
