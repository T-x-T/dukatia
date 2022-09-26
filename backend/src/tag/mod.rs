mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use super::CustomError;

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
	pub id: Option<u32>,
	pub name: String,
	pub user_id: u32,
	pub parent_id: Option<u32>,
}

pub async fn add(pool: &Pool, tag: &Tag) -> Result<(), Box<dyn Error>> {
	if tag.parent_id.is_some() && !is_valid_parent(&pool, tag.parent_id.unwrap(), None).await {
		return Err(Box::new(CustomError::InvalidItem{reason: String::from("it doesn't exist or because it would create a cyclic relationship")}));
	}
	return db::add(&pool, &tag).await;
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Tag>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn update(pool: &Pool, tag: &Tag) -> Result<(), Box<dyn Error>> {
	return db::update(&pool, &tag).await;
}

pub async fn delete(pool: &Pool, tag_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete(&pool, tag_id).await;
}

//If tag_id is supplied check if parent_id can be parent of tag (checks cyclic dependency)
async fn is_valid_parent(pool: &Pool, parent_id: u32, tag_id: Option<u32>) -> bool {
	if db::get_by_id(&pool, parent_id).await.is_err() {
		return false;
	}

	if tag_id.is_none() {
		return true;
	}

	//Check cyclic dependency
	let mut next_parent_id_to_check = parent_id;
	loop {
		if next_parent_id_to_check == tag_id.unwrap() {
			return false;
		}
		let next_tag = db::get_by_id(&pool, next_parent_id_to_check).await;
		if next_tag.is_err() {
			break;
		}
		next_parent_id_to_check = next_tag.unwrap().id.unwrap();
	}

	return true;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	fn get_tag() -> Tag {
		return Tag {
			id: None,
			name: String::from("test_tag"),
			user_id: 0,
			parent_id: None,
		};
	}

	mod add {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_without_parent() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			add(&pool, &get_tag()).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_existing_parent() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let mut tag = get_tag();

			add(&pool, &tag).await?;
			add(&pool, &tag).await?;
			tag.parent_id = Some(1);
			add(&pool, &tag).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		#[should_panic]
		async fn panic_with_non_existing_parent() {
			let (config, pool) = setup().await;

			let mut tag = get_tag();
			tag.parent_id = Some(1);
			add(&pool, &tag).await.unwrap();

			teardown(&config).await;
		}
	}

	mod get_all {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_default_db() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_all(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let tag = get_tag();
			add(&pool, &tag).await?;
			add(&pool, &tag).await?;
			add(&pool, &tag).await?;

			get_all(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}
		
		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_parent_id() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let mut tag = get_tag();
			add(&pool, &tag).await?;
			add(&pool, &tag).await?;

			tag.parent_id = Some(1);
			add(&pool, &tag).await?;

			get_all(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let tag = get_tag();
			add(&pool, &tag).await?;
			add(&pool, &tag).await?;
			add(&pool, &tag).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res.len(), 3);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_correct_parent_id_with_some() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let mut tag = get_tag();
			add(&pool, &tag).await?;

			tag.parent_id = Some(0);
			add(&pool, &tag).await?;
			let res = get_all(&pool).await?;
			assert_eq!(res[1].parent_id.unwrap(), 0);
			
			tag.parent_id = Some(1);
			add(&pool, &tag).await?;
			let res = get_all(&pool).await?;
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

			let mut tag = get_tag();
			add(&pool, &tag).await?;

			tag.id = Some(0);
			update(&pool, &tag).await?;

			teardown(&config).await;
			return Ok(());
		}
		
		#[tokio::test(flavor = "multi_thread")]
		async fn returns_error_without_id() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let tag = get_tag();
			add(&pool, &tag).await?;

			let res = update(&pool, &tag).await;
			
			teardown(&config).await;
			match res {
				Ok(_) => panic!("this should have returned an error, but didnt"),
				Err(_) => return Ok(())
			};
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_error_when_specified_tag_doesnt_exist() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let mut tag = get_tag();
			add(&pool, &tag).await?;

			tag.id = Some(1);
			let res = update(&pool, &tag).await;
			
			teardown(&config).await;
			match res {
				Ok(_) => panic!("this should have returned an error, but didnt"),
				Err(_) => return Ok(())
			};
		}
	}
}