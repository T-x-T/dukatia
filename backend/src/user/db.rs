use deadpool_postgres::Pool;
use std::error::Error;
use chrono::prelude::*;
use uuid::Uuid;
use super::super::CustomError;
use super::{User, LoginCredentials};
use crate::traits::*;

#[derive(Debug)]
pub struct UserDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, User> for UserDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<User>, Box<dyn Error>> {
		let query = "SELECT id, name, superuser, active, last_logon FROM public.users";
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
pub struct UserDbWriter<'a> {
	pool: &'a Pool,
	user: User,
}

impl<'a> DbWriter<'a, User> for UserDbWriter<'a> {
	fn new(pool: &'a Pool, item: User) -> Self {
		return Self {
			pool,
			user: item,
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		let client = self.pool.get().await.unwrap();
		client
			.query(
				"INSERT INTO public.users (id, name, secret, superuser, active) VALUES ($1, $2, $3, $4, $5);", 
				&[&self.user.id, &self.user.name, &self.user.encrypted_secret, &self.user.superuser, &self.user.active]
			).await?;

		insert_charts_and_stuff(self.pool, &self.user).await?;

		return Ok(self.user.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let client = self.pool.get().await?;

		if self.user.encrypted_secret.is_some() {
			client
				.query(
					"UPDATE public.users SET name=$1, secret=$2, superuser=$3, active=$4 WHERE id=$5;",
					&[&self.user.name, &self.user.encrypted_secret, &self.user.superuser, &self.user.active, &self.user.id]
				)
				.await?;
		} else {
			client
				.query(
					"UPDATE public.users SET name=$1, superuser=$2, active=$3 WHERE id=$4;",
					&[&self.user.name, &self.user.superuser, &self.user.active, &self.user.id]
				)
				.await?;
		}

		if !self.user.active {
			client
				.query(
					"DELETE FROM public.access_tokens WHERE user_id=$1;",
					&[&self.user.id]
				)
				.await?;
		}


		return Ok(());
	}
}

pub async fn login(pool: &Pool, credentials: &LoginCredentials, hashed_secret: String) -> Result<super::LoginResult, Box<dyn Error>> {
	let client = pool.get().await?;
	
	let rows = client
		.query(
			"SELECT id, last_logon FROM public.users WHERE name=$1 AND secret=$2 AND active=true",
			&[&credentials.name, &hashed_secret]
		).await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::InvalidCredentials));
	}

	let user_id: Uuid = rows[0].get(0);
	let last_logon: Option<chrono::DateTime<chrono::Utc>> = rows[0].get(1);

	client
		.query(
			"UPDATE public.users SET last_logon=$1 WHERE id=$2;",
			&[&Some(Utc::now()), &user_id]
		)
		.await?;

	return Ok(super::LoginResult {
		user_id,
		first_login: last_logon.is_none(),
		..Default::default()
	});
}

pub async fn update_secret(pool: &Pool, user_id: Uuid, new_hashed_secret: String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"UPDATE public.users SET secret=$1 WHERE id=$2", 
		&[&new_hashed_secret, &user_id]
		).await?;

	return Ok(());
}

pub async fn get_by_id(pool: &Pool, id: &Uuid) -> Result<User, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT id, name, superuser, active, last_logon FROM public.users WHERE id=$1",
			&[id]
		).await?;

	return Ok(User {
		id: *id,
		name: rows[0].get(1),
		superuser: rows[0].get(2),
		active: rows[0].get(3),
		last_logon: rows[0].get(4),
		..Default::default()
	});
}

impl From<tokio_postgres::Row> for User {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: Uuid = value.get(0);
		let name: String = value.get(1);
		let superuser: bool = value.get(2);
		let active: bool = value.get(3);
		let last_logon: Option<DateTime<Utc>> = value.get(4);
		let encrypted_secret: Option<String> = value.try_get(5).unwrap_or_default();


		return User {
			id,
			name,
			encrypted_secret,
			superuser,
			active,
			last_logon,
			..Default::default()
		};
	}
}

pub async fn insert_charts_and_stuff(pool: &Pool, user: &User) -> Result<(), Box<dyn Error>> {
	let client = pool.get().await?;
	
	let dashboard_id: Uuid = client
	.query(
		"INSERT INTO public.dashboards(id, user_id, name, description) VALUES (DEFAULT, $1, 'Default Dashboard', 'The default Dashboard') RETURNING id;",
		&[&user.id]
	).await?[0].get(0);

	let chart_ids: Vec<Uuid> = client
		.query(
			"
				INSERT INTO 
					public.charts(id, user_id, chart_type, title, filter_from, filter_to, filter_collection, date_period, max_items, date_range, top_left_x, top_left_y, bottom_right_x, bottom_right_y, only_positive, only_negative)
				VALUES
					(DEFAULT,$1,'pie','Spending per Tag in Date Range',NULL,NULL,'get_per_tag_over_time','daily',5,6,4,0,6,2,NULL,True),
					(DEFAULT,$1,'line','Balance per Recipient over Time',NULL,NULL,'get_per_recipient_over_time','monthly',10,6,6,0,10,2,NULL,NULL),
					(DEFAULT,$1,'line','Balance per Account over Time',NULL,NULL,'get_per_account_over_time','monthly',10,6,0,2,5,4,False,False),
					(DEFAULT,$1,'line','Earning and spending over Time',NULL,NULL,'get_earning_spending_net_over_time','monthly',NULL,7,0,4,10,6,NULL,NULL),
					(DEFAULT,$1,'pie','Spending per Recipient in Date Range',NULL,NULL,'get_per_recipient_over_time','daily',7,6,2,0,4,2,NULL,True),
					(DEFAULT,$1,'line','Balance per Currency over Time',NULL,NULL,'get_per_currency_over_time','monthly',10,6,5,2,10,4,NULL,NULL),
					(DEFAULT,$1,'table','Current Balance per Account',NULL,NULL,'get_per_account_over_time','daily',10,0,0,0,2,2,False,False)
				RETURNING id;
			",
			&[&user.id]
		).await?
		.into_iter()
		.map(|x| x.get(0))
		.collect();

	for chart_id in chart_ids {
		client.query(
			"INSERT INTO public.dashboard_charts (dashboard_id, chart_id) VALUES ($1, $2);", 
			&[&dashboard_id, &chart_id]
		).await?;
	};

	return Ok(());
}