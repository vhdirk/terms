use std::{sync::{Mutex, Arc}, future::Future, task::{Context, Poll}, pin::Pin};

#[derive(Debug, PartialEq)]
pub enum ExecState {
    RunOnce,
    Waiting,
}

pub struct FutureWithReturn<T> {
    pub item: std::rc::Rc<std::cell::RefCell<Option<T>>>,
}

impl<T> Unpin for FutureWithReturn<T> {}

impl<T> Future for FutureWithReturn<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let context: &mut ExecState = unsafe { std::mem::transmute(context) };

        if *context == ExecState::Waiting {
            Poll::Pending
        } else if let Some(item) = self.item.borrow_mut().take() {
            *context = ExecState::Waiting;
            Poll::Ready(item)
        } else {
            Poll::Pending
        }
    }
}

/// returns true if future is done
pub fn resume<'a>(future: &mut Pin<Box<dyn Future<Output = ()> + 'a>>) -> bool {
    let mut futures_context = ExecState::RunOnce;
    let futures_context_ref: &mut _ = unsafe { std::mem::transmute(&mut futures_context) };

    matches!(future.as_mut().poll(futures_context_ref), Poll::Ready(_))
}


/// An adapter between callbacks and futures.
///
/// Allows wrapping asynchronous API with callbacks into futures.
/// Calls loader upon first `Future::poll` call; stores result and wakes upon getting callback.
pub struct CallbackFuture<T> {
    loader: Option<Box<dyn FnOnce(Box<dyn FnOnce(T) + Send + 'static>) + Send + 'static>>,
    result: Arc<Mutex<Option<T>>>,
}

impl<T> CallbackFuture<T> {
    /// Creates a new CallbackFuture
    ///
    /// # Examples
    /// ```
    /// use callback_future::CallbackFuture;
    /// use futures::executor::block_on;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let future = CallbackFuture::new(|complete| {
    ///     // make call with callback here, call `complete` upon callback reception, e.g.:
    ///     thread::spawn(move || {
    ///         complete("Test");
    ///     });
    /// });
    /// assert_eq!(block_on(future), "Test");
    /// ```
    pub fn new(loader: impl FnOnce(Box<dyn FnOnce(T) + Send + 'static>) + Send + 'static)
               -> CallbackFuture<T> {
        CallbackFuture {
            loader: Some(Box::new(loader)),
            result: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a ready CallbackFuture
    ///
    /// # Examples
    /// ```
    /// use callback_future::CallbackFuture;
    /// use futures::executor::block_on;
    ///
    /// assert_eq!(block_on(CallbackFuture::ready("Test")), "Test");
    /// ```
    pub fn ready(value: T) -> CallbackFuture<T> {
        CallbackFuture {
            loader: None,
            result: Arc::new(Mutex::new(Some(value))),
        }
    }
}

impl<T: Send + 'static> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = self.get_mut();
        match self_mut.loader.take() {
            // in case loader is still present, loader was not yet invoked: invoke it
            Some(loader) => {
                let waker = cx.waker().clone();
                let result = self_mut.result.clone();
                loader(Box::new(move |value| {
                    *result.lock().unwrap() = Some(value);
                    waker.wake();
                }));
                Poll::Pending
            }
            // in case loader was moved-out: either result is already ready,
            // or we haven't yet received callback
            None => {
                match self_mut.result.lock().unwrap().take() {
                    Some(value) => Poll::Ready(value),
                    None => Poll::Pending, // we haven't received callback yet
                }
            }
        }
    }
}
