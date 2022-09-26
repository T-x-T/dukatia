use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Tag;

pub async fn add(pool: &Pool, tag: &Tag) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"INSERT INTO public.\"Tags\" (id, name, parent, \"user\") VALUES (DEFAULT, $1, $2, $3);",
			&[&tag.name, &(tag.parent_id.map(|x| x as i32)), &(tag.user_id as i32)]
		).await?;
	return Ok(());
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Tag>, Box<dyn Error>> {
	return Ok(
		pool.get()
		.await?
		.query("SELECT * FROM public.\"Tags\";", &[])
		.await?
		.iter()
		.map(|x| turn_row_into_tag(&x))
		.collect()
	)
}

pub async fn get_by_id(pool: &Pool, tag_id: u32) -> Result<Tag, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.\"Tags\" WHERE id=$1;", &[&(tag_id as i32)]).await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound{item_type: String::from("tag"), filter: format!("id={}", tag_id)}));
	}

	return Ok(turn_row_into_tag(&rows[0]));
}

pub async fn update(pool: &Pool, tag: &Tag) -> Result<(), Box<dyn Error>> {
	if tag.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("tag")}));
	}

	get_by_id(&pool, tag.id.unwrap()).await?;

	pool.get()
		.await?
		.query(
			"UPDATE public.\"Tags\" SET name=$1, parent=$2 WHERE id=$3;",
			&[&tag.name, &tag.parent_id.map(|x| x as i32), &tag.id.map(|x| x as i32)]
		)
		.await?;

	return Ok(());
}

pub async fn delete(pool: &Pool, tag_id: u32) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query("DELETE FROM public.\"Tags\" WHERE id=$1;", &[&(tag_id as i32)]).await?;

	pool.get().await?
		.query("UPDATE public.\"Tags\" SET parent=null WHERE parent=$1", &[&(tag_id as i32)]).await?;

	return Ok(());
}

fn turn_row_into_tag(row: &tokio_postgres::Row) -> Tag {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(3);
	let parent_id: Option<i32> = row.get(2);

	return Tag {
		id: Some(id as u32),
		name: row.get(1),
		user_id: user_id.map_or(0, |x| x as u32),
		parent_id: parent_id.map(|x| x as u32),
	}
}