use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Recipient;

pub async fn add(pool: &Pool, recipient: &Recipient) -> Result<(), Box<dyn Error>> {
	let client = pool.get().await.unwrap();
	let id: i32 = client
		.query(
			"INSERT INTO public.recipients (id, name, user_id) VALUES (DEFAULT, $1, $2) RETURNING id;",
			&[&recipient.name, &(recipient.user_id.unwrap_or(0) as i32)]
		)
		.await?
		[0].get(0);

	if recipient.tag_ids.is_some() {
		for tag_id in recipient.clone().tag_ids.unwrap() {
			client.query(
				"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
				&[&id, &(tag_id as i32)]
			).await?;
		}
	}
	return Ok(());
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Recipient>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT * FROM public.recipient_data;",
			&[]
		)
		.await?;
	
	if rows.is_empty() {
		return Err(Box::new(CustomError::NoItemFound{item_type: String::from("recipient")}));
	}
	
	return Ok(
		rows
			.into_iter()
			.map(|x| turn_row_into_recipient(&x))
			.collect()
	);
}

pub async fn get_by_id(pool: &Pool, recipient_id: u32) -> Result<Recipient, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT * FROM public.recipient_data WHERE id=$1;",
			&[&(recipient_id as i32)]
		)
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound{item_type: String::from("recipient"), filter: format!("id={recipient_id}")}));
	}

	return Ok(turn_row_into_recipient(&rows[0]));
}

pub async fn update(pool: &Pool, recipient: &Recipient) -> Result<(), Box<dyn Error>> {
	if recipient.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("recipient")}));
	}

	get_by_id(&pool, recipient.id.unwrap()).await?;

	let client = pool.get().await?;
	
	client.query(
			"UPDATE public.recipients SET name=$1 WHERE id=$2;",
			&[&recipient.name, &(recipient.id.unwrap() as i32)]
		)
		.await?;
	
	client.query(
			"DELETE FROM public.recipient_tags WHERE recipient_id=$1",
			&[&(recipient.id.unwrap() as i32)]
		).await?;

	if recipient.tag_ids.is_some() {
		for tag_id in recipient.tag_ids.clone().unwrap() {
			client.query(
				"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
				&[&(recipient.id.unwrap() as i32), &(tag_id as i32)]
			).await?;
		}
	}

	return Ok(());
}

fn turn_row_into_recipient(row: &tokio_postgres::Row) -> Recipient {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(2);
	let tag_ids = row
		.try_get(3)
		.unwrap_or(Vec::new())
		.into_iter()
		.map(|x: i32| x as u32)
		.collect();
	
	return Recipient {
		id: Some(id as u32),
		name: row.get(1),
		user_id: user_id.map(|x| x as u32),
		tag_ids: Some(tag_ids),
	};
}