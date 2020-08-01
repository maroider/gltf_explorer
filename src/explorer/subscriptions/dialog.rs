use std::{fmt, sync::Arc};

use native_dialog::AsyncDialog;

use iced_native::futures;

pub fn dialog<D>(dialog: D) -> iced_native::Subscription<Result<D::Output>>
where
    D: native_dialog::Dialog + Send + Sync + 'static,
    D::Output: Send + Sync + 'static,
{
    DialogRecipe::new(dialog.create_async()).into_subscription()
}

pub struct DialogRecipe<O> {
    dialog: AsyncDialog<native_dialog::Result<O>>,
}

impl<O> DialogRecipe<O>
where
    O: Send + Sync + 'static,
{
    pub fn new(dialog: AsyncDialog<native_dialog::Result<O>>) -> Self {
        Self { dialog }
    }

    pub fn into_subscription(self) -> iced_native::Subscription<Result<O>> {
        iced_native::Subscription::from_recipe(self)
    }
}

impl<H, I, O> iced_native::subscription::Recipe<H, I> for DialogRecipe<O>
where
    H: std::hash::Hasher,
    O: Send + Sync + 'static,
{
    type Output = Result<O>;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Initial(self.dialog),
            |state| async move {
                match state {
                    State::Initial(dialog) => {
                        Some((dialog.await.map_err(Into::into), State::Finished))
                    }
                    State::Finished => None,
                }
            },
        ))
    }
}

pub type Result<T> = std::result::Result<T, DialogError>;

enum State<O> {
    Initial(AsyncDialog<native_dialog::Result<O>>),
    Finished,
}

#[derive(Clone, Debug)]
pub struct DialogError {
    inner: Arc<native_dialog::Error>,
}

impl fmt::Display for DialogError {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<native_dialog::Error> for DialogError {
    fn from(from: native_dialog::Error) -> Self {
        Self {
            inner: Arc::new(from),
        }
    }
}
