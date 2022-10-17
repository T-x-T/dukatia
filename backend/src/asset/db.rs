use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Asset;

pub async fn add(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	if asset.amount.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("amount"), item_type: String::from("asset") }))
	}
	if asset.value_per_unit.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("value_per_unit"), item_type: String::from("asset") }))
	}

	let client = pool.get().await?;

	let id: i32 = client.query(
			"INSERT INTO public.\"Assets\" (id, name, description, \"userId\", \"currencyId\") VALUES (DEFAULT, $1, $2, $3, $4) RETURNING id;", 
			&[&asset.name, &asset.description, &(asset.user_id as i32), &(asset.currency_id as i32)]
		).await?
		[0].get(0);

	if asset.tag_ids.is_some() {
		for tag_id in asset.tag_ids.clone().unwrap() {
			client.query(
					"INSERT INTO public.\"AssetTags\" (\"assetId\", \"tagId\") VALUES ($1, $2);",
					&[&id, &(tag_id as i32)]
				).await?;
		}
	}

	client.query(
		"INSERT INTO public.\"AssetAmounts\" (\"assetId\", timestamp, amount) VALUES ($1, $2, $3);",
		&[&(id as i32), &chrono::Local::now(), &asset.amount]
	).await?;

	client.query(
		"INSERT INTO public.\"AssetValuations\" (\"assetId\", timestamp, \"valuePerUnit\") VALUES ($1, $2, $3)", 
		&[&(id as i32), &chrono::Local::now(), &(asset.value_per_unit.unwrap() as i32)]
	).await?;

	return Ok(());
}


pub async fn get_all(pool: &Pool) -> Result<Vec<Asset>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.\"AssetData\";", &[])
		.await?;
	
	if rows.is_empty() {
		return Err(Box::new(CustomError::NoItemFound { item_type: String::from("asset") }));
	}

	return Ok(rows.into_iter().map(|x| turn_row_into_asset(&x)).collect());
}

pub async fn get_by_id(pool: &Pool, asset_id: u32) -> Result<Asset, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.\"AssetData\" WHERE id=$1;", &[&(asset_id as i32)])
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::NoItemFound { item_type: String::from("asset") }));
	}

	return Ok(turn_row_into_asset(&rows[0]));
}

pub async fn update(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	if asset.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("asset") }));
	}

	get_by_id(&pool, asset.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.\"Assets\" SET name=$1, description=$2 WHERE id=$3",
		&[&asset.name, &asset.description, &(asset.id.unwrap() as i32)]
	).await?;

	client.query(
		"DELETE FROM public.\"AssetTags\" WHERE \"assetId\"=$1", 
		&[&(asset.id.unwrap() as i32)]
	).await?;

	if asset.tag_ids.is_some() {
		for tag_id in asset.tag_ids.clone().unwrap() {
			client.query(
					"INSERT INTO public.\"AssetTags\" (\"assetId\", \"tagId\") VALUES ($1, $2);",
					&[&(asset.id.unwrap() as i32), &(tag_id as i32)]
				).await?;
		}
	}

	if asset.amount.is_some() {
		client.query(
			"INSERT INTO public.\"AssetAmounts\" (\"assetId\", timestamp, amount) VALUES ($1, $2, $3);",
			&[&(asset.id.unwrap() as i32), &chrono::Local::now(), &asset.amount]
		).await?;
	}
	
	if asset.value_per_unit.is_some() {
		client.query(
			"INSERT INTO public.\"AssetValuations\" (\"assetId\", timestamp, \"valuePerUnit\") VALUES ($1, $2, $3)", 
			&[&(asset.id.unwrap() as i32), &chrono::Local::now(), &(asset.value_per_unit.unwrap() as i32)]
		).await?;
	}

	return Ok(());
}

pub async fn delete_by_id(pool: &Pool, asset_id: u32) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"DELETE FROM public.\"Assets\" WHERE id=$1;", 
			&[&(asset_id as i32)]
		).await?;

	return Ok(());
}

fn turn_row_into_asset(row: &tokio_postgres::Row) -> Asset {
	let id: i32 = row.get(0);
	let name: String = row.get(1);
	let description: Option<String> = row.get(2);
	let user_id: i32 = row.get(3);
	let currency_id: i32 = row.get(4);
	let tag_ids: Vec<u32> = row.try_get(5)
		.unwrap_or(Vec::new())
		.into_iter()
		.map(|x: i32| x as u32)
		.collect();
	let amount: f64 = row.get(6);
	let value_per_unit: i32 = row.get(7);

	return Asset {
		id: Some(id as u32),
		name,
		description,
		user_id: user_id as u32,
		currency_id: currency_id as u32,
		tag_ids: Some(tag_ids),
		value_per_unit: Some(value_per_unit as u32),
		amount: Some(amount),
	}
}