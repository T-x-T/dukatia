use super::*;
use super::super::{setup, teardown};
use crate::tag::Tag;

fn get_account() -> Account {
	return Account::default()
		.set_name("test".to_string());
}

mod add {
	use super::*;

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		get_account().save(&pool).await?;

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;
		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;
		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;

		get_account()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;

		teardown(&config).await;
		return Ok(());
	}
}

mod get_all {
	use super::*;

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic_on_default_db() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		AccountLoader::new(&pool).get().await?;
		
		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;

		get_account().save(&pool).await?;
		get_account().save(&pool).await?;
		get_account().save(&pool).await?;

		let res = AccountLoader::new(&pool).get().await?;
		assert_eq!(res.len(), 4);

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;

		get_account()
		.set_tag_ids(vec![0])
		.save(&pool).await?;

		let res = AccountLoader::new(&pool).get().await?;
		assert_eq!(res[1].clone().tag_ids.unwrap(), vec![0]);

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_multiple_tags_correctly() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;
		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;
		Tag::default().set_name("test_tag".to_string()).save(&pool).await?;

		get_account()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;
		get_account()
			.set_tag_ids(vec![0, 1, 2])
			.save(&pool).await?;

			let res = AccountLoader::new(&pool).get().await?;
		assert_eq!(res[1].clone().tag_ids.unwrap(), vec![0, 1, 2]);

		teardown(&config).await;
		return Ok(());
	}
}