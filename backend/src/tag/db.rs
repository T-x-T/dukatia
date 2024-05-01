use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
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

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		self.pool.get()
			.await?
			.query(
				"INSERT INTO public.tags (id, name, parent_id, user_id) VALUES ($1, $2, $3, $4)",
				&[&self.tag.id, &self.tag.name, &(self.tag.parent_id), &self.tag.user_id]
			).await?;
		return Ok(self.tag.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let old = super::TagLoader::new(self.pool)
			.set_filter_id(self.tag.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.tag.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		self.pool.get()
			.await?
			.query(
				"UPDATE public.tags SET name=$1, parent_id=$2 WHERE id=$3;",
				&[&self.tag.name, &self.tag.parent_id, &self.tag.id]
			)
			.await?;
	
		return Ok(());
	}
}

impl<'a> DbDeleter<'a, Tag> for TagDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		let old = super::TagLoader::new(self.pool)
			.set_filter_id(self.tag.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.tag.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		self.pool.get()
			.await?
			.query("DELETE FROM public.tags WHERE id=$1;", &[&(self.tag.id)]).await?;
	
		self.pool.get().await?
			.query("UPDATE public.tags SET parent_id=null WHERE parent_id=$1", &[&self.tag.id]).await?;
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Tag {
	fn from(value: tokio_postgres::Row) -> Self {
		let name: String = value.get(0);
		let id: Uuid = value.get(1);
		let parent_id: Option<Uuid> = value.get(2);
		let user_id: Uuid = value.get(3);
	
		return Self {
			id,
			name,
			user_id,
			parent_id,
		}
	}
}