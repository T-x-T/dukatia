use super::*;
use super::super::{setup, teardown};

#[tokio::test(flavor = "multi_thread")]
async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
	let (config, pool) = setup().await;

	CurrencyLoader::new(&pool).get().await?;

	teardown(&config).await;
	return Ok(());
}

#[tokio::test(flavor = "multi_thread")]
async fn returns_two_rows() -> Result<(), Box<dyn Error>> {
	let (config, pool) = setup().await;

	let res = CurrencyLoader::new(&pool).get().await?;
	assert_eq!(res.len(), 2);
	
	teardown(&config).await;
	return Ok(());
}