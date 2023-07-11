use super::*;
	use super::super::{setup, teardown};

	fn get_tag() -> Tag {
		return Tag::default()
			.set_name("test_tag".to_string());
	}

	mod add {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_without_parent() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_existing_parent() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;
			get_tag().set_parent_id(1).save(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		#[should_panic]
		async fn panic_with_non_existing_parent() {
			let (config, pool) = setup().await;

			get_tag()
				.set_parent_id(1)
				.save(&pool)
				.await
				.unwrap();

			teardown(&config).await;
		}
	}

	mod get_all {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_default_db() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			TagLoader::new(&pool).get().await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;

			TagLoader::new(&pool).get().await?;

			teardown(&config).await;
			return Ok(());
		}
		
		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_parent_id() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;
			get_tag().set_parent_id(1).save(&pool).await?;

			TagLoader::new(&pool).get().await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;

			let res = TagLoader::new(&pool).get().await?;
			assert_eq!(res.len(), 3);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_correct_parent_id_with_some() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().set_parent_id(0).save(&pool).await?;

			let res = TagLoader::new(&pool).get().await?;
			assert_eq!(res[1].parent_id.unwrap(), 0);
			
			get_tag().set_parent_id(1).save(&pool).await?;
			let res = TagLoader::new(&pool).get().await?;
			assert_eq!(res[2].parent_id.unwrap(), 1);

			teardown(&config).await;
			return Ok(());
		}
	}

	mod update {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().set_id(0).save(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_error_when_specified_tag_doesnt_exist() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			let res = get_tag().set_id(1).save(&pool).await;
			
			teardown(&config).await;
			match res {
				Ok(_) => panic!("this should have returned an error, but didnt"),
				Err(_) => return Ok(())
			};
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_error_when_trying_to_create_cyclic_relationship() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_tag().save(&pool).await?;
			get_tag().save(&pool).await?;
			get_tag().set_id(0).set_parent_id(1).save(&pool).await?;
			let res = get_tag().set_id(1).set_parent_id(0).save(&pool).await;
			
			teardown(&config).await;
			match res {
				Ok(_) => panic!("this should have returned an error, but didnt"),
				Err(_) => return Ok(())
			};
		}
	}