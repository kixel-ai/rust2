//! Futures and polls whose `Pending`s JS can't tell apart.

/// `Ready(Some(Ready(Pending)))` would be `rs.OuterPending`, as the outer `Pending` is.
pub fn nested_poll(p: std::task::Poll<Option<std::task::Poll<std::task::Poll<u8>>>>) -> bool {
	matches!(p, std::task::Poll::Pending)
}

/// A future's `Ready(Pending)` would be `rs.Pending`, as its own pending is.
pub async fn pending_output() -> std::task::Poll<u8> {
	std::task::Poll::Pending
}

/// A census lowers this over `F` and `O`, and `awaited`'s body with it, whose closure names
/// `O` as this function's second type parameter: that refuses nothing.
pub async fn generic_awaited<F: Fn() -> u8, O: std::future::Future<Output = u8>>(f: F, o: O) -> u8 {
	f() + awaited(o).await
}

async fn awaited<O: std::future::Future<Output = u8>>(o: O) -> u8 {
	let mut o = std::pin::pin!(o);
	std::future::poll_fn(|cx| poll_pinned(o.as_mut(), cx)).await
}

fn poll_pinned<O: std::future::Future>(
	o: std::pin::Pin<&mut O>,
	cx: &mut std::task::Context<'_>,
) -> std::task::Poll<O::Output> {
	o.poll(cx)
}

/// A `Ready`'s `Ready(Pending)` would be `rs.Pending`, as its own poll's pending is.
pub fn ready_pending() -> std::future::Ready<std::task::Poll<u8>> {
	std::future::ready(std::task::Poll::Pending)
}

/// What a waker writes is its data's and vtable's addresses.
pub fn waker_debug(w: &std::task::Waker) -> String {
	format!("{w:?}")
}

/// What a raw waker writes is its data's and vtable's addresses too.
pub fn raw_waker_debug(w: &std::task::Waker) -> String {
	format!("{:?}", std::task::RawWaker::new(w.data(), w.vtable()))
}
