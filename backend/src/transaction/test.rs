use super::*;
use super::super::{setup, teardown};

fn get_transaction() -> Transaction {
	return Transaction::default()
		.set_currency_id(0)
		.set_comment("this is a comment".to_string())
		.set_positions(vec![Position {amount: 12345, ..Default::default()}]);
}

mod add {
	use super::*;
	use super::super::super::tag;

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		get_transaction().save(&pool).await?;
		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

		get_transaction()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;

		teardown(&config).await;
		return Ok(());
	}
}

mod get_all {
	use super::*;
	use super::super::super::tag;

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_no_results_on_empty_db() {
		let (config, pool) = setup().await;

		let res = TransactionLoader::new(&pool).get().await.unwrap();
		assert_eq!(res.len(), 0);

		teardown(&config).await;
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		get_transaction().save(&pool).await?;
		get_transaction().save(&pool).await?;
		get_transaction().save(&pool).await?;

		let res = TransactionLoader::new(&pool).get().await?;
		assert_eq!(res.len(), 3);

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

		get_transaction()
			.set_tag_ids(vec![0])
			.save(&pool).await?;

		let res = TransactionLoader::new(&pool).get().await?;
		assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0]);

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_multiple_tags_correctly() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
		tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

		get_transaction()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;
		get_transaction()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;

		let res = TransactionLoader::new(&pool).get().await?;
		assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0, 1, 2]);

		teardown(&config).await;
		return Ok(());
	}
}