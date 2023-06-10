use deadpool_postgres::Pool;
use chrono::{Utc, Date};
use std::collections::BTreeMap;
use std::error::Error;
use super::super::CustomError;
use super::{Asset, AssetValuation, DeepAsset};

pub async fn add(pool: &Pool, asset: &Asset) -> Result<u32, Box<dyn Error>> {
	let client = pool.get().await?;

	let id: i32 = client.query(
			"INSERT INTO public.assets (id, name, description, user_id, currency_id) VALUES (DEFAULT, $1, $2, $3, $4) RETURNING id;", 
			&[&asset.name, &asset.description, &(asset.user_id as i32), &(asset.currency_id as i32)]
		).await?
		[0].get(0);

	if asset.tag_ids.is_some() {
		for tag_id in asset.tag_ids.clone().unwrap() {
			client.query(
					"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
					&[&id, &(tag_id as i32)]
				).await?;
		}
	}

	return Ok(id as u32);
}


pub async fn get_all(pool: &Pool) -> Result<Vec<Asset>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.asset_data;", &[])
		.await?;
	
	return Ok(rows.into_iter().map(|x| turn_row_into_asset(&x)).collect());
}

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepAsset>, Box<dyn Error>> {
	return Ok(
		pool.get()
			.await?
			.query("SELECT * FROM deep_assets", &[])
			.await?
			.iter()
			.map(|x| turn_row_into_deep_asset(x))
			.collect()
	);
}

pub async fn get_all_from_user(pool: &Pool, user_id: u32) -> Result<Vec<Asset>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.asset_data WHERE user_id=$1;", &[&(user_id as i32)])
		.await?;

	return Ok(rows.into_iter().map(|x| turn_row_into_asset(&x)).collect());
}

pub async fn get_by_id(pool: &Pool, asset_id: u32) -> Result<Asset, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.asset_data WHERE id=$1;", &[&(asset_id as i32)])
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("asset"), filter: format!("id={asset_id}") } ));
	}

	return Ok(turn_row_into_asset(&rows[0]));
}

#[allow(unused)]
pub async fn get_amount_at_day(pool: &Pool, asset_id: u32, date: Date<Utc>) -> Result<f64, Box<dyn Error>> {
	let res = pool.get()
		.await?
		.query(
			"SELECT * FROM public.asset_amounts WHERE asset_id = $1 AND timestamp < $2 ORDER BY timestamp DESC LIMIT 1;",
			&[&(asset_id as i32), &(date.and_time(chrono::NaiveTime::from_num_seconds_from_midnight(0, 0)))] 
		).await?;

	if res.len() == 0 {
		return Err(Box::new(CustomError::NoItemFound { item_type: String::from("asset") }));
	}

	return Ok(res[0].try_get(2).unwrap_or(0.0));
}

#[allow(unused)]
pub async fn get_value_at_day(pool: &Pool, asset_id: u32, date: Date<Utc>) -> Result<i32, Box<dyn Error>> {
	let res = pool.get()
		.await?
		.query(
			"SELECT * FROM public.asset_valuations WHERE asset_id = $1 AND timestamp < $2 ORDER BY timestamp DESC LIMIT 1;",
			&[&(asset_id as i32), &(date.and_time(chrono::NaiveTime::from_num_seconds_from_midnight(0, 0)))] 
		).await?;

	if res.len() == 0 {
		return Err(Box::new(CustomError::NoItemFound { item_type: String::from("asset") }));
	}

	return Ok(res[0].try_get(2).unwrap_or(0));
}

pub async fn get_value_per_unit_history(pool: &Pool, asset_id: u32) -> Result<BTreeMap<chrono::DateTime<chrono::Utc>, u32>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT \"timestamp\", value_per_unit	FROM public.asset_valuations WHERE asset_id=$1;",
			&[&(asset_id as i32)]
		).await?;

		let mut output: BTreeMap<chrono::DateTime<chrono::Utc>, u32> = BTreeMap::new();

		rows.into_iter().for_each(|x| {
			let timestamp: chrono::DateTime<chrono::Utc> = x.get(0);
			let value_per_unit: i32 = x.get(1);
			output.insert(timestamp, value_per_unit as u32);
		});

		return Ok(output);
}

pub async fn get_amount_history(pool: &Pool, asset_id: u32) -> Result<BTreeMap<chrono::DateTime<chrono::Utc>, f64>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT timestamp, amount FROM public.asset_amounts WHERE asset_id=$1;",
			&[&(asset_id as i32)]
		).await?;

		let mut output: BTreeMap<chrono::DateTime<chrono::Utc>, f64> = BTreeMap::new();

		rows.into_iter().for_each(|x| {
			let timestamp: chrono::DateTime<chrono::Utc> = x.get(0);
			let value_per_unit: f64 = x.get(1);
			output.insert(timestamp, value_per_unit);
		});

		return Ok(output);
}

pub async fn update(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	if asset.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("asset") }));
	}

	get_by_id(&pool, asset.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.assets SET name=$1, description=$2 WHERE id=$3",
		&[&asset.name, &asset.description, &(asset.id.unwrap() as i32)]
	).await?;

	client.query(
		"DELETE FROM public.asset_tags WHERE asset_id=$1", 
		&[&(asset.id.unwrap() as i32)]
	).await?;

	if asset.tag_ids.is_some() {
		for tag_id in asset.tag_ids.clone().unwrap() {
			client.query(
					"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
					&[&(asset.id.unwrap() as i32), &(tag_id as i32)]
				).await?;
		}
	}

	return Ok(());
}

pub async fn get_valuation_history_by_asset_id(pool: &Pool, asset_id: u32) -> Result<Vec<AssetValuation>, Box<dyn Error>> {
	get_by_id(&pool, asset_id).await?;

	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.asset_valuation_history WHERE asset_id=$1",
		&[&(asset_id as i32)]
	).await?;

	return Ok(rows.into_iter().map(|x| turn_row_into_asset_valuation(&x)).collect());
}

pub async fn replace_valuation_history_of_asset(pool: &Pool, asset_id: u32, asset_valuations: Vec<AssetValuation>) -> Result<(), Box<dyn Error>> {
	get_by_id(&pool, asset_id).await?;
	
	let client = pool.get().await?;

	client.query(
		"DELETE FROM public.asset_amounts WHERE asset_id=$1",
		&[&(asset_id as i32)]
	).await?;

	client.query(
		"DELETE FROM public.asset_valuations WHERE asset_id=$1",
		&[&(asset_id as i32)]
	).await?;

	for asset_valuation in asset_valuations {
		add_valuation(&pool, asset_id, &asset_valuation).await?;
	}

	return Ok(());
}

pub async fn add_valuation(pool: &Pool, asset_id: u32, asset_valuation: &AssetValuation) -> Result<(), Box<dyn Error>> {
	get_by_id(&pool, asset_id).await?;
	
	let client = pool.get().await?;
	
	client.query(
		"INSERT INTO public.asset_amounts (asset_id, timestamp, amount) VALUES ($1, $2, $3);",
		&[&(asset_id as i32), &asset_valuation.timestamp, &asset_valuation.amount]
	).await?;
	
	client.query(
		"INSERT INTO public.asset_valuations (asset_id, timestamp, value_per_unit) VALUES ($1, $2, $3)", 
		&[&(asset_id as i32), &asset_valuation.timestamp, &(asset_valuation.value_per_unit as i32)]
	).await?;

	return Ok(());
}

pub async fn delete_by_id(pool: &Pool, asset_id: u32) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"DELETE FROM public.assets WHERE id=$1;", 
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
	let amount: f64 = row.try_get(6).unwrap_or(0.0);
	let value_per_unit: i32 = row.try_get(7).unwrap_or(0);

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

fn turn_row_into_asset_valuation(row: &tokio_postgres::Row) -> AssetValuation {
	let timestamp: chrono::DateTime<chrono::Utc> = row.get(1);
	let amount: f64 = row.try_get(2).unwrap_or(0.0);
	let value_per_unit: i32 = row.try_get(3).unwrap_or(0);

	return AssetValuation { 
		value_per_unit: value_per_unit as u32,
		amount,
		timestamp
	}
}

fn turn_row_into_deep_asset(row: &tokio_postgres::Row) -> DeepAsset {
	let id: i32 = row.get(0);
	let name: String = row.get(1);
	let description: Option<String> = row.get(2);
	let value_per_unit: i32 = row.try_get(3).unwrap_or(0);
	let amount: f64 = row.try_get(4).unwrap_or(0.0);
	let currency_id: i32 = row.get(5);
	let currency_minor_in_mayor: i32 = row.get(6);
	let currency_name: String = row.get(7);
	let currency_symbol: String = row.get(8);
	let user_id: i32 = row.get(9);
	let user_name: String = row.get(10);
	let user_superuser: bool = row.get(11);
	let tag_ids: Vec<Option<i32>> = row.get(12);
	let tag_names: Vec<Option<String>> = row.get(13);
	let tag_parent_ids: Vec<Option<i32>> = row.get(14);
	let tag_parent_names: Vec<Option<String>> = row.get(15);
	let tag_parent_parent_ids: Vec<Option<i32>> = row.get(16);
	let tag_parent_user_ids: Vec<Option<i32>> = row.get(17);
	let tag_user_ids: Vec<Option<i32>> = row.get(18);
	let tag_user_names: Vec<Option<String>> = row.get(19);
	let tag_user_superusers: Vec<Option<bool>> = row.get(20);

	let currency = crate::currency::Currency {
		id: Some(currency_id as u32),
		name: currency_name,
		minor_in_mayor: currency_minor_in_mayor as u32,
		symbol: currency_symbol
	};

	let user = crate::user::User {
		id: Some(user_id as u32),
		name: user_name,
		secret: None,
		superuser: user_superuser
	};

	let tags: Vec<crate::tag::DeepTag> = tag_ids
		.into_iter()
		.filter(|x| x.is_some())
		.enumerate()
		.map(|(i, tag_id)| {
			let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
				Some(x) => {
					match x {
						Some(_) => {
							Some(crate::tag::Tag {
								id: tag_parent_ids[i].map(|x| x as u32),
								name: tag_parent_names[i].clone().unwrap(),
								user_id: tag_parent_user_ids[i].unwrap() as u32,
								parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
							})
						},
						None => None,
					}
				},
				None => None,
			};
			
			crate::tag::DeepTag {
				id: tag_id.unwrap() as u32,
				name: tag_names[i].clone().unwrap(),
				user: crate::user::User {
					id: tag_user_ids[i].map(|x| x as u32),
					name: tag_user_names[i].clone().unwrap(),
					secret: None,
					superuser: tag_user_superusers[i].unwrap(),
				},
				parent,
			}
		}).collect();

	return DeepAsset {
		id: id as u32,
		name,
		description,
		value_per_unit: value_per_unit as u32,
		amount,
		user,
		currency,
		tags,
	}

}