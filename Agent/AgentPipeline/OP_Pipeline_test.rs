use std::future::Future;

use futures::stream;
#[allow(unused_imports)] // Needed since this is used in a macro rule
use futures::try_join;

use super::op::{self};

pub trait TryOp: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Send + Sync;

    /// Execute the current op with the given input.
    fn try_call(
        &self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;

    /// # Example
    /// ```rust
    /// use Qubit::pipeline::{self, TryOp};
    ///
    /// let op = pipeline::new()
    ///    .map(|x: i32| if x % 2 == 0 { Ok(x + 1) } else { Err("x is odd") });
    ///
    /// // Execute the pipeline concurrently with 2 inputs
    /// let result = op.try_batch_call(2, vec![2, 4]).await;
    /// assert_eq!(result, Ok(vec![3, 5]));
    /// ```
    fn try_batch_call<I>(
        &self,
        n: usize,
        input: I,
    ) -> impl Future<Output = Result<Vec<Self::Output>, Self::Error>> + Send
    where
        I: IntoIterator<Item = Self::Input> + Send,
        I::IntoIter: Send,
        Self: Sized,
    {
        use stream::{StreamExt, TryStreamExt};

        async move {
            stream::iter(input)
                .map(|input| self.try_call(input))
                .buffered(n)
                .try_collect()
                .await
        }
    }

    fn map_ok<F, Output>(self, f: F) -> MapOk<Self, op::Map<F, Self::Output>>
    where
        F: Fn(Self::Output) -> Output + Send + Sync,
        Output: Send + Sync,
        Self: Sized,
    {
        MapOk::new(self, op::Map::new(f))
    }

    fn map_err<F, E>(self, f: F) -> MapErr<Self, op::Map<F, Self::Error>>
    where
        F: Fn(Self::Error) -> E + Send + Sync,
        E: Send + Sync,
        Self: Sized,
    {
        MapErr::new(self, op::Map::new(f))
    }

    fn and_then<F, Fut, Output>(self, f: F) -> AndThen<Self, op::Then<F, Self::Output>>
    where
        F: Fn(Self::Output) -> Fut + Send + Sync,
        Fut: Future<Output = Result<Output, Self::Error>> + Send + Sync,
        Output: Send + Sync,
        Self: Sized,
    {
        AndThen::new(self, op::Then::new(f))
    }

    fn or_else<F, Fut, E>(self, f: F) -> OrElse<Self, op::Then<F, Self::Error>>
    where
        F: Fn(Self::Error) -> Fut + Send + Sync,
        Fut: Future<Output = Result<Self::Output, E>> + Send + Sync,
        E: Send + Sync,
        Self: Sized,
    {
        OrElse::new(self, op::Then::new(f))
    }

    fn chain_ok<T>(self, op: T) -> TrySequential<Self, T>
    where
        T: op::Op<Input = Self::Output>,
        Self: Sized,
    {
        TrySequential::new(self, op)
    }
}

impl<Op, T, E> TryOp for Op
where
    Op: super::Op<Output = Result<T, E>>,
    T: Send + Sync,
    E: Send + Sync,
{
    type Input = Op::Input;
    type Output = T;
    type Error = E;

    async fn try_call(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        self.call(input).await
    }
}

pub struct MapOk<Op1, Op2> {
    prev: Op1,
    op: Op2,
}

impl<Op1, Op2> MapOk<Op1, Op2> {
    pub(crate) fn new(prev: Op1, op: Op2) -> Self {
        Self { prev, op }
    }
}

impl<Op1, Op2> op::Op for MapOk<Op1, Op2>
where
    Op1: TryOp,
    Op2: super::Op<Input = Op1::Output>,
{
    type Input = Op1::Input;
    type Output = Result<Op2::Output, Op1::Error>;

    #[inline]
    async fn call(&self, input: Self::Input) -> Self::Output {
        match self.prev.try_call(input).await {
            Ok(output) => Ok(self.op.call(output).await),
            Err(err) => Err(err),
        }
    }
}

pub struct MapErr<Op1, Op2> {
    prev: Op1,
    op: Op2,
}

impl<Op1, Op2> MapErr<Op1, Op2> {
    pub(crate) fn new(prev: Op1, op: Op2) -> Self {
        Self { prev, op }
    }
}

// Result<T, E1> -> Result<T, E2>
impl<Op1, Op2> op::Op for MapErr<Op1, Op2>
where
    Op1: TryOp,
    Op2: super::Op<Input = Op1::Error>,
{
    type Input = Op1::Input;
    type Output = Result<Op1::Output, Op2::Output>;

    #[inline]
    async fn call(&self, input: Self::Input) -> Self::Output {
        match self.prev.try_call(input).await {
            Ok(output) => Ok(output),
            Err(err) => Err(self.op.call(err).await),
        }
    }
}

