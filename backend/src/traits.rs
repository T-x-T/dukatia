use deadpool_postgres::Pool;
use postgres_types::ToSql;
use tokio_postgres::Row;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::error::Error;


#[derive(Debug, Clone, Copy, Deserialize)]
pub enum FilterAndSortProperties {
	AccountId,
	Comment,
	CurrencyId,
	Id,
	RecipientId,
	Status,
	Timestamp,
	UserId,
	TotalAmount,
}

impl std::fmt::Display for FilterAndSortProperties {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return match self {
			FilterAndSortProperties::AccountId => write!(f, "account_id"),
			FilterAndSortProperties::Comment => write!(f, "comment"),
			FilterAndSortProperties::CurrencyId => write!(f, "currency_id"),
			FilterAndSortProperties::Id => write!(f, "id"),
			FilterAndSortProperties::RecipientId => write!(f, "recipient_id"),
			FilterAndSortProperties::Status => write!(f, "status"),
			FilterAndSortProperties::Timestamp => write!(f, "timestamp"),
			FilterAndSortProperties::UserId => write!(f, "user_id"),
			FilterAndSortProperties::TotalAmount => write!(f, "total_amount"),
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct QueryParameters {
	pub max_results: Option<u32>,
	pub skip_results: Option<u32>,
	pub sort_property: Option<FilterAndSortProperties>,
	pub sort_direction: Option<String>,
	pub filters: Filters,
}

impl QueryParameters {
	pub fn set_max_results_opt(mut self, max_results: Option<u32>) -> QueryParameters {
		self.max_results = max_results;
		return self;
	}

	pub fn set_skip_results_opt(mut self, skip_results: Option<u32>) -> QueryParameters {
		self.skip_results = skip_results;
		return self;
	}

	pub fn set_sort_property_opt(mut self, sort_property: Option<FilterAndSortProperties>) -> QueryParameters {
		self.sort_property = sort_property;
		return self;
	}

	pub fn set_sort_direction_opt(mut self, sort_direction: Option<String>) -> QueryParameters {
		self.sort_direction = sort_direction;
		return self;
	}

