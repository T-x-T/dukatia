use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Tag;
use crate::traits::*;

#[derive(Debug)]
pub struct TagDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Tag> for TagDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<Tag>, Box<dyn Error>> {
		let query = "SELECT * FROM public.tags";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}


#[derive(Debug)]
pub struct TagDbWriter<'a> {
	pool: &'a Pool,
	tag: Tag,
}

impl<'a> DbWriter<'a, Tag> for TagDbWriter<'a> {
	fn new(pool: &'a Pool, item: Tag) -> Self {
		return Self {
			pool,
			tag: item,
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let id: i32 = self.pool.get()
			.await?
			.query(
				"INSERT INTO public.tags (id, name, parent_id, user_id) VALUES (DEFAULT, $1, $2, $3) RETURNING id;",
				&[&self.tag.name, &(self.tag.parent_id.map(|x| x as i32)), &(self.tag.user_id as i32)]
			).await?[0].get(0);
		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.tag.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("tag")}));
		}
	
		let old = super::TagLoader::new(self.pool)
			.set_filter_id(self.tag.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.tag.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		self.pool.get()
			.await?
			.query(
				"UPDATE public.tags SET name=$1, parent_id=$2 WHERE id=$3;",
				&[&self.tag.name, &self.tag.parent_id.map(|x| x as i32), &self.tag.id.map(|x| x as i32)]
			)
			.await?;
	
		return Ok(());
	}
}

impl<'a> DbDeleter<'a, Tag> for TagDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		if self.tag.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("tag")}));
		}

		let old = super::TagLoader::new(self.pool)
			.set_filter_id(self.tag.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.tag.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		self.pool.get()
			.await?
			.query("DELETE FROM public.tags WHERE id=$1;", &[&(self.tag.id.unwrap() as i32)]).await?;
	
		self.pool.get().await?
			.query("UPDATE public.tags SET parent_id=null WHERE parent_id=$1", &[&(self.tag.id.unwrap() as i32)]).await?;
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Tag {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let parent_id: Option<i32> = value.get(2);
		let user_id: Option<i32> = value.get(3);
	
		return Self {
			id: Some(id as u32),
			name,
			user_id: user_id.map_or(0, |x| x as u32),
			parent_id: parent_id.map(|x| x as u32),
		}
	}
}