use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Recipient, DeepRecipient};

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
	return Ok(
		pool.get()
			.await?
			.query(
				"SELECT * FROM public.recipient_data;",
				&[]
			)
			.await?
			.iter()
			.map(turn_row_into_recipient)
			.collect()
	);
}

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepRecipient>, Box<dyn Error>> {
	return Ok(
		pool.get()
			.await?
			.query(
				"SELECT * FROM public.deep_recipients;",
				&[]
			)
			.await?
			.iter()
			.map(turn_row_into_deep_recipient)
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

	get_by_id(pool, recipient.id.unwrap()).await?;

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

fn turn_row_into_deep_recipient(row: &tokio_postgres::Row) -> DeepRecipient {
	let id: i32 = row.get(0);
	let name: String = row.get(1);
	let user_id: Option<i32> = row.get(2);
	let user_name: Option<String> = row.get(3);
	let user_superuser: Option<bool> = row.get(4);
	let tag_ids: Vec<Option<i32>> = row.get(5);
	let tag_names: Vec<Option<String>> = row.get(6);
	let tag_parent_ids: Vec<Option<i32>> = row.get(7);
	let tag_parent_names: Vec<Option<String>> = row.get(8);
	let tag_parent_parent_ids: Vec<Option<i32>> = row.get(9);
	let tag_parent_user_ids: Vec<Option<i32>> = row.get(10);
	let tag_user_ids: Vec<Option<i32>> = row.get(11);
	let tag_user_names: Vec<Option<String>> = row.get(12);
	let tag_user_superusers: Vec<Option<bool>> = row.get(13);

	let user = user_id.map(|_| crate::user::User {
		id: Some(user_id.unwrap() as u32),
		name: user_name.unwrap(),
		secret: None,
		superuser: user_superuser.unwrap()
	});

	let tags: Vec<crate::tag::DeepTag> = tag_ids
		.into_iter()
		.filter(Option::is_some)
		.enumerate()
		.map(|(i, tag_id)| {
			let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
				Some(x) => {
					x.as_ref().map(|_| crate::tag::Tag {
						id: tag_parent_ids[i].map(|x| x as u32),
						name: tag_parent_names[i].clone().unwrap(),
						user_id: tag_parent_user_ids[i].unwrap() as u32,
						parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
					})
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

		return DeepRecipient {
			id: id as u32,
			name,
			user,
			tags,
		};
}