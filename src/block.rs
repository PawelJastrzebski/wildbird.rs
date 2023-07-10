///  Blocks async fn in non async environment
/// > ⚠️ Calling from the main asynchronous thread will freeze the application
///
/// # Example
/// ```
/// use wildbird::prelude::*;
///
/// async fn fetch_from_api() -> String {
///     std::thread::sleep(std::time::Duration::from_millis(100));
///     String::from("Api respone")
/// }
///
/// let res = fetch_from_api().block();
/// ```
pub trait Block<F>
where
    F: std::future::IntoFuture,
{
    fn block(self) -> F::Output;
}

impl<F> Block<F> for F
where
    F: std::future::IntoFuture,
{
    fn block(self) -> F::Output {
        crate::private::block(self.into_future())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::time::Duration;

    async fn fetch_from_api() -> String {
        std::thread::sleep(Duration::from_millis(100));
        String::from("Api respone")
    }

    #[test]
    fn should_block() {
        println!("Res: {}", fetch_from_api().block())
    }

    #[tokio::test]
    async fn should_block_async() {
        println!("Res: {}", fetch_from_api().block())
    }

    #[test]
    fn should_block_async_block() {
        let action = async {
            let data1 = fetch_from_api().await;
            let data2 = fetch_from_api().await;
            format!("{data1}, {data2}")
        };

        println!("Data: {}", action.block())
    }
}