pub struct AndThen<Op1, Op2> {
    prev: Op1,
    op: Op2,
}

impl<Op1, Op2> AndThen<Op1, Op2> {
    pub(crate) fn new(prev: Op1, op: Op2) -> Self {
        Self { prev, op }
    }
}

impl<Op1, Op2> op::Op for AndThen<Op1, Op2>
where
    Op1: TryOp,
    Op2: TryOp<Input = Op1::Output, Error = Op1::Error>,
{
    type Input = Op1::Input;
    type Output = Result<Op2::Output, Op1::Error>;

    #[inline]
    async fn call(&self, input: Self::Input) -> Self::Output {
        let output = self.prev.try_call(input).await?;
        self.op.try_call(output).await
    }
}

pub struct OrElse<Op1, Op2> {
    prev: Op1,
    op: Op2,
}

impl<Op1, Op2> OrElse<Op1, Op2> {
    pub(crate) fn new(prev: Op1, op: Op2) -> Self {
        Self { prev, op }
    }
}

impl<Op1, Op2> op::Op for OrElse<Op1, Op2>
where
    Op1: TryOp,
    Op2: TryOp<Input = Op1::Error, Output = Op1::Output>,
{
    type Input = Op1::Input;
    type Output = Result<Op1::Output, Op2::Error>;

    #[inline]
    async fn call(&self, input: Self::Input) -> Self::Output {
        match self.prev.try_call(input).await {
            Ok(output) => Ok(output),
            Err(err) => self.op.try_call(err).await,
        }
    }
}

pub struct TrySequential<Op1, Op2> {
    prev: Op1,
    op: Op2,
}

impl<Op1, Op2> TrySequential<Op1, Op2> {
    pub(crate) fn new(prev: Op1, op: Op2) -> Self {
        Self { prev, op }
    }
}

impl<Op1, Op2> op::Op for TrySequential<Op1, Op2>
where
    Op1: TryOp,
    Op2: op::Op<Input = Op1::Output>,
{
    type Input = Op1::Input;
    type Output = Result<Op2::Output, Op1::Error>;

    #[inline]
    async fn call(&self, input: Self::Input) -> Self::Output {
        match self.prev.try_call(input).await {
            Ok(output) => Ok(self.op.call(output).await),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::op::{map, then};

    #[tokio::test]
    async fn test_try_op() {
        let op = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") });
        let result = op.try_call(2).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_map_ok_constructor() {
        let op1 = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") });
        let op2 = then(|x: i32| async move { x * 2 });
        let op3 = map(|x: i32| x - 1);

        let pipeline = MapOk::new(MapOk::new(op1, op2), op3);

        let result = pipeline.try_call(2).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_map_ok_chain() {
        let pipeline = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") })
            .map_ok(|x| x * 2)
            .map_ok(|x| x - 1);

        let result = pipeline.try_call(2).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_map_err_constructor() {
        let op1 = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") });
        let op2 = then(|err: &str| async move { format!("Error: {}", err) });
        let op3 = map(|err: String| err.len());

        let pipeline = MapErr::new(MapErr::new(op1, op2), op3);

        let result = pipeline.try_call(1).await;
        assert_eq!(result, Err(15));
    }

    #[tokio::test]
    async fn test_map_err_chain() {
        let pipeline = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") })
            .map_err(|err| format!("Error: {}", err))
            .map_err(|err| err.len());

        let result = pipeline.try_call(1).await;
        assert_eq!(result, Err(15));
    }

    #[tokio::test]
    async fn test_and_then_constructor() {
        let op1 = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") });
        let op2 = then(|x: i32| async move { Ok(x * 2) });
        let op3 = map(|x: i32| Ok(x - 1));

        let pipeline = AndThen::new(AndThen::new(op1, op2), op3);

        let result = pipeline.try_call(2).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_and_then_chain() {
        let pipeline = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") })
            .and_then(|x| async move { Ok(x * 2) })
            .and_then(|x| async move { Ok(x - 1) });

        let result = pipeline.try_call(2).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_or_else_constructor() {
        let op1 = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") });
        let op2 = then(|err: &str| async move { Err(format!("Error: {}", err)) });
        let op3 = map(|err: String| Ok::<i32, String>(err.len() as i32));

        let pipeline = OrElse::new(OrElse::new(op1, op2), op3);

        let result = pipeline.try_call(1).await.unwrap();
        assert_eq!(result, 15);
    }

    #[tokio::test]
    async fn test_or_else_chain() {
        let pipeline = map(|x: i32| if x % 2 == 0 { Ok(x) } else { Err("x is odd") })
            .or_else(|err| async move { Err(format!("Error: {}", err)) })
            .or_else(|err| async move { Ok::<i32, String>(err.len() as i32) });

        let result = pipeline.try_call(1).await.unwrap();
        assert_eq!(result, 15);
    }
}
