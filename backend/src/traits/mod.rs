#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use postgres_types::ToSql;
use tokio_postgres::Row;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::error::Error;
use uuid::Uuid;


#[derive(Debug, Clone, Copy, Deserialize)]
pub enum FilterAndSortProperties {
	AccountId,
	Comment,
	CurrencyId,
	Id,
	IdUuid,
	RecipientId,
	Status,
	Timestamp,
	UserId,
	TotalAmount,
	TagId,
	Name,
	Symbol,
	MinorInmajor,
	ParentId,
	Balance,
	DefaultCurrencyId,
	Description,
	FloatAmount,
	IntAmount,
	ValuePerUnit,
	Rollover,
	ActiveFrom,
	ActiveTo,
}

impl std::fmt::Display for FilterAndSortProperties {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return match self {
			FilterAndSortProperties::AccountId => write!(f, "account_id"),
			FilterAndSortProperties::Comment => write!(f, "comment"),
			FilterAndSortProperties::CurrencyId => write!(f, "currency_id"),
			FilterAndSortProperties::Id => write!(f, "id"),
			FilterAndSortProperties::IdUuid => write!(f, "id"),
			FilterAndSortProperties::RecipientId => write!(f, "recipient_id"),
			FilterAndSortProperties::Status => write!(f, "status"),
			FilterAndSortProperties::Timestamp => write!(f, "timestamp"),
			FilterAndSortProperties::UserId => write!(f, "user_id"),
			FilterAndSortProperties::TotalAmount => write!(f, "total_amount"),
			FilterAndSortProperties::TagId => write!(f, "tag_id"),
			FilterAndSortProperties::Name => write!(f, "name"),
			FilterAndSortProperties::Symbol => write!(f, "symbol"),
			FilterAndSortProperties::MinorInmajor => write!(f, "minor_in_major"),
			FilterAndSortProperties::ParentId => write!(f, "parent_id"),
			FilterAndSortProperties::Balance => write!(f, "balance"),
			FilterAndSortProperties::DefaultCurrencyId => write!(f, "default_currency_id"),
			FilterAndSortProperties::Description => write!(f, "description"),
			FilterAndSortProperties::FloatAmount | FilterAndSortProperties::IntAmount => write!(f, "amount"),
			FilterAndSortProperties::ValuePerUnit => write!(f, "value_per_unit"),
			FilterAndSortProperties::Rollover => write!(f, "rollover"),
			FilterAndSortProperties::ActiveFrom => write!(f, "active_from"),
			FilterAndSortProperties::ActiveTo => write!(f, "active_to"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
	Asc, Desc
}

impl From<SortDirection> for &str {
	fn from(value: SortDirection) -> Self {
		return match value {
			SortDirection::Asc => "ASC",
			SortDirection::Desc => "DESC",
		}
	}
}

impl From<SortDirection> for String {
	fn from(value: SortDirection) -> Self {
		return String::from(<&str>::from(value));
	}
}

#[derive(Debug, Default, Clone)]
pub struct QueryParameters {
	pub max_results: Option<u32>,
	pub skip_results: Option<u32>,
	pub sort_property: Option<FilterAndSortProperties>,
	pub sort_direction: Option<SortDirection>,
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