	pub fn set_filters(mut self, filters: Filters) -> QueryParameters {
		self.filters = filters;
		return self;
	}
}

#[derive(Debug, Default, Clone)]
pub struct Filters {
	pub id: Option<(u32, NumberFilterModes)>,
	pub asset_id: Option<(u32, NumberFilterModes)>,
	pub user_id: Option<(u32, NumberFilterModes)>,
	pub currency_id: Option<(u32, NumberFilterModes)>,
	pub account_id: Option<(u32, NumberFilterModes)>,
	pub recipient_id: Option<(u32, NumberFilterModes)>,
	pub comment: Option<(String, StringFilterModes)>,
	pub time_range: Option<(DateTime<Utc>, DateTime<Utc>, TimeRangeFilterModes)>
}

#[derive(Debug, Clone, Copy)]
pub enum NumberFilterModes {
	Exact, Not, Less, More
}

impl Default for NumberFilterModes {
	fn default() -> Self {
		return Self::Exact;
	}
}

impl From<String> for NumberFilterModes {
	fn from(value: String) -> Self {
		return match value.as_str() {
			"not" => NumberFilterModes::Not,
			"more" => NumberFilterModes::More,
			"less" => NumberFilterModes::Less,
			_ => NumberFilterModes::Exact,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum StringFilterModes {
	Contains, Exact, BeginsWith, EndsWith, DoesntContain
}

impl Default for StringFilterModes {
	fn default() -> Self {
		return Self::Contains;
	}
}

impl From<String> for StringFilterModes {
	fn from(value: String) -> Self {
		return match value.as_str() {
			"exact" => StringFilterModes::Exact,
			"begins_with" => StringFilterModes::BeginsWith,
			"ends_with" => StringFilterModes::EndsWith,
			"doesnt_contain" => StringFilterModes::DoesntContain,
			_ => StringFilterModes::Contains,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TimeRangeFilterModes {
	Between, Outside
}

impl Default for TimeRangeFilterModes {
	fn default() -> Self {
		return Self::Between;
	}
}

impl From<String> for TimeRangeFilterModes {
	fn from(value: String) -> Self {
		return match value.as_str() {
			"outside" => TimeRangeFilterModes::Outside,
			_ => TimeRangeFilterModes::Between,
		}
	}
}



pub trait Save {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>>;
}

pub trait Delete {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

pub trait Loader<'a, T: Clone>: Sized + Clone {
	fn new(pool: &'a Pool) -> Self;
	fn get_query_parameters(&self) -> &QueryParameters;
	fn set_query_parameters(self, query_parameters: QueryParameters) -> Self;
	async fn get(self) -> Result<Vec<T>, Box<dyn Error>>;
	
	fn set_filter_id(self, id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.id = Some((id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_asset_id(self, asset_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.asset_id = Some((asset_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_user_id(self, user_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.user_id = Some((user_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_currency_id(self, currency_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.currency_id = Some((currency_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_account_id(self, account_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.account_id = Some((account_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_recipient_id(self, recipient_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.recipient_id = Some((recipient_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_comment(self, comment: String, filter_mode: StringFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.comment = Some((comment, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_time_range(self, lower_time: DateTime<Utc>, upper_time: DateTime<Utc>, filter_mode: TimeRangeFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.time_range = Some((lower_time, upper_time, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	async fn get_first(self) -> Result<T, Box<dyn Error>> {
		match self.get().await?.first() {
			Some(x) => return Ok(x.clone()),
			None => return Err(Box::new(crate::CustomError::NoItemFound { item_type: "unknown".to_string() })),
		}
	}
}

pub trait DbReader<'a, T: From<Row>>: Sized {
	fn new(pool: &'a Pool) -> Self;
	fn get_pool(&self) -> &Pool;
	fn get_query_parameters(&self) -> &QueryParameters;
	fn set_query_parameters(self, query_parameters: QueryParameters) -> Self;
	async fn execute(self) -> Result<Vec<T>, Box<dyn Error>>;

	#[allow(clippy::too_many_lines)]
	fn get_formatted_query_parameters(&self, table_name: Option<String>) -> (String, Vec<Box<(dyn ToSql + Sync)>>)  {
		let mut i = 1;
		let mut parameters = String::new();
		let mut parameter_values: Vec<Box<(dyn ToSql + Sync)>> = Vec::new();

		
		
		let mut first_where_clause = true;

		if self.get_query_parameters().filters.id.is_some() {
			match self.get_query_parameters().filters.id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.asset_id.is_some() {
			match self.get_query_parameters().filters.asset_id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}asset_id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}asset_id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}asset_id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}asset_id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.asset_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.user_id.is_some() {
			match self.get_query_parameters().filters.user_id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}user_id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}user_id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}user_id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}user_id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.user_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.currency_id.is_some() {
			match self.get_query_parameters().filters.currency_id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}currency_id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}currency_id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}currency_id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}currency_id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.currency_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.account_id.is_some() {
			match self.get_query_parameters().filters.account_id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}account_id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}account_id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}account_id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}account_id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.account_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.recipient_id.is_some() {
			match self.get_query_parameters().filters.recipient_id.unwrap().1 {
				NumberFilterModes::Exact => parameters.push_str(format!(" {} {}recipient_id=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} {}recipient_id!=${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} {}recipient_id<${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} {}recipient_id>${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.recipient_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.comment.is_some() {
			match self.get_query_parameters().filters.comment.clone().unwrap().1 {
				StringFilterModes::Exact => {
					parameters.push_str(format!(" {} {}comment LIKE ${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str());
					parameter_values.push(Box::new(self.get_query_parameters().filters.comment.clone().unwrap().0));
				},
				StringFilterModes::Contains => {
					parameters.push_str(format!(" {} {}comment LIKE ${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str());
					parameter_values.push(Box::new(format!("%{}%", self.get_query_parameters().filters.comment.clone().unwrap().0)));
				},
				StringFilterModes::BeginsWith => {
					parameters.push_str(format!(" {} {}comment LIKE ${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str());
					parameter_values.push(Box::new(format!("{}%", self.get_query_parameters().filters.comment.clone().unwrap().0)));
				},
				StringFilterModes::EndsWith => {
					parameters.push_str(format!(" {} {}comment LIKE ${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str());
					parameter_values.push(Box::new(format!("%{}", self.get_query_parameters().filters.comment.clone().unwrap().0)));
				},
				StringFilterModes::DoesntContain => {
					parameters.push_str(format!(" {} {}comment NOT LIKE ${i}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str());
					parameter_values.push(Box::new(format!("%{}%", self.get_query_parameters().filters.comment.clone().unwrap().0)));
				},
			};
			first_where_clause = false;
			i += 1;
		}

		if self.get_query_parameters().filters.time_range.is_some() {
			match self.get_query_parameters().filters.time_range.unwrap().2 {
				TimeRangeFilterModes::Between => parameters.push_str(format!(" {} {}timestamp BETWEEN ${i} AND ${}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}, i + 1).as_str()),
				TimeRangeFilterModes::Outside => parameters.push_str(format!(" {} {}timestamp NOT BETWEEN ${i} AND ${}", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}, i + 1).as_str()),
			};
			//first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.time_range.unwrap().0));
			parameter_values.push(Box::new(self.get_query_parameters().filters.time_range.unwrap().1));
			i += 2;
		}



		if self.get_query_parameters().sort_property.is_some() {
			let direction = match &self.get_query_parameters().sort_direction {
				Some(x) => match x.as_str() {
					"ASC" => "ASC",
					_ => "DESC",
				},
				None => "DESC",
			};

			parameters.push_str(format!(" ORDER BY {}{} {}", if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}, self.get_query_parameters().sort_property.unwrap(), direction).as_str());
		}

		if self.get_query_parameters().skip_results.is_some() {
			parameters.push_str(format!(" OFFSET ${i}").as_str());
			parameter_values.push(Box::new(i64::from(self.get_query_parameters().skip_results.unwrap())));
			i += 1;
		}
		if self.get_query_parameters().max_results.is_some() {
			parameters.push_str(format!(" LIMIT ${i}").as_str());
			parameter_values.push(Box::new(i64::from(self.get_query_parameters().max_results.unwrap())));
		}

		return (
			parameters,
			parameter_values
		);
	}

	async fn actually_execute(self, query: &str) -> Result<Vec<Row>, Box<dyn Error>> {
		let parameters = self.get_formatted_query_parameters(None);
		let parameter_values: Vec<_> = parameters.1.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		let rows = self.get_pool()
			.get()
			.await?
			.query(format!("{}{};", query, parameters.0).as_str(), parameter_values.as_slice())
			.await?;
	
		return Ok(rows);
	}
}

pub trait DbWriter<'a, T> {
	fn new(pool: &'a Pool, item: T) -> Self;
	async fn insert(self) -> Result<u32, Box<dyn Error>>;
	async fn replace(self) -> Result<(), Box<dyn Error>>;
}

pub trait DbDeleter<'a, T>: DbWriter<'a, T> {
	async fn delete(self) -> Result<(), Box<dyn Error>>;
}