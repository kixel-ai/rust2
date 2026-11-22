// The runtime every generated C# file includes, inside its namespace, beside the `Native`
// half the generator writes: the C structs every export shares, the handles objects own,
// text, bytes, and Rust's other std values as C# holds them exactly, the callbacks Rust
// calls into C#, and the source a Rust future's `ValueTask` completes from.

/// The C ABI's shared structs and the runtime's own helpers, beside the export's functions.
internal static unsafe partial class Native
{
	/// `<prefix>_buf`: bytes Rust owns, or a view of the caller's.
	[StructLayout(LayoutKind.Sequential)]
	internal struct Buf
	{
		public byte* ptr;
		public nuint len;
		public void* owner;
	}

	/// `<prefix>_waker`.
	[StructLayout(LayoutKind.Sequential)]
	internal struct CWaker
	{
		public IntPtr wake;
		public IntPtr drop;
		public void* data;
	}

	/// `<prefix>_foreign_future`: a future C# implements.
	[StructLayout(LayoutKind.Sequential)]
	internal struct CForeignFuture
	{
		public IntPtr poll;
		public IntPtr free;
		public void* data;
	}

	/// `<prefix>_foreign_reader`: a stream C# lends Rust for a call.
	[StructLayout(LayoutKind.Sequential)]
	internal struct CForeignReader
	{
		public IntPtr read;
		public IntPtr seek;
		public void* data;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CU128
	{
		public ulong lo;
		public ulong hi;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CDuration
	{
		public ulong secs;
		public uint nanos;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CIpv4Addr
	{
		public fixed byte octets[4];
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CIpv6Addr
	{
		public fixed byte octets[16];
	}

	[StructLayout(LayoutKind.Explicit)]
	internal struct CIpUnion
	{
		[FieldOffset(0)] public CIpv4Addr v4;
		[FieldOffset(0)] public CIpv6Addr v6;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CIpAddr
	{
		public byte v6;
		public CIpUnion ip;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CSocketAddrV4
	{
		public CIpv4Addr ip;
		public ushort port;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CSocketAddrV6
	{
		public CIpv6Addr ip;
		public ushort port;
		public uint flowinfo;
		public uint scope_id;
	}

	[StructLayout(LayoutKind.Explicit)]
	internal struct CSocketUnion
	{
		[FieldOffset(0)] public CSocketAddrV4 v4;
		[FieldOffset(0)] public CSocketAddrV6 v6;
	}

	[StructLayout(LayoutKind.Sequential)]
	internal struct CSocketAddr
	{
		public byte v6;
		public CSocketUnion addr;
	}

	internal const int PollPending = 0;
	internal const int PollReady = 1;
	internal const int PollError = 2;

	/// The library's target, as its index among those this file serves.
	internal static readonly int Target = (int)target();

	/// Throws unless the library's target is among those of `on`, as the item `what` is.
	internal static void On(ulong on, string what)
	{
		if (((on >> Target) & 1) == 0)
		{
			throw new PlatformNotSupportedException(what + ": it is not on this library's target");
		}
	}

	/// Throws what `status`, which a call of `what` wrote, says failed, if anything.
	[MethodImpl(MethodImplOptions.AggressiveInlining)]
	internal static void Check(int status, string what)
	{
		if (status != 0)
		{
			throw Failure(status, what);
		}
	}

	/// The error a failed check throws: out of line, so each call that checks stays small.
	[MethodImpl(MethodImplOptions.NoInlining)]
	internal static Exception Failure(int status, string what) =>
		new InvalidOperationException(what + ": " + Marshal.PtrToStringAnsi((IntPtr)status_message(status)));

	/// Whether the platform is Windows, whose paths are WTF-8.
	internal static readonly bool Windows = RuntimeInformation.IsOSPlatform(OSPlatform.Windows);

	/// Whether `b` is WTF-8: UTF-8 that may also hold a surrogate's code point, but never a
	/// lead surrogate's right before a trail's, which a pair's own encoding spells.
	internal static bool IsWtf8(ReadOnlySpan<byte> b)
	{
		var lead = false;
		var i = 0;
		while (i < b.Length)
		{
			var c = b[i];
			if (c < 0x80)
			{
				lead = false;
				i++;
				continue;
			}
			int n;
			byte lo = 0x80, hi = 0xBF;
			if (c >= 0xC2 && c <= 0xDF)
			{
				n = 1;
			}
			else if (c >= 0xE0 && c <= 0xEF)
			{
				n = 2;
				if (c == 0xE0)
				{
					lo = 0xA0;
				}
			}
			else if (c >= 0xF0 && c <= 0xF4)
			{
				n = 3;
				if (c == 0xF0)
				{
					lo = 0x90;
				}
				else if (c == 0xF4)
				{
					hi = 0x8F;
				}
			}
			else
			{
				return false;
			}
			if (b.Length - i <= n || b[i + 1] < lo || b[i + 1] > hi)
			{
				return false;
			}
			for (var k = 2; k <= n; k++)
			{
				if ((b[i + k] & 0xC0) != 0x80)
				{
					return false;
				}
			}
			var trail = c == 0xED && b[i + 1] >= 0xB0;
			if (lead && trail)
			{
				return false;
			}
			lead = c == 0xED && b[i + 1] >= 0xA0 && b[i + 1] <= 0xAF;
			i += n + 1;
		}
		return true;
	}

	/// Runs what other threads left a closing handle, or this thread, given none: whether any ran.
	internal static bool CloseDrain(IntPtr raw) => close_drain(raw) != 0;

	/// Ends the process with `message`, as Rust's panic would: where no caller can take an
	/// exception, such as a callback Rust made.
	internal static void Abort(string message, Exception? error = null)
	{
		Console.Error.WriteLine(error == null ? message : message + " " + error);
		Console.Error.Flush();
		Environment.FailFast(error == null ? message : message + " " + error.Message);
	}

	/// The empty view: a null pointer is `None`, so `Some` of no bytes points somewhere.
	static readonly byte* empty = (byte*)Marshal.AllocHGlobal(1);

	/// A view of `bytes`, pinned by `pin`, which Rust reads during the call and never frees.
	internal static Buf View(ReadOnlySpan<byte> bytes, byte* pinned) =>
		new Buf { ptr = bytes.Length == 0 ? empty : pinned, len = (nuint)bytes.Length, owner = null };

	/// A view of `bytes`, which `arena` keeps pinned until the call returns.
	internal static Buf View(ReadOnlyMemory<byte> bytes, Arena arena)
	{
		if (bytes.Length == 0)
		{
			return new Buf { ptr = empty, len = 0, owner = null };
		}
		var pin = bytes.Pin();
		arena.Add(pin);
		return new Buf { ptr = (byte*)pin.Pointer, len = (nuint)bytes.Length, owner = null };
	}

	/// The null view, `None`.
	internal static Buf None => default;

	/// The bytes of `buf`, which Rust returned: copied into C#'s memory, then freed.
	internal static ReadOnlyMemory<byte> Bytes(Buf buf)
	{
		var bytes = Copy(buf);
		buf_free(buf);
		return bytes;
	}

	/// The bytes `buf` views, which Rust lends for a call, copied.
	internal static byte[] Copy(Buf buf)
	{
		var len = checked((int)buf.len);
		if (len == 0)
		{
			return System.Array.Empty<byte>();
		}
#if NET5_0_OR_GREATER
		var bytes = GC.AllocateUninitializedArray<byte>(len);
#else
		var bytes = new byte[len];
#endif
		new ReadOnlySpan<byte>(buf.ptr, len).CopyTo(bytes);
		return bytes;
	}

	/// `Some` bytes of `buf`, or null for its null pointer.
	// Typed, as a bare `null` would convert to the empty memory through `byte[]`.
	internal static ReadOnlyMemory<byte>? OptionBytes(Buf buf) => buf.ptr == null ? (ReadOnlyMemory<byte>?)null : Bytes(buf);

	internal static Str String(Buf buf) => new Str(Bytes(buf));

	internal static Str? OptionString(Buf buf) => buf.ptr == null ? (Str?)null : String(buf);

	internal static PathBuf Path(Buf buf)
	{
		var bytes = Copy(buf);
		buf_free(buf);
		return new PathBuf(bytes, true);
	}

	internal static PathBuf? OptionPath(Buf buf) => buf.ptr == null ? (PathBuf?)null : Path(buf);

	/// A new array of `n` elements, as a generic method writes one whatever `T`'s syntax.
	internal static T[] NewArray<T>(int n) => new T[n];

	/// A buffer Rust owns, holding a copy of `bytes`: what a callback hands Rust.
	internal static Buf Owned(ReadOnlySpan<byte> bytes)
	{
		fixed (byte* p = bytes)
		{
			return buf_copy(View(bytes, p));
		}
	}

	/// A buffer Rust owns, holding `text`'s message: what a failed callback hands Rust.
	internal static Buf Message(string text) => Owned(System.Text.Encoding.UTF8.GetBytes(text));

	internal static CU128 U128(U128 v) => new CU128 { lo = v.Low, hi = v.High };

	internal static U128 U128(CU128 c) => new U128(c.hi, c.lo);

	internal static CDuration Duration(Duration d) => new CDuration { secs = d.Secs, nanos = d.Nanos };

	internal static Duration Duration(CDuration c) => new Duration(c.secs, c.nanos);

	internal static CIpv4Addr Addr(Ipv4Addr a)
	{
		var c = default(CIpv4Addr);
		for (var k = 0; k < 4; k++)
		{
			c.octets[k] = (byte)(a.Bits >> (24 - 8 * k));
		}
		return c;
	}

	internal static Ipv4Addr Addr(CIpv4Addr c)
	{
		uint bits = 0;
		for (var k = 0; k < 4; k++)
		{
			bits = bits << 8 | c.octets[k];
		}
		return new Ipv4Addr(bits);
	}

	internal static CIpv6Addr Addr(Ipv6Addr a)
	{
		var c = default(CIpv6Addr);
		for (var k = 0; k < 16; k++)
		{
			c.octets[k] = a[k];
		}
		return c;
	}

	internal static Ipv6Addr Addr(CIpv6Addr c)
	{
		var octets = new byte[16];
		for (var k = 0; k < 16; k++)
		{
			octets[k] = c.octets[k];
		}
		return new Ipv6Addr(octets);
	}

	internal static CIpAddr Addr(IpAddr a)
	{
		var c = default(CIpAddr);
		if (a.V6 is Ipv6Addr v6)
		{
			c.v6 = 1;
			c.ip.v6 = Addr(v6);
		}
		else
		{
			c.ip.v4 = Addr(a.V4!.Value);
		}
		return c;
	}

	internal static IpAddr Addr(CIpAddr c) => c.v6 != 0 ? new IpAddr(Addr(c.ip.v6)) : new IpAddr(Addr(c.ip.v4));

	internal static CSocketAddrV4 Addr(SocketAddrV4 a) => new CSocketAddrV4 { ip = Addr(a.Ip), port = a.Port };

	internal static SocketAddrV4 Addr(CSocketAddrV4 c) => new SocketAddrV4(Addr(c.ip), c.port);

	internal static CSocketAddrV6 Addr(SocketAddrV6 a) =>
		new CSocketAddrV6 { ip = Addr(a.Ip), port = a.Port, flowinfo = a.FlowInfo, scope_id = a.ScopeId };

	internal static SocketAddrV6 Addr(CSocketAddrV6 c) => new SocketAddrV6(Addr(c.ip), c.port, c.flowinfo, c.scope_id);

	internal static CSocketAddr Addr(SocketAddr a)
	{
		var c = default(CSocketAddr);
		if (a.V6 is SocketAddrV6 v6)
		{
			c.v6 = 1;
			c.addr.v6 = Addr(v6);
		}
		else
		{
			c.addr.v4 = Addr(a.V4!.Value);
		}
		return c;
	}

	internal static SocketAddr Addr(CSocketAddr c) =>
		c.v6 != 0 ? new SocketAddr(Addr(c.addr.v6)) : new SocketAddr(Addr(c.addr.v4));

	/// An array of `len` C values Rust reads for the call, which `arena` frees once it returns.
	internal static T* Array<T>(int len, Arena arena) where T : unmanaged
	{
		if (len == 0)
		{
			return (T*)empty;
		}
		var p = (T*)Marshal.AllocHGlobal(checked(len * sizeof(T)));
		arena.Add(p);
		return p;
	}

	/// The handle of each object in `items`, which the call borrows or takes, in an array
	/// `arena` frees.
	internal static IntPtr* Handles<T>(T[] items, Arena arena, bool take) where T : RustObject
	{
		var p = Array<IntPtr>(items.Length, arena);
		for (var k = 0; k < items.Length; k++)
		{
			p[k] = arena.Hold(items[k], take);
		}
		return p;
	}

	/// The items of a block Rust returned, copied out whole.
	internal static T[] Block<T>(void* ptr, nuint len) where T : unmanaged
	{
		var n = checked((int)len);
		if (n == 0)
		{
			return System.Array.Empty<T>();
		}
		var items = new T[n];
		new ReadOnlySpan<T>(ptr, n).CopyTo(items);
		return items;
	}

	/// The `GCHandle` Rust holds as a callback's data, freed once Rust lets it go.
	internal static void* Give(object target) => (void*)GCHandle.ToIntPtr(GCHandle.Alloc(target));

	/// The object behind data `Give` made.
	internal static T Held<T>(void* data) where T : class => (T)GCHandle.FromIntPtr((IntPtr)data).Target!;

	/// Frees data `Give` made.
	internal static void Release(void* data) => GCHandle.FromIntPtr((IntPtr)data).Free();

	/// Wakes a waker Rust returned.
	internal static void Wake(CWaker w)
	{
		if (w.wake != IntPtr.Zero)
		{
			((delegate* unmanaged[Cdecl]<void*, void>)w.wake)(w.data);
		}
	}

	/// Drops a waker Rust returned.
	internal static void Drop(CWaker w)
	{
		if (w.drop != IntPtr.Zero)
		{
			((delegate* unmanaged[Cdecl]<void*, void>)w.drop)(w.data);
		}
	}

	/// The C waker of `wake`, a C# action, which Rust calls from any thread until it drops it.
	internal static CWaker Waker(Action wake) =>
		new CWaker { wake = Callbacks.ActionWake, drop = Callbacks.ActionDrop, data = Give(wake) };

	/// The C future of `run`, which Rust polls from any thread until it drops it.
	internal static CForeignFuture Future(Func<CancellationToken, Task> run) =>
		new CForeignFuture { poll = Callbacks.FuturePoll, free = Callbacks.FutureFree, data = Give(new ForeignFuture(run)) };

	/// The C reader of `reader`, lent for one call, whose data the caller releases after it.
	internal static CForeignReader Reader(ISeekableReader reader) =>
		new CForeignReader
		{
			read = Callbacks.ReaderRead,
			seek = Callbacks.ReaderSeek,
			data = Give(reader ?? throw new ArgumentNullException(nameof(reader))),
		};
}

/// What a call lowers its arguments into that must outlive the C values but not the call:
/// pins of C#'s memory, arrays it allocates, and holds of the objects it borrows.
internal sealed unsafe class Arena : IDisposable
{
	List<MemoryHandle>? pins;
	List<IntPtr>? allocs;
	List<(RustObject, int, bool)>? holds;
	List<GCHandle>? handles;

	internal void Add(MemoryHandle pin) => (pins ??= new List<MemoryHandle>()).Add(pin);

	internal void Add(void* alloc) => (allocs ??= new List<IntPtr>()).Add((IntPtr)alloc);

	internal void Add(GCHandle handle) => (handles ??= new List<GCHandle>()).Add(handle);

	/// The handle of `o`, held until the call returns; `take` closes it then, as Rust took its value.
	internal IntPtr Hold(RustObject o, bool take)
	{
		var raw = o.enter(out var slot);
		(holds ??= new List<(RustObject, int, bool)>()).Add((o, slot, take));
		return raw;
	}

	/// A pinned pointer to `bytes`, or the empty one, for a view.
	internal Native.Buf View(ReadOnlyMemory<byte> bytes) => Native.View(bytes, this);

	internal Native.Buf View(Str text) => Native.View(text.Utf8, this);

	internal Native.Buf View(PathBuf path) => Native.View(path.Bytes, this);

	/// Whether the objects held next move into the call, as a value Rust takes does.
	internal bool Take = true;

	/// Whether the call ran, so each object it took moved out.
	internal bool Ran;

	public void Dispose()
	{
		if (pins != null)
		{
			foreach (var pin in pins)
			{
				pin.Dispose();
			}
		}
		if (allocs != null)
		{
			foreach (var p in allocs)
			{
				Marshal.FreeHGlobal(p);
			}
		}
		if (handles != null)
		{
			foreach (var h in handles)
			{
				h.Free();
			}
		}
		if (holds != null)
		{
			foreach (var (o, slot, take) in holds)
			{
				o.exit(slot);
				if (take && Ran)
				{
					o.moved();
				}
			}
		}
	}
}

/// An object Rust owns behind a handle, which it frees once disposed and no call holds it,
/// or once the garbage collector finds it unreachable. Each call holds the handle, so a
/// dispose frees it only once no call does, and a call after it throws
/// `ObjectDisposedException`. Dispose of one whose value Rust checks drops the value as
/// Rust's own drop does, throwing while a call borrows it in Rust; of any other, it throws
/// while a call holds it.
public abstract unsafe class RustObject : IDisposable
{
	/// The handle, an `Arc` Rust made.
	internal readonly IntPtr raw;

	/// The holds, one for the owner until closed and one per call, shifted past a bit set
	/// once closed, one set while a dispose that may yet fail decides, and one set once calls
	/// contended for them, after which they hold it in `slots`: one word, so a call reads them
	/// all at once, and below `Held` only once no hold is left.
	long state = Held;

	/// What a slot holds for this handle, given once calls first contend for it, before
	/// `Bias`: unique while the handle lives, unlike `raw`, which Rust reuses once freed.
	long key;

	/// Whether Rust lent the handle for one call, so C# never frees it.
	readonly bool lent;

	const long Closed = 1, Bias = 2, Closing = 4, Held = 8;

	/// Set in a slot whose hold a close counted.
	const long Revoked = 1;

	// Once calls contend for a handle's count, each holds it in a slot of `slots` instead, so
	// threads sharing a handle write nothing in common. A close that would free the handle
	// claims it closed first, then counts each hold it finds there, which lets go through the
	// count, so the last to let go frees it: ffi/tests/hold.rs models this. A cache line of
	// rows to a column, which the thread picks, and a row per handle: two threads in one
	// column, or two handles in one row on one thread, use the count. Two cache lines to a
	// column, of which it uses one's worth, so a column shares no line with the next
	// wherever the array starts.
	const int RowBits = 3, ColumnBits = 8, Stride = 2 << RowBits;
	static readonly long[] slots = new long[Stride << ColumnBits];

	/// The last key given, in steps of two, so a key's low bit is clear for `Revoked`.
	static long keys;

	internal RustObject(IntPtr raw, bool lent = false)
	{
		this.raw = raw;
		this.lent = lent;
		if (lent)
		{
			GC.SuppressFinalize(this);
		}
	}

	/// Frees the handle.
	internal abstract void free(IntPtr raw);

	/// Whether Rust checks its value, which `drop` then drops.
	internal virtual bool dropped => false;

	/// Drops the value now, as Rust's `drop` would, writing why it could not to `status`;
	/// given `commit`, Rust calls it with `state` once the drop can no longer fail.
	internal virtual void drop(IntPtr raw, int* status, IntPtr commit, long* state)
	{
	}

	/// The handle, held for a call until `exit` takes `slot`; throws once closed.
	[MethodImpl(MethodImplOptions.AggressiveInlining)]
	internal IntPtr enter(out int slot)
	{
		var n = Volatile.Read(ref state);
		if ((n & (Closed | Bias)) == 0 && Interlocked.CompareExchange(ref state, n + Held, n) == n)
		{
			slot = -1;
			return raw;
		}
		slot = contend();
		return raw;
	}

	/// `enter` once calls contend for it: in the table, unless another handle or thread has
	/// its slot, then in the count, after which later calls go to the table.
	int contend()
	{
		if ((Volatile.Read(ref state) & Bias) != 0)
		{
			var i = slot();
			var k = Volatile.Read(ref key);
			if (Interlocked.CompareExchange(ref slots[i], k, 0) == 0)
			{
				if ((Volatile.Read(ref state) & Closed) == 0)
				{
					return i;
				}
				// A close claimed it meanwhile, and may have counted this hold.
				if (Interlocked.CompareExchange(ref slots[i], 0, k) != k)
				{
					Volatile.Write(ref slots[i], 0);
					exit(-1);
				}
				throw new ObjectDisposedException(GetType().Name);
			}
		}
		while (true)
		{
			var n = Volatile.Read(ref state);
			if ((n & Closed) != 0)
			{
				throw new ObjectDisposedException(GetType().Name);
			}
			if (Interlocked.CompareExchange(ref state, n + Held, n) == n)
			{
				break;
			}
		}
		if (Volatile.Read(ref key) == 0)
		{
			Interlocked.CompareExchange(ref key, Interlocked.Add(ref keys, 2), 0);
		}
		while (true)
		{
			var n = Volatile.Read(ref state);
			if ((n & Bias) != 0 || Interlocked.CompareExchange(ref state, n | Bias, n) == n)
			{
				return -1;
			}
		}
	}

	/// This thread's slot of the table for this handle: a column of a cache line per thread,
	/// and a row in it per handle.
	int slot()
	{
		var column = (int)((uint)Environment.CurrentManagedThreadId * 0x9E3779B9u >> (32 - ColumnBits));
		var row = (int)(((ulong)raw >> 4) * 0x9E3779B97F4A7C15UL >> (64 - RowBits));
		return column * Stride + row;
	}

	/// Ends a call's hold, wherever `enter` put it, freeing the handle if it closed meanwhile
	/// and this was the last.
	[MethodImpl(MethodImplOptions.AggressiveInlining)]
	internal void exit(int slot)
	{
		if (slot >= 0 && Interlocked.Exchange(ref slots[slot], 0) == Volatile.Read(ref key))
		{
			return;
		}
		freed(Interlocked.Add(ref state, -Held));
	}

	/// Frees it once no hold is left, which happens once.
	void freed(long n)
	{
		if (n < Held && !lent)
		{
			free(raw);
		}
	}

	/// Whether a call holds it in the table.
	bool listed()
	{
		if ((Volatile.Read(ref state) & Bias) == 0)
		{
			return false;
		}
		var k = Volatile.Read(ref key);
		for (var i = slot() % Stride; i < slots.Length; i += Stride)
		{
			if (Volatile.Read(ref slots[i]) == k)
			{
				return true;
			}
		}
		return false;
	}

	/// Closes it once a call moved its value out, freeing the handle once no call holds it.
	internal void moved() => abandon();

	/// Ends a lent handle once the call Rust lent it for returns, which Rust frees next: a call
	/// still holding it would reach the freed handle, so it aborts.
	internal void end()
	{
		abandon();
		if ((Volatile.Read(ref state) & ~Bias) != Closed)
		{
			Native.Abort(GetType().Name + ": a call still holds it when its lend ends", null);
		}
	}

	/// Whether it is closed.
	public bool IsClosed => (Volatile.Read(ref state) & Closed) != 0;

	/// Gives up the owner's hold, freeing it once no call holds one.
	void abandon()
	{
		while (true)
		{
			var n = Volatile.Read(ref state);
			// A close is deciding: wait for its outcome, which may leave it open.
			if ((n & Closing) != 0)
			{
				Thread.Yield();
				continue;
			}
			if ((n & Closed) != 0)
			{
				return;
			}
			if (Interlocked.CompareExchange(ref state, n | Closed, n) == n)
			{
				break;
			}
		}
		GC.SuppressFinalize(this);
		// Claimed closed, so each call holding it in the table did so before: count them.
		if ((Volatile.Read(ref state) & Bias) != 0)
		{
			var k = Volatile.Read(ref key);
			for (var i = slot() % Stride; i < slots.Length; i += Stride)
			{
				if (Volatile.Read(ref slots[i]) == k)
				{
					Interlocked.Add(ref state, Held);
					if (Interlocked.CompareExchange(ref slots[i], k | Revoked, k) != k)
					{
						Interlocked.Add(ref state, -Held);
					}
				}
			}
		}
		exit(-1);
	}

	/// Drops its value through Rust's own `drop`, then frees its handle once no call holds
	/// it. A value a call borrows is refused, as Rust refuses that drop, throwing and leaving
	/// it open; a second dispose does nothing.
	public void Dispose()
	{
		if (lent)
		{
			abandon();
			return;
		}
		if (dropped)
		{
			disposeChecked();
			return;
		}
		while (true)
		{
			var n = Volatile.Read(ref state);
			// Another dispose is deciding: wait for its outcome, which may leave it open.
			if ((n & Closing) != 0)
			{
				Thread.Yield();
				continue;
			}
			if ((n & Closed) != 0)
			{
				return;
			}
			if ((n & ~Bias) != Held)
			{
				throw Borrowed();
			}
			if (Interlocked.CompareExchange(ref state, n | Closed | Closing, n) == n)
			{
				break;
			}
		}
		// Claimed closed, so a call holding it in the table after this finds it closed, and
		// one before is in the table. A call that left the table finding it closed held
		// nothing, so the count is still the owner's alone.
		if (listed())
		{
			Interlocked.Add(ref state, -(Closed | Closing));
			throw Borrowed();
		}
		Volatile.Write(ref state, Closed);
		GC.SuppressFinalize(this);
		freed(Closed);
	}

	Exception Borrowed() => new InvalidOperationException(GetType().Name + ".Dispose: a handle is already borrowed");

	/// `Dispose` of a handle whose value Rust checks.
	void disposeChecked()
	{
		while (true)
		{
			// Held by no call, as soon after its return: claimed closed so none starts, then
			// the value dropped and the handle freed in one crossing. A call meanwhile throws
			// as closed, as one racing any dispose may. A drop that passed stored `Closed`
			// itself.
			while (Interlocked.CompareExchange(ref state, Closed | Closing, Held) != Held)
			{
				if (disposeMet())
				{
					return;
				}
			}
			int status = 0;
			fixed (long* s = &state)
			{
				drop(raw, &status, Callbacks.Commit, s);
			}
			if (status == 0)
			{
				GC.SuppressFinalize(this);
				free(raw);
				return;
			}
			// Rust refused the drop, before it committed: reopened. It refused before
			// draining what other threads left this one, which may end the borrow that
			// refused it: drained, it tries again.
			Volatile.Write(ref state, Held);
			if (Native.CloseDrain(raw))
			{
				continue;
			}
			throw Native.Failure(status, GetType().Name + ".Dispose");
		}
	}

	/// `disposeChecked` once a call holds it or another dispose is deciding: whether it is
	/// done, rather than to try the one crossing again.
	bool disposeMet()
	{
		var n = Volatile.Read(ref state);
		// Another dispose is deciding: wait for its outcome, which may leave it open. Rust
		// clears `Closing` once the drop can no longer fail, and runs none of the program's
		// code on the deciding thread before then, so a dispose a `Drop` makes, from any
		// thread, finds it closed rather than waiting on itself.
		if ((n & Closing) != 0)
		{
			Native.CloseDrain(IntPtr.Zero);
			Thread.Yield();
			return false;
		}
		if ((n & Closed) != 0)
		{
			return true;
		}
		int slot;
		IntPtr r;
		try
		{
			r = enter(out slot);
		}
		catch (ObjectDisposedException)
		{
			return false;
		}
		int status = 0;
		try
		{
			drop(r, &status, IntPtr.Zero, null);
		}
		finally
		{
			exit(slot);
		}
		if (status != 0)
		{
			throw Native.Failure(status, GetType().Name + ".Dispose");
		}
		abandon();
		return true;
	}

	~RustObject()
	{
		if ((Volatile.Read(ref state) & Closed) == 0)
		{
			free(raw);
		}
	}
}

/// Rust's text: UTF-8, which C#'s `string` converts to only where it holds no lone
/// surrogate, as Rust's text never does. `Str.From` and the implicit conversion check a
/// `string` once; `ToString()` gives one back.
public readonly struct Str : IEquatable<Str>, IComparable<Str>
{
	static readonly System.Text.UTF8Encoding strict = new System.Text.UTF8Encoding(false, true);

	/// Its UTF-8 bytes.
	public ReadOnlyMemory<byte> Utf8 { get; }

	internal Str(ReadOnlyMemory<byte> utf8)
	{
		Utf8 = utf8;
	}

	/// `text` as Rust's, throwing an `ArgumentException` where it holds a lone surrogate,
	/// which no Rust text holds.
	public static Str From(string text)
	{
		if (text == null)
		{
			throw new ArgumentNullException(nameof(text));
		}
		try
		{
			return new Str(strict.GetBytes(text));
		}
		catch (System.Text.EncoderFallbackException e)
		{
			throw new ArgumentException("a string holding a lone surrogate is no Rust text", nameof(text), e);
		}
	}

	/// The text of `utf8`, copied, so a later change to the caller's bytes cannot unmake it,
	/// throwing an `ArgumentException` unless it is UTF-8.
	public static Str FromUtf8(ReadOnlySpan<byte> utf8)
	{
		var copy = utf8.ToArray();
		try
		{
			strict.GetCharCount(copy);
		}
		catch (System.Text.DecoderFallbackException e)
		{
			throw new ArgumentException("bytes that are not UTF-8 are no Rust text", nameof(utf8), e);
		}
		return new Str(copy);
	}

	public static implicit operator Str(string text) => From(text);

	/// Its length in bytes, as Rust's `len`.
	public int Length => Utf8.Length;

	public override string ToString() => System.Text.Encoding.UTF8.GetString(Utf8.Span);

	public bool Equals(Str other) => Utf8.Span.SequenceEqual(other.Utf8.Span);

	public override bool Equals(object? obj) => obj is Str other && Equals(other);

	public override int GetHashCode() => Native.HashBytes(Utf8.Span);

	/// Rust's order of text: by its bytes.
	public int CompareTo(Str other) => Utf8.Span.SequenceCompareTo(other.Utf8.Span);

	public static bool operator ==(Str a, Str b) => a.Equals(b);

	public static bool operator !=(Str a, Str b) => !a.Equals(b);
}

/// A Rust path: the OS's bytes, any on Unix and WTF-8 on Windows, so one that is no text
/// keeps them.
public readonly struct PathBuf : IEquatable<PathBuf>
{
	/// Its bytes as the OS holds them.
	public ReadOnlyMemory<byte> Bytes { get; }

	/// The path of `bytes`, copied, so a later change to the caller's bytes cannot unmake it:
	/// on Windows, throwing an `ArgumentException` unless they are WTF-8.
	public PathBuf(ReadOnlySpan<byte> bytes)
	{
		var copy = bytes.ToArray();
		if (Native.Windows && !Native.IsWtf8(copy))
		{
			throw new ArgumentException("bytes that are not WTF-8 are no Windows path", nameof(bytes));
		}
		Bytes = copy;
	}

	/// Rust's bytes of a path, as they are.
	internal PathBuf(byte[] bytes, bool unchecked_)
	{
		Bytes = bytes;
	}

	/// The path of `text`, throwing an `ArgumentException` where it holds a lone surrogate.
	public PathBuf(string text)
	{
		Bytes = Str.From(text).Utf8;
	}

	public static implicit operator PathBuf(string text) => new PathBuf(text);

	/// Its text, throwing a `FormatException` unless it is UTF-8.
	public string ToText()
	{
		try
		{
			return new System.Text.UTF8Encoding(false, true).GetString(Bytes.Span);
		}
		catch (System.Text.DecoderFallbackException e)
		{
			throw new FormatException("the path is not UTF-8", e);
		}
	}

	/// Its text, each part that is not UTF-8 as U+FFFD, as Rust's `display` shows it.
	public override string ToString() => System.Text.Encoding.UTF8.GetString(Bytes.Span);

	public bool Equals(PathBuf other) => Bytes.Span.SequenceEqual(other.Bytes.Span);

	public override bool Equals(object? obj) => obj is PathBuf other && Equals(other);

	public override int GetHashCode() => Native.HashBytes(Bytes.Span);
}

/// A Rust `char`: a Unicode scalar value, which C#'s `char` cannot hold past the BMP.
public readonly struct Scalar : IEquatable<Scalar>, IComparable<Scalar>
{
	/// Its code point.
	public uint Value { get; }

	/// The scalar `value`, throwing an `ArgumentOutOfRangeException` for a surrogate or past
	/// U+10FFFF.
	public Scalar(uint value)
	{
		if (value > 0x10FFFF || (value >= 0xD800 && value <= 0xDFFF))
		{
			throw new ArgumentOutOfRangeException(nameof(value), "not a Unicode scalar value");
		}
		Value = value;
	}

	internal Scalar(uint value, bool unchecked_)
	{
		Value = value;
	}

	public static implicit operator Scalar(char c) => new Scalar(c);

	public override string ToString() => char.ConvertFromUtf32((int)Value);

	public bool Equals(Scalar other) => Value == other.Value;

	public override bool Equals(object? obj) => obj is Scalar other && Equals(other);

	public override int GetHashCode() => (int)Value;

	public int CompareTo(Scalar other) => Value.CompareTo(other.Value);

	public static bool operator ==(Scalar a, Scalar b) => a.Value == b.Value;

	public static bool operator !=(Scalar a, Scalar b) => a.Value != b.Value;
}

/// A Rust `u128`, of its high and low 64 bits.
public readonly struct U128 : IEquatable<U128>, IComparable<U128>
{
	public ulong High { get; }
	public ulong Low { get; }

	public U128(ulong high, ulong low)
	{
		High = high;
		Low = low;
	}

	public static implicit operator U128(ulong v) => new U128(0, v);

	/// It as a `BigInteger`, which holds every value.
	public System.Numerics.BigInteger ToBigInteger() => (new System.Numerics.BigInteger(High) << 64) | Low;

	/// `v` as a `U128`, throwing an `ArgumentOutOfRangeException` unless it is in range.
	public static U128 From(System.Numerics.BigInteger v)
	{
		if (v.Sign < 0 || v >> 128 != 0)
		{
			throw new ArgumentOutOfRangeException(nameof(v), "not a u128");
		}
		return new U128((ulong)(v >> 64), (ulong)(v & ulong.MaxValue));
	}

#if NET7_0_OR_GREATER
	public static implicit operator UInt128(U128 v) => new UInt128(v.High, v.Low);

	public static implicit operator U128(UInt128 v) => new U128((ulong)(v >> 64), (ulong)v);
#endif

	/// Its hex digits, zero-padded to `width`.
	public string ToHex(int width = 0)
	{
		var text = High == 0 ? Low.ToString("x") : High.ToString("x") + Low.ToString("x16");
		return text.PadLeft(width, '0');
	}

	public override string ToString() => ToBigInteger().ToString();

	public bool Equals(U128 other) => High == other.High && Low == other.Low;

	public override bool Equals(object? obj) => obj is U128 other && Equals(other);

	public override int GetHashCode() => (High ^ Low).GetHashCode();

	public int CompareTo(U128 other) => High != other.High ? High.CompareTo(other.High) : Low.CompareTo(other.Low);

	public static bool operator ==(U128 a, U128 b) => a.Equals(b);

	public static bool operator !=(U128 a, U128 b) => !a.Equals(b);
}

/// A Rust `Duration`: whole seconds and the nanoseconds past them, below a second, so it
/// holds every value Rust's does and no other, which `TimeSpan` cannot.
public readonly struct Duration : IEquatable<Duration>, IComparable<Duration>
{
	public ulong Secs { get; }
	public uint Nanos { get; }

	/// `secs` and `nanos`, throwing an `ArgumentOutOfRangeException` for a second's
	/// nanoseconds or more.
	public Duration(ulong secs, uint nanos = 0)
	{
		if (nanos >= 1_000_000_000)
		{
			throw new ArgumentOutOfRangeException(nameof(nanos), "a second or more of nanoseconds");
		}
		Secs = secs;
		Nanos = nanos;
	}

	/// `span`, throwing an `ArgumentOutOfRangeException` for a negative one.
	public static Duration From(TimeSpan span)
	{
		if (span.Ticks < 0)
		{
			throw new ArgumentOutOfRangeException(nameof(span), "a negative duration");
		}
		return new Duration((ulong)(span.Ticks / TimeSpan.TicksPerSecond), (uint)(span.Ticks % TimeSpan.TicksPerSecond * 100));
	}

	/// It as a `TimeSpan`, throwing an `OverflowException` past its longest, and dropping
	/// what is below its 100 ns tick.
	public TimeSpan ToTimeSpan() => new TimeSpan(checked((long)Secs * TimeSpan.TicksPerSecond + Nanos / 100));

	public bool Equals(Duration other) => Secs == other.Secs && Nanos == other.Nanos;

	public override bool Equals(object? obj) => obj is Duration other && Equals(other);

	public override int GetHashCode() => (Secs, Nanos).GetHashCode();

	public int CompareTo(Duration other) => Secs != other.Secs ? Secs.CompareTo(other.Secs) : Nanos.CompareTo(other.Nanos);

	public override string ToString() => Secs + "s " + Nanos + "ns";

	public static bool operator ==(Duration a, Duration b) => a.Equals(b);

	public static bool operator !=(Duration a, Duration b) => !a.Equals(b);
}

/// An IPv4 address, as Rust's `Ipv4Addr` holds it.
public readonly struct Ipv4Addr : IEquatable<Ipv4Addr>
{
	/// Its 32 bits, the first octet highest, as Rust's `to_bits` gives them.
	public uint Bits { get; }

	public Ipv4Addr(uint bits)
	{
		Bits = bits;
	}

	public Ipv4Addr(byte a, byte b, byte c, byte d)
	{
		Bits = (uint)a << 24 | (uint)b << 16 | (uint)c << 8 | d;
	}

	/// Its 4 octets, in network order.
	public byte[] Octets() => new[] { (byte)(Bits >> 24), (byte)(Bits >> 16), (byte)(Bits >> 8), (byte)Bits };

	/// It as an `IPAddress`.
	public System.Net.IPAddress ToIPAddress() => new System.Net.IPAddress(Octets());

	/// `address`, throwing an `ArgumentException` unless it is IPv4.
	public static Ipv4Addr From(System.Net.IPAddress address)
	{
		if (address.AddressFamily != System.Net.Sockets.AddressFamily.InterNetwork)
		{
			throw new ArgumentException("not an IPv4 address", nameof(address));
		}
		var o = address.GetAddressBytes();
		return new Ipv4Addr(o[0], o[1], o[2], o[3]);
	}

	public override string ToString() => $"{Bits >> 24}.{(Bits >> 16) & 255}.{(Bits >> 8) & 255}.{Bits & 255}";

	public bool Equals(Ipv4Addr other) => Bits == other.Bits;

	public override bool Equals(object? obj) => obj is Ipv4Addr other && Equals(other);

	public override int GetHashCode() => (int)Bits;
}

/// An IPv6 address, as Rust's `Ipv6Addr` holds it.
public readonly struct Ipv6Addr : IEquatable<Ipv6Addr>
{
	readonly ulong high, low;

	/// The address of `octets`, in network order, throwing an `ArgumentException` unless it
	/// holds exactly 16.
	public Ipv6Addr(ReadOnlySpan<byte> octets)
	{
		if (octets.Length != 16)
		{
			throw new ArgumentException("an IPv6 address is 16 octets", nameof(octets));
		}
		high = 0;
		low = 0;
		for (var k = 0; k < 8; k++)
		{
			high = high << 8 | octets[k];
			low = low << 8 | octets[k + 8];
		}
	}

	/// Its `k`th octet.
	public byte this[int k] => (byte)((k < 8 ? high : low) >> (56 - 8 * (k % 8)));

	/// Its 16 octets, in network order.
	public byte[] Octets()
	{
		var octets = new byte[16];
		for (var k = 0; k < 16; k++)
		{
			octets[k] = this[k];
		}
		return octets;
	}

	/// It as an `IPAddress`, with no scope, which Rust's has none of.
	public System.Net.IPAddress ToIPAddress() => new System.Net.IPAddress(Octets());

	/// `address`, throwing an `ArgumentException` unless it is IPv6 with no scope.
	public static Ipv6Addr From(System.Net.IPAddress address)
	{
		if (address.AddressFamily != System.Net.Sockets.AddressFamily.InterNetworkV6 || address.ScopeId != 0)
		{
			throw new ArgumentException("not an IPv6 address without a scope", nameof(address));
		}
		return new Ipv6Addr(address.GetAddressBytes());
	}

	public override string ToString() => ToIPAddress().ToString();

	public bool Equals(Ipv6Addr other) => high == other.high && low == other.low;

	public override bool Equals(object? obj) => obj is Ipv6Addr other && Equals(other);

	public override int GetHashCode() => (high, low).GetHashCode();
}

/// An IP address of either family, as Rust's `IpAddr`: exactly one of `V4` and `V6`.
public readonly struct IpAddr : IEquatable<IpAddr>
{
	// One family at a time, so `default` is 0.0.0.0, as Rust holds no address of neither.
	readonly Ipv4Addr v4;
	readonly Ipv6Addr v6;
	readonly bool isV6;

	public Ipv4Addr? V4 => isV6 ? null : v4;
	public Ipv6Addr? V6 => isV6 ? v6 : null;

	public IpAddr(Ipv4Addr v4)
	{
		this.v4 = v4;
		v6 = default;
		isV6 = false;
	}

	public IpAddr(Ipv6Addr v6)
	{
		v4 = default;
		this.v6 = v6;
		isV6 = true;
	}

	public static implicit operator IpAddr(Ipv4Addr v4) => new IpAddr(v4);

	public static implicit operator IpAddr(Ipv6Addr v6) => new IpAddr(v6);

	public override string ToString() => V6 is Ipv6Addr v6 ? v6.ToString() : V4!.Value.ToString();

	public bool Equals(IpAddr other) => V4.Equals(other.V4) && V6.Equals(other.V6);

	public override bool Equals(object? obj) => obj is IpAddr other && Equals(other);

	public override int GetHashCode() => (V4, V6).GetHashCode();
}

/// An IPv4 socket address, as Rust's `SocketAddrV4`.
public readonly struct SocketAddrV4 : IEquatable<SocketAddrV4>
{
	public Ipv4Addr Ip { get; }
	public ushort Port { get; }

	public SocketAddrV4(Ipv4Addr ip, ushort port)
	{
		Ip = ip;
		Port = port;
	}

	public override string ToString() => $"{Ip}:{Port}";

	public bool Equals(SocketAddrV4 other) => Ip.Equals(other.Ip) && Port == other.Port;

	public override bool Equals(object? obj) => obj is SocketAddrV4 other && Equals(other);

	public override int GetHashCode() => (Ip, Port).GetHashCode();
}

/// An IPv6 socket address, as Rust's `SocketAddrV6`, with its flow label and scope.
public readonly struct SocketAddrV6 : IEquatable<SocketAddrV6>
{
	public Ipv6Addr Ip { get; }
	public ushort Port { get; }
	public uint FlowInfo { get; }
	public uint ScopeId { get; }

	public SocketAddrV6(Ipv6Addr ip, ushort port, uint flowInfo = 0, uint scopeId = 0)
	{
		Ip = ip;
		Port = port;
		FlowInfo = flowInfo;
		ScopeId = scopeId;
	}

	public override string ToString() => ScopeId == 0 ? $"[{Ip}]:{Port}" : $"[{Ip}%{ScopeId}]:{Port}";

	public bool Equals(SocketAddrV6 other) =>
		Ip.Equals(other.Ip) && Port == other.Port && FlowInfo == other.FlowInfo && ScopeId == other.ScopeId;

	public override bool Equals(object? obj) => obj is SocketAddrV6 other && Equals(other);

	public override int GetHashCode() => (Ip, Port, FlowInfo, ScopeId).GetHashCode();
}

/// A socket address of either family, as Rust's `SocketAddr`: exactly one of `V4` and `V6`.
public readonly struct SocketAddr : IEquatable<SocketAddr>
{
	// One family at a time, so `default` is 0.0.0.0:0, as Rust holds no address of neither.
	readonly SocketAddrV4 v4;
	readonly SocketAddrV6 v6;
	readonly bool isV6;

	public SocketAddrV4? V4 => isV6 ? null : v4;
	public SocketAddrV6? V6 => isV6 ? v6 : null;

	public SocketAddr(SocketAddrV4 v4)
	{
		this.v4 = v4;
		v6 = default;
		isV6 = false;
	}

	public SocketAddr(SocketAddrV6 v6)
	{
		v4 = default;
		this.v6 = v6;
		isV6 = true;
	}

	/// `ip` at `port`, with no flow label or scope for IPv6, as Rust's `SocketAddr::new`.
	public SocketAddr(IpAddr ip, ushort port)
	{
		if (ip.V6 is Ipv6Addr ip6)
		{
			v4 = default;
			v6 = new SocketAddrV6(ip6, port);
			isV6 = true;
		}
		else
		{
			v4 = new SocketAddrV4(ip.V4!.Value, port);
			v6 = default;
			isV6 = false;
		}
	}

	public static implicit operator SocketAddr(SocketAddrV4 v4) => new SocketAddr(v4);

	public static implicit operator SocketAddr(SocketAddrV6 v6) => new SocketAddr(v6);

	/// Its port.
	public ushort Port => V6 is SocketAddrV6 v6 ? v6.Port : V4!.Value.Port;

	public override string ToString() => V6 is SocketAddrV6 v6 ? v6.ToString() : V4!.Value.ToString();

	public bool Equals(SocketAddr other) => V4.Equals(other.V4) && V6.Equals(other.V6);

	public override bool Equals(object? obj) => obj is SocketAddr other && Equals(other);

	public override int GetHashCode() => (V4, V6).GetHashCode();
}

/// A waker Rust returned: `Wake` wakes its task from any thread, any number of times, and
/// `Dispose`, or the garbage collector, drops it once.
public sealed unsafe class Waker : IDisposable
{
	Native.CWaker waker;

	/// Two per `Wake` in progress, plus one once disposed: the last of them to end drops the
	/// C waker, so a dispose racing a wake never frees what the wake reads.
	int state;

	/// Set once the C waker dropped.
	int freed;

	internal Waker(Native.CWaker waker)
	{
		this.waker = waker;
	}

	public void Wake()
	{
		if ((Interlocked.Add(ref state, 2) & 1) != 0)
		{
			release();
			throw new ObjectDisposedException(nameof(Waker));
		}
		try
		{
			Native.Wake(waker);
		}
		finally
		{
			release();
		}
	}

	void release()
	{
		if (Interlocked.Add(ref state, -2) == 1)
		{
			drop();
		}
	}

	void drop()
	{
		if (Interlocked.Exchange(ref freed, 1) == 0)
		{
			Native.Drop(waker);
		}
	}

	/// The C waker of a second owner, for a call that takes one.
	internal Native.CWaker Lower()
	{
		if ((Volatile.Read(ref state) & 1) != 0)
		{
			throw new ObjectDisposedException(nameof(Waker));
		}
		var w = this;
		return Native.Waker(() => w.Wake());
	}

	public void Dispose()
	{
		int n;
		do
		{
			n = Volatile.Read(ref state);
			if ((n & 1) != 0)
			{
				return;
			}
		}
		while (Interlocked.CompareExchange(ref state, n | 1, n) != n);
		GC.SuppressFinalize(this);
		if (n == 0)
		{
			drop();
		}
	}

	~Waker() => drop();
}

/// What a call of a Rust function throws for a `Result`'s error: `Error` holds it, and the
/// message is its `Display` where it has one.
public abstract class RustException : Exception
{
	protected RustException(string message) : base(message)
	{
	}
}

/// Bytes Rust reads and seeks, as its `Read + Seek`: lent for the call it is passed to, which
/// Rust calls only during that call, from any thread. An `IOException` either method throws
/// reaches Rust as an `io::Error` of its message; anything else it throws aborts.
public interface ISeekableReader
{
	/// Reads up to `buffer.Length` bytes into `buffer`, returning how many: 0 only at the end.
	int Read(Span<byte> buffer);

	/// Moves to `offset` from `origin`, returning the position from the start. From
	/// `SeekOrigin.Begin`, `offset` holds a `u64`'s 64 bits, so one past `long.MaxValue` is negative.
	ulong Seek(long offset, SeekOrigin origin);
}

/// A `Stream` as Rust reads and seeks it, which must be readable and seekable.
public sealed class SeekableStream : ISeekableReader
{
	readonly Stream stream;

	public SeekableStream(Stream stream)
	{
		if (!stream.CanRead || !stream.CanSeek)
		{
			throw new ArgumentException("Rust reads and seeks the stream, which must allow both", nameof(stream));
		}
		this.stream = stream;
	}

	public int Read(Span<byte> buffer) => stream.Read(buffer);

	public ulong Seek(long offset, SeekOrigin origin)
	{
		// A `u64` past `long.MaxValue`, which no `Stream` takes.
		if (origin == SeekOrigin.Begin && offset < 0)
		{
			throw new IOException("a seek past the largest position a Stream takes");
		}
		return (ulong)stream.Seek(offset, origin);
	}
}

/// Each item of a Rust stream, as C# iterates one, by its `next`.
internal static class Streams
{
	internal static async IAsyncEnumerator<T> Classes<T>(Func<CancellationToken, ValueTask<T?>> next, CancellationToken token)
		where T : class
	{
		while (await next(token).ConfigureAwait(false) is T item)
		{
			yield return item;
		}
	}

	internal static async IAsyncEnumerator<T> Structs<T>(Func<CancellationToken, ValueTask<T?>> next, CancellationToken token)
		where T : struct
	{
		while (await next(token).ConfigureAwait(false) is T item)
		{
			yield return item;
		}
	}
}

/// What a waker Rust holds wakes.
internal interface IWakeable
{
	void Wake();
}

/// A Rust future C# awaits as a `ValueTask`: polled again on the caller's
/// `SynchronizationContext`, or the thread pool without one, after each wake, never inside it.
internal abstract unsafe class Pending<T> : IValueTaskSource<T>, IValueTaskSource, IWakeable
{
	ManualResetValueTaskSourceCore<T> core;

	/// The C future, until it finishes or is freed.
	IntPtr future;

	/// `Idle`, `Polling`, `Woken`, a poll scheduled, or `Done`.
	int state;
	const int Idle = 0, Polling = 1, Woken = 2, Scheduled = 3, Done = 4;

	/// The waker each poll passes, the same one each time, which Rust drops once.
	Native.CWaker waker;

	/// Whether a poll passed the waker, so Rust drops it rather than this.
	bool given;

	SynchronizationContext? context;
	CancellationTokenRegistration cancel;
	readonly object gate = new object();

	/// Polls the C future once with `waker`, writing its output or error on `READY` or `ERROR`.
	protected abstract int Poll(IntPtr future, Native.CWaker waker, out T value, out Exception? error);

	/// Frees the C future.
	protected abstract void Free(IntPtr future);

	/// Awaits `future`, which its first poll left pending; cancelling `token` frees it.
	internal ValueTask<T> Start(IntPtr future, CancellationToken token)
	{
		Begin(future, token);
		return new ValueTask<T>(this, core.Version);
	}

	/// [`Start`] for a future of `()`.
	internal ValueTask StartVoid(IntPtr future, CancellationToken token)
	{
		Begin(future, token);
		return new ValueTask(this, core.Version);
	}

	void Begin(IntPtr future, CancellationToken token)
	{
		this.future = future;
		core.RunContinuationsAsynchronously = true;
		context = SynchronizationContext.Current;
		waker = new Native.CWaker { wake = Callbacks.PendingWake, drop = Callbacks.PendingDrop, data = Native.Give(this) };
		// Rust's first poll took no waker, so this one polls again at once, with one.
		Run();
		if (token.CanBeCanceled)
		{
			var registration = token.Register(s => ((Pending<T>)s!).Cancel(token), this);
			bool finished;
			lock (gate)
			{
				cancel = registration;
				finished = this.future == IntPtr.Zero;
			}
			// A future that finished meanwhile listens no more.
			if (finished)
			{
				registration.Dispose();
			}
		}
	}

	/// A wake from Rust, on any thread: polls again, after any poll running now.
	public void Wake()
	{
		while (true)
		{
			var s = Volatile.Read(ref state);
			var next = s switch
			{
				Idle => Scheduled,
				Polling => Woken,
				_ => -1,
			};
			if (next < 0)
			{
				return;
			}
			if (Interlocked.CompareExchange(ref state, next, s) == s)
			{
				if (next == Scheduled)
				{
					Schedule();
				}
				return;
			}
		}
	}

	void Schedule()
	{
		if (context != null)
		{
			context.Post(s => ((Pending<T>)s!).Run(), this);
		}
		else
		{
			ThreadPool.UnsafeQueueUserWorkItem(s => ((Pending<T>)s!).Run(), this);
		}
	}

	void Run()
	{
		CancellationTokenRegistration? done = null;
		lock (gate)
		{
			while (true)
			{
				if (future == IntPtr.Zero)
				{
					return;
				}
				Volatile.Write(ref state, Polling);
				int poll;
				T value;
				Exception? error;
				given = true;
				try
				{
					poll = Poll(future, waker, out value, out error);
				}
				catch (Exception e)
				{
					done = Finish();
					core.SetException(e);
					break;
				}
				if (poll == Native.PollPending)
				{
					// A wake during the poll polls again, as it may have come before Rust
					// registered the waker.
					if (Interlocked.CompareExchange(ref state, Idle, Polling) == Polling)
					{
						return;
					}
					continue;
				}
				done = Finish();
				if (poll == Native.PollReady)
				{
					core.SetResult(value);
				}
				else
				{
					core.SetException(error!);
				}
				break;
			}
		}
		// Outside the gate, as disposing waits for a cancel running now, which takes it.
		done?.Dispose();
	}

	/// Frees the finished future, which drops the waker, returning the cancel to stop.
	CancellationTokenRegistration Finish()
	{
		Volatile.Write(ref state, Done);
		var f = future;
		future = IntPtr.Zero;
		Free(f);
		return cancel;
	}

	/// Frees the future unless it finished, which cancels it in Rust and loses nothing.
	void Cancel(CancellationToken token)
	{
		lock (gate)
		{
			if (future == IntPtr.Zero)
			{
				return;
			}
			var f = future;
			future = IntPtr.Zero;
			Volatile.Write(ref state, Done);
			Free(f);
			if (!given)
			{
				Native.Release(waker.data);
			}
			core.SetException(new OperationCanceledException(token));
		}
	}

	public T GetResult(short token) => core.GetResult(token);

	void IValueTaskSource.GetResult(short token) => core.GetResult(token);

	public ValueTaskSourceStatus GetStatus(short token) => core.GetStatus(token);

	public void OnCompleted(Action<object?> continuation, object? state, short token, ValueTaskSourceOnCompletedFlags flags) =>
		core.OnCompleted(continuation, state, token, flags);
}

/// A future C# implements, which Rust polls from any thread until it is done, and frees.
internal sealed unsafe class ForeignFuture
{
	readonly Func<CancellationToken, Task> run;
	readonly CancellationTokenSource cancel = new CancellationTokenSource();
	Task? task;
	Native.CWaker waker;
	bool hasWaker;

	internal ForeignFuture(Func<CancellationToken, Task> run)
	{
		this.run = run;
	}

	internal bool Poll(Native.CWaker w)
	{
		lock (this)
		{
			task ??= Task.Run(() => run(cancel.Token));
			if (hasWaker)
			{
				Native.Drop(waker);
			}
			waker = w;
			hasWaker = true;
			if (task.IsCompleted)
			{
				Check(task);
				return true;
			}
		}
		task.ContinueWith((_, s) => ((ForeignFuture)s!).Woken(), this, TaskScheduler.Default);
		return false;
	}

	void Woken()
	{
		lock (this)
		{
			if (hasWaker)
			{
				Native.Wake(waker);
			}
		}
	}

	static void Check(Task task)
	{
		if (task.IsFaulted)
		{
			Native.Abort("a future C# implements threw", task.Exception);
		}
	}

	internal void Free()
	{
		lock (this)
		{
			cancel.Cancel();
			if (hasWaker)
			{
				hasWaker = false;
				Native.Drop(waker);
			}
		}
	}
}

/// The C functions Rust calls into C#'s runtime through: each an `UnmanagedCallersOnly`
/// method's address on .NET, and under Unity's IL2CPP, which has none, a delegate's, kept
/// alive here, whose method is marked `MonoPInvokeCallback`.
internal static unsafe partial class Callbacks
{
#if UNITY_5_3_OR_NEWER
	internal delegate void DataFn(void* data);
	internal delegate byte FuturePollFn(void* data, Native.CWaker waker);
	static readonly DataFn actionWake = ActionWakeImpl, actionDrop = ActionDropImpl, pendingWake = PendingWakeImpl, pendingDrop = PendingDropImpl, futureFree = FutureFreeImpl, release = ReleaseImpl, commit = CommitImpl;
	static readonly FuturePollFn futurePoll = FuturePollImpl;
	internal static readonly IntPtr ActionWake = Marshal.GetFunctionPointerForDelegate(actionWake);
	internal static readonly IntPtr ActionDrop = Marshal.GetFunctionPointerForDelegate(actionDrop);
	internal static readonly IntPtr PendingWake = Marshal.GetFunctionPointerForDelegate(pendingWake);
	internal static readonly IntPtr PendingDrop = Marshal.GetFunctionPointerForDelegate(pendingDrop);
	internal static readonly IntPtr FuturePoll = Marshal.GetFunctionPointerForDelegate(futurePoll);
	internal static readonly IntPtr FutureFree = Marshal.GetFunctionPointerForDelegate(futureFree);
	internal static readonly IntPtr Release = Marshal.GetFunctionPointerForDelegate(release);
	internal static readonly IntPtr Commit = Marshal.GetFunctionPointerForDelegate(commit);
	internal delegate byte ReadFn(void* data, byte* buf, nuint len, nuint* n, Native.Buf* err);
	internal delegate byte SeekFn(void* data, int whence, long offset, ulong* position, Native.Buf* err);
	static readonly ReadFn readerRead = ReaderReadImpl;
	static readonly SeekFn readerSeek = ReaderSeekImpl;
	internal static readonly IntPtr ReaderRead = Marshal.GetFunctionPointerForDelegate(readerRead);
	internal static readonly IntPtr ReaderSeek = Marshal.GetFunctionPointerForDelegate(readerSeek);
#else
	internal static readonly IntPtr ActionWake = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&ActionWakeImpl;
	internal static readonly IntPtr ActionDrop = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&ActionDropImpl;
	internal static readonly IntPtr PendingWake = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&PendingWakeImpl;
	internal static readonly IntPtr PendingDrop = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&PendingDropImpl;
	internal static readonly IntPtr FuturePoll = (IntPtr)(delegate* unmanaged[Cdecl]<void*, Native.CWaker, byte>)&FuturePollImpl;
	internal static readonly IntPtr FutureFree = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&FutureFreeImpl;
	internal static readonly IntPtr Release = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&ReleaseImpl;
	internal static readonly IntPtr Commit = (IntPtr)(delegate* unmanaged[Cdecl]<void*, void>)&CommitImpl;
	internal static readonly IntPtr ReaderRead = (IntPtr)(delegate* unmanaged[Cdecl]<void*, byte*, nuint, nuint*, Native.Buf*, byte>)&ReaderReadImpl;
	internal static readonly IntPtr ReaderSeek = (IntPtr)(delegate* unmanaged[Cdecl]<void*, int, long, ulong*, Native.Buf*, byte>)&ReaderSeekImpl;
#endif

	/// Reads at most `len` bytes into `buf`, as Rust's `Read::read`; an `IOException` is
	/// Rust's `io::Error` of its message, and any other exception aborts.
#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(ReadFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static byte ReaderReadImpl(void* data, byte* buf, nuint len, nuint* n, Native.Buf* err)
	{
		try
		{
			var span = new Span<byte>(buf, (int)Math.Min(len, (nuint)int.MaxValue));
			var read = Native.Held<ISeekableReader>(data).Read(span);
			if (read < 0 || read > span.Length)
			{
				throw new InvalidOperationException("a read of " + span.Length + " bytes returned " + read);
			}
			*n = (nuint)read;
			return 1;
		}
		catch (IOException e)
		{
			*err = Native.Message(e.Message);
			return 0;
		}
		catch (Exception e)
		{
			Native.Abort("a reader's read: a call into C# threw", e);
			return 0;
		}
	}

	/// Seeks as Rust's `Seek::seek`: `whence` is `SEEK_SET`, whose offset's bits are a `u64`,
	/// `SEEK_CUR`, or `SEEK_END`.
#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(SeekFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static byte ReaderSeekImpl(void* data, int whence, long offset, ulong* position, Native.Buf* err)
	{
		try
		{
			var origin = whence switch
			{
				0 => SeekOrigin.Begin,
				1 => SeekOrigin.Current,
				_ => SeekOrigin.End,
			};
			*position = Native.Held<ISeekableReader>(data).Seek(offset, origin);
			return 1;
		}
		catch (IOException e)
		{
			*err = Native.Message(e.Message);
			return 0;
		}
		catch (Exception e)
		{
			Native.Abort("a reader's seek: a call into C# threw", e);
			return 0;
		}
	}

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void ActionWakeImpl(void* data)
	{
		try
		{
			Native.Held<Action>(data)();
		}
		catch (Exception e)
		{
			Native.Abort("a waker threw", e);
		}
	}

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void ActionDropImpl(void* data) => Native.Release(data);

/// Stores a disposing handle's state closed once Rust's drop can no longer fail, before
	/// the value's `Drop` runs, so a dispose waiting on it returns.
#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void CommitImpl(void* data) => Volatile.Write(ref *(long*)data, 1);

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void ReleaseImpl(void* data)
	{
		var target = GCHandle.FromIntPtr((IntPtr)data).Target;
		Native.Release(data);
		if (target is IDisposable disposable)
		{
			try
			{
				disposable.Dispose();
			}
			catch (Exception e)
			{
				Native.Abort("an implementation's Dispose threw", e);
			}
		}
	}

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void PendingWakeImpl(void* data) => ((IWakeable)GCHandle.FromIntPtr((IntPtr)data).Target!).Wake();

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void PendingDropImpl(void* data) => Native.Release(data);

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(FuturePollFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static byte FuturePollImpl(void* data, Native.CWaker waker)
	{
		try
		{
			return Native.Held<ForeignFuture>(data).Poll(waker) ? (byte)1 : (byte)0;
		}
		catch (Exception e)
		{
			Native.Abort("a future C# implements threw", e);
			return 0;
		}
	}

#if UNITY_5_3_OR_NEWER
	[AOT.MonoPInvokeCallback(typeof(DataFn))]
#else
	[UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
#endif
	static void FutureFreeImpl(void* data)
	{
		Native.Held<ForeignFuture>(data).Free();
		Native.Release(data);
	}
}
