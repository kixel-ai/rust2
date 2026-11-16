use std::{fmt::Debug, future::Future, pin::Pin};

use tokio::sync::mpsc;
use tracing::warn;

/// Given multiple futures with output = `Result<Option<T>, E>`,
/// returns the the first one that returns either `Err(_)` or
/// `Ok(Some(_))`.
pub struct FuturesResolver<T, E> {
	rx: mpsc::Receiver<Result<T, E>>,
	tx: mpsc::Sender<Result<T, E>>,
}

impl<T, E> Default for FuturesResolver<T, E> {
	fn default() -> Self {
		// We only need the first one, so the channel is of size 1.
		let (tx, rx) = mpsc::channel(1);
		Self { tx, rx }
	}
}

impl<T: Send + 'static, E: Send + Debug + 'static> FuturesResolver<T, E> {
	/// Insert new future into this resolver, they will start running
	/// right away.
	pub fn push<Fut>(&self, fut: Fut)
	where
		Fut: Future<Output = Result<Option<T>, E>> + Send + 'static,
	{
		let tx = self.tx.clone();

		tokio::spawn(async move {
			tokio::pin!(fut);

			Self::spawn_inner(fut, tx).await;
		});
	}

	async fn spawn_inner(
		fut: Pin<&mut (dyn Future<Output = Result<Option<T>, E>> + Send)>,
		tx: mpsc::Sender<Result<T, E>>,
	) {
		let res = tokio::select! {
			biased;

			_ = tx.closed() => return,
			res = fut => res,
		};

		if let Some(res) = res.transpose() {
			// An error can occupy the slot before the sole successful candidate.
			tx.send(res).await.ok();
		}
	}

	/// Insert multiple futures into this resolver, they will start running
	/// right away.
	pub fn extend<Fut, Iter>(&self, iter: Iter)
	where
		Fut: Future<Output = Result<Option<T>, E>> + Send + 'static,
		Iter: IntoIterator<Item = Fut>,
	{
		iter.into_iter().for_each(|fut| self.push(fut));
	}

	/// Return the resolution.
	pub fn resolve(self) -> impl Future<Output = Option<T>> {
		let mut rx = self.rx;
		drop(self.tx);

		async move {
			loop {
				match rx.recv().await {
					Some(Ok(ret)) => return Some(ret),
					Some(Err(err)) => warn!(?err, "Fail to resolve the future"),
					None => return None,
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn success_survives_a_queued_error() {
		crate::network::runtime().unwrap().block_on(async {
			let mut resolver = FuturesResolver::<u8, &str>::default();
			resolver.tx.send(Err("candidate failed")).await.unwrap();
			let future = async { Ok(Some(42)) };
			tokio::pin!(future);
			let send = FuturesResolver::spawn_inner(future, resolver.tx.clone());
			tokio::pin!(send);
			// Poll while the error occupies the sole slot, then free it explicitly.
			assert!(std::future::poll_fn(|cx| std::task::Poll::Ready(send.as_mut().poll(cx).is_pending())).await);
			assert_eq!(resolver.rx.recv().await, Some(Err("candidate failed")));
			send.await;
			assert_eq!(resolver.resolve().await, Some(42));
		});
	}
}