	pub fn set_sort_direction_opt(mut self, sort_direction: Option<SortDirection>) -> QueryParameters {
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
	pub id_uuid: Option<(Uuid, NumberFilterModes)>,
	pub total_amount: Option<(i32, NumberFilterModes)>,
	pub asset_id: Option<(u32, NumberFilterModes)>,
	pub user_id: Option<(u32, NumberFilterModes)>,
	pub currency_id: Option<(u32, NumberFilterModes)>,
	pub account_id: Option<(u32, NumberFilterModes)>,
	pub recipient_id: Option<(u32, NumberFilterModes)>,
	pub tag_id: Option<(u32, NumberFilterModes)>,
	pub comment: Option<(String, StringFilterModes)>,
	pub time_range: Option<(DateTime<Utc>, DateTime<Utc>, TimeRangeFilterModes)>,
	pub name: Option<(String, StringFilterModes)>,
	pub symbol: Option<(String, StringFilterModes)>,
	pub minor_in_major: Option<(u32, NumberFilterModes)>,
	pub parent_id: Option<(u32, NumberFilterModes)>,
	pub balance: Option<(i64, NumberFilterModes)>,
	pub default_currency_id: Option<(u32, NumberFilterModes)>,
	pub description: Option<(String, StringFilterModes)>,
	pub float_amount: Option<(f64, NumberFilterModes)>,
	pub int_amount: Option<(i32, NumberFilterModes)>,
	pub value_per_unit: Option<(u32, NumberFilterModes)>,
	pub rollover: Option<(bool, BoolFilterModes)>,
	pub active_from: Option<(DateTime<Utc>, DateTime<Utc>, TimeRangeFilterModes)>,
	pub active_to: Option<(DateTime<Utc>, DateTime<Utc>, TimeRangeFilterModes)>,
}

#[derive(Debug, Clone, Copy)]
pub enum NumberFilterModes {
	Exact, Not, Less, More, ExactOrAlsoNull
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
			"exact_or_also_null" => NumberFilterModes::ExactOrAlsoNull,
			_ => NumberFilterModes::Exact,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum BoolFilterModes {
	Is, Not
}

impl Default for BoolFilterModes {
	fn default() -> Self {
		return Self::Is;
	}
}

impl From<String> for BoolFilterModes {
	fn from(value: String) -> Self {
		return match value.as_str() {
			"not" => BoolFilterModes::Not,
			_ => BoolFilterModes::Is,
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

pub trait Create {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>>;
}

pub trait Update {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

pub trait Delete {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

#[allow(unused)]
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
	
	fn set_filter_id_uuid(self, id: Uuid, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.id_uuid = Some((id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}
	
	fn set_filter_total_amount(self, total_amount: i32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.total_amount = Some((total_amount, filter_mode));
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

	fn set_filter_tag_id(self, tag_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.tag_id = Some((tag_id, filter_mode));
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

	fn set_filter_name(self, name: String, filter_mode: StringFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.name = Some((name, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_symbol(self, symbol: String, filter_mode: StringFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.symbol = Some((symbol, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_minor_in_major(self, minor_in_major: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.minor_in_major = Some((minor_in_major, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_parent_id(self, parent_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.parent_id = Some((parent_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_balance(self, balance: i64, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.balance = Some((balance, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_default_currency_id(self, default_currency_id: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.default_currency_id = Some((default_currency_id, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_description(self, description: String, filter_mode: StringFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.description = Some((description, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_float_amount(self, amount: f64, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.float_amount = Some((amount, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_int_amount(self, amount: i32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.int_amount = Some((amount, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_value_per_unit(self, value_per_unit: u32, filter_mode: NumberFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.value_per_unit = Some((value_per_unit, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_rollover(self, rollover: bool, filter_mode: BoolFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.rollover = Some((rollover, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_active_from(self, lower_active_from: DateTime<Utc>, upper_active_from: DateTime<Utc>, filter_mode: TimeRangeFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.active_from = Some((lower_active_from, upper_active_from, filter_mode));
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_active_to(self, lower_active_to: DateTime<Utc>, upper_active_to: DateTime<Utc>, filter_mode: TimeRangeFilterModes) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.active_to = Some((lower_active_to, upper_active_to, filter_mode));
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
			let property_name = get_property_name("id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.id.unwrap().0 as i32));
			i += 1;
		}

		if self.get_query_parameters().filters.id_uuid.is_some() {
			let property_name = get_property_name("id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.id_uuid.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.id_uuid.unwrap().0));
			i += 1;
		}

		if self.get_query_parameters().filters.total_amount.is_some() {
			let property_name = get_property_name("total_amount", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.total_amount.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(i64::from(self.get_query_parameters().filters.total_amount.unwrap().0)));
			i += 1;
		}
		
		if self.get_query_parameters().filters.asset_id.is_some() {
			let property_name = get_property_name("asset_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.asset_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.asset_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.user_id.is_some() {
			let property_name = get_property_name("user_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.user_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.user_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.currency_id.is_some() {
			let property_name = get_property_name("currency_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.currency_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.currency_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.account_id.is_some() {
			let property_name = get_property_name("account_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.account_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.account_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.recipient_id.is_some() {
			let property_name = get_property_name("recipient_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.recipient_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.recipient_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.minor_in_major.is_some() {
			let property_name = get_property_name("minor_in_major", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.minor_in_major.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.minor_in_major.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.parent_id.is_some() {
			let property_name = get_property_name("parent_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.parent_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.parent_id.unwrap().0 as i32));
			i += 1;
		}

		if self.get_query_parameters().filters.default_currency_id.is_some() {
			let property_name = get_property_name("default_currency_id", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.default_currency_id.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.default_currency_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.balance.is_some() {
			let property_name = get_property_name("balance", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.balance.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.balance.unwrap().0));
			i += 1;
		}
		
		if self.get_query_parameters().filters.float_amount.is_some() {
			let property_name = get_property_name("amount", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.float_amount.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.float_amount.unwrap().0));
			i += 1;
		}
		
		if self.get_query_parameters().filters.int_amount.is_some() {
			let property_name = get_property_name("amount", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.int_amount.unwrap().1, where_or_and, &property_name, i).as_str());
			
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.int_amount.unwrap().0));
			i += 1;
		}
		
		if self.get_query_parameters().filters.value_per_unit.is_some() {
			let property_name = get_property_name("value_per_unit", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			parameters.push_str(render_number_filter_mode(self.get_query_parameters().filters.value_per_unit.unwrap().1, where_or_and, &property_name, i).as_str());

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.value_per_unit.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.tag_id.is_some() {
			match self.get_query_parameters().filters.tag_id.unwrap().1 {
				NumberFilterModes::Exact | NumberFilterModes::ExactOrAlsoNull => parameters.push_str(format!(" {} ${i} = ANY({}tags)", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Not => parameters.push_str(format!(" {} NOT ${i} = ANY({}tags)", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::Less => parameters.push_str(format!(" {} ${i} > ANY({}tags)", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
				NumberFilterModes::More => parameters.push_str(format!(" {} ${i} < ANY({}tags)", if first_where_clause {"WHERE"} else {"AND"}, if table_name.is_some() {table_name.clone().unwrap() + "."} else {String::new()}).as_str()),
			};

			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.tag_id.unwrap().0 as i32));
			i += 1;
		}
		
		if self.get_query_parameters().filters.rollover.is_some() {
			let property_name = get_property_name("rollover", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			match self.get_query_parameters().filters.rollover.unwrap().1 {
				BoolFilterModes::Is => parameters.push_str(format!(" {where_or_and} {property_name}=${i}").as_str()),
				BoolFilterModes::Not => parameters.push_str(format!(" {where_or_and} {property_name}!=${i}").as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.rollover.unwrap().0));
			i += 1;
		}
		
		if self.get_query_parameters().filters.comment.is_some() {
			let property_name = get_property_name("comment", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			let res = render_string_filter_mode(self.get_query_parameters().filters.comment.clone().unwrap(), where_or_and, &property_name, i);
			parameters.push_str(res.0.as_str());
			parameter_values.push(Box::new(res.1));

			first_where_clause = false;
			i += 1;
		}
		
		if self.get_query_parameters().filters.name.is_some() {
			let property_name = get_property_name("name", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			let res = render_string_filter_mode(self.get_query_parameters().filters.name.clone().unwrap(), where_or_and, &property_name, i);
			parameters.push_str(res.0.as_str());
			parameter_values.push(Box::new(res.1));

			first_where_clause = false;
			i += 1;
		}
		
		if self.get_query_parameters().filters.symbol.is_some() {
			let property_name = get_property_name("symbol", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			let res = render_string_filter_mode(self.get_query_parameters().filters.symbol.clone().unwrap(), where_or_and, &property_name, i);
			parameters.push_str(res.0.as_str());
			parameter_values.push(Box::new(res.1));

			first_where_clause = false;
			i += 1;
		}
		
		if self.get_query_parameters().filters.description.is_some() {
			let property_name = get_property_name("description", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};
			
			let res = render_string_filter_mode(self.get_query_parameters().filters.description.clone().unwrap(), where_or_and, &property_name, i);
			parameters.push_str(res.0.as_str());
			parameter_values.push(Box::new(res.1));

			first_where_clause = false;
			i += 1;
		}

		if self.get_query_parameters().filters.time_range.is_some() {
			let property_name = get_property_name("timestamp", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			match self.get_query_parameters().filters.time_range.unwrap().2 {
				TimeRangeFilterModes::Between => parameters.push_str(format!(" {where_or_and} {property_name} BETWEEN ${i} AND ${}", i + 1).as_str()),
				TimeRangeFilterModes::Outside => parameters.push_str(format!(" {where_or_and} {property_name} NOT BETWEEN ${i} AND ${}", i + 1).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.time_range.unwrap().0));
			parameter_values.push(Box::new(self.get_query_parameters().filters.time_range.unwrap().1));
			i += 2;
		}

		if self.get_query_parameters().filters.active_from.is_some() {
			let property_name = get_property_name("active_from", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			match self.get_query_parameters().filters.active_from.unwrap().2 {
				TimeRangeFilterModes::Between => parameters.push_str(format!(" {where_or_and} {property_name} BETWEEN ${i} AND ${}", i + 1).as_str()),
				TimeRangeFilterModes::Outside => parameters.push_str(format!(" {where_or_and} {property_name} NOT BETWEEN ${i} AND ${}", i + 1).as_str()),
			};
			first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.active_from.unwrap().0));
			parameter_values.push(Box::new(self.get_query_parameters().filters.active_from.unwrap().1));
			i += 2;
		}

		if self.get_query_parameters().filters.active_to.is_some() {
			let property_name = get_property_name("active_to", &table_name);
			let where_or_and = if first_where_clause {"WHERE"} else {"AND"};

			match self.get_query_parameters().filters.active_to.unwrap().2 {
				TimeRangeFilterModes::Between => parameters.push_str(format!(" {where_or_and} {property_name} BETWEEN ${i} AND ${}", i + 1).as_str()),
				TimeRangeFilterModes::Outside => parameters.push_str(format!(" {where_or_and} {property_name} NOT BETWEEN ${i} AND ${}", i + 1).as_str()),
			};
			//first_where_clause = false;
			parameter_values.push(Box::new(self.get_query_parameters().filters.active_to.unwrap().0));
			parameter_values.push(Box::new(self.get_query_parameters().filters.active_to.unwrap().1));
			i += 2;
		}



		if self.get_query_parameters().sort_property.is_some() {
			let direction: &str = match &self.get_query_parameters().sort_direction {
				Some(x) => (*x).into(),
				None => "DESC",
			};
			let property_name = get_property_name(self.get_query_parameters().sort_property.unwrap().to_string().as_str(), &table_name);
			parameters.push_str(format!(" ORDER BY {property_name} {direction}").as_str());
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

pub trait OldDbWriter<'a, T> {
	fn new(pool: &'a Pool, item: T) -> Self;
	async fn insert(self) -> Result<u32, Box<dyn Error>>;
	async fn replace(self) -> Result<(), Box<dyn Error>>;
}

pub trait OldDbDeleter<'a, T>: OldDbWriter<'a, T> {
	async fn delete(self) -> Result<(), Box<dyn Error>>;
}

pub trait DbWriter<'a, T> {
	fn new(pool: &'a Pool, item: T) -> Self;
	async fn insert(self) -> Result<Uuid, Box<dyn Error>>;
	async fn replace(self) -> Result<(), Box<dyn Error>>;
}

pub trait DbDeleter<'a, T>: OldDbWriter<'a, T> {
	async fn delete(self) -> Result<(), Box<dyn Error>>;
}

fn get_property_name(property: &str, table_name: &Option<String>) -> String {
	match table_name {
		Some(table_name) => return format!("{table_name}.{property}"),
		None => return property.to_string(),
	};
}

fn render_number_filter_mode(input: NumberFilterModes, where_or_and: &str, property_name: &str, i: i32) -> String {
	return match input {
		NumberFilterModes::Exact => format!(" {where_or_and} {property_name}=${i}"),
		NumberFilterModes::Not => format!(" {where_or_and} {property_name}!=${i}"),
		NumberFilterModes::Less => format!(" {where_or_and} {property_name}<${i}"),
		NumberFilterModes::More => format!(" {where_or_and} {property_name}>${i}"),
		NumberFilterModes::ExactOrAlsoNull => format!(" {where_or_and} ({property_name} IS NULL OR {property_name}=${i})"),
	};
}

fn render_string_filter_mode(input: (String, StringFilterModes), where_or_and: &str, property_name: &str, i: i32) -> (String, String) {
	return match input.1 {
		StringFilterModes::Exact => (format!(" {where_or_and} {property_name} ILIKE ${i}"), input.0),
		StringFilterModes::Contains => (format!(" {where_or_and} {property_name} ILIKE ${i}"), format!("%{}%", input.0)),
		StringFilterModes::BeginsWith => (format!(" {where_or_and} {property_name} ILIKE ${i}"), format!("{}%", input.0)),
		StringFilterModes::EndsWith => (format!(" {where_or_and} {property_name} ILIKE ${i}"), format!("%{}", input.0)),
		StringFilterModes::DoesntContain => (format!(" {where_or_and} {property_name} NOT ILIKE ${i}"), format!("%{}%", input.0)),
	};
}