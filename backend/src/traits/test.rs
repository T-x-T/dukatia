use super::*;

mod db_reader {
	use super::*;

	struct TestDbReader {
		query_parameters: QueryParameters,
	}

	impl From<tokio_postgres::Row> for TestDbReader {
    fn from(_: tokio_postgres::Row) -> Self {
			todo!()
    }
	}

	impl<'a> DbReader<'a, TestDbReader> for TestDbReader {
	fn new(_: &'a Pool) -> Self {
		todo!()
	}

	fn get_pool(&self) -> &Pool {
		todo!()
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(self, _: QueryParameters) -> Self {
		todo!()
	}

	async fn execute(self) -> Result<Vec<TestDbReader>, Box<dyn Error>> {
		todo!()
	}
}

	fn get_db_reader(query_parameters: QueryParameters) -> TestDbReader {
		return TestDbReader { query_parameters };
	}


	mod get_formatted_query_parameters {
		use super::*;
		
		#[test]
		fn nothing() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, "");
			assert_eq!(format!("{res_values:?}"), "[]");
		}
		
		#[test]
		fn skip_results() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: Some(10), sort_property: None, sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " OFFSET $1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}
		
		#[test]
		fn max_results() {
			let test = get_db_reader(QueryParameters { max_results: Some(10), skip_results: None, sort_property: None, sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " LIMIT $1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn max_and_skip_results() {
			let test = get_db_reader(QueryParameters { max_results: Some(10), skip_results: Some(20), sort_property: None, sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " OFFSET $1 LIMIT $2");
			assert_eq!(format!("{res_values:?}"), "[20, 10]");
		}

		#[test]
		fn sort_property() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Id), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_and_direction_desc() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Id), sort_direction: Some(SortDirection::Desc), filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_and_direction_asc() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Id), sort_direction: Some(SortDirection::Asc), filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY id ASC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_account_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::AccountId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY account_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_comment() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Comment), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY comment DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_currency_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::CurrencyId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY currency_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Id), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_recipient_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::RecipientId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY recipient_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_status() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Status), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY status DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_timestamp() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Timestamp), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY timestamp DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_user_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::UserId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY user_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}


		#[test]
		fn sort_property_total_amount() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::TotalAmount), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY total_amount DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_tag_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::TagId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY tag_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_name() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Name), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY name DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_symbol() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Symbol), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY symbol DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_minor_in_major() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::MinorInmajor), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY minor_in_major DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_parent_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::ParentId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY parent_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_balance() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Balance), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY balance DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_default_currency_id() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::DefaultCurrencyId), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY default_currency_id DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_description() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::Description), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY description DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_amount() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::IntAmount), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY amount DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn sort_property_value_per_unit() {
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: Some(FilterAndSortProperties::ValuePerUnit), sort_direction: None, filters: Filters::default() });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " ORDER BY value_per_unit DESC");
			assert_eq!(format!("{res_values:?}"), "[]");
		}

		#[test]
		fn filter_total_amount_exact() {
			let mut filters = Filters::default();
			filters.total_amount = Some((10, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE total_amount=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_total_amount_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.total_amount = Some((10, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (total_amount IS NULL OR total_amount=$1)");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_total_amount_not() {
			let mut filters = Filters::default();
			filters.total_amount = Some((10, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE total_amount!=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_total_amount_less() {
			let mut filters = Filters::default();
			filters.total_amount = Some((10, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE total_amount<$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_total_amount_more() {
			let mut filters = Filters::default();
			filters.total_amount = Some((10, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE total_amount>$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_asset_id_exact() {
			let mut filters = Filters::default();
			filters.asset_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE asset_id=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_asset_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.asset_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (asset_id IS NULL OR asset_id=$1)");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_asset_id_not() {
			let mut filters = Filters::default();
			filters.asset_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE asset_id!=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_asset_id_less() {
			let mut filters = Filters::default();
			filters.asset_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE asset_id<$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_asset_id_more() {
			let mut filters = Filters::default();
			filters.asset_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE asset_id>$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_user_id_exact() {
			let mut filters = Filters::default();
			filters.user_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE user_id=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_user_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.user_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (user_id IS NULL OR user_id=$1)");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_user_id_not() {
			let mut filters = Filters::default();
			filters.user_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE user_id!=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_user_id_less() {
			let mut filters = Filters::default();
			filters.user_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE user_id<$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_user_id_more() {
			let mut filters = Filters::default();
			filters.user_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE user_id>$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_currency_id_exact() {
			let mut filters = Filters::default();
			filters.currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE currency_id=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_currency_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.currency_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (currency_id IS NULL OR currency_id=$1)");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_currency_id_not() {
			let mut filters = Filters::default();
			filters.currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE currency_id!=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_currency_id_less() {
			let mut filters = Filters::default();
			filters.currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE currency_id<$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_currency_id_more() {
			let mut filters = Filters::default();
			filters.currency_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE currency_id>$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_account_id_exact() {
			let mut filters = Filters::default();
			filters.account_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE account_id=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_account_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.account_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (account_id IS NULL OR account_id=$1)");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_account_id_not() {
			let mut filters = Filters::default();
			filters.account_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE account_id!=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_account_id_less() {
			let mut filters = Filters::default();
			filters.account_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE account_id<$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_account_id_more() {
			let mut filters = Filters::default();
			filters.account_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE account_id>$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_recipient_id_exact() {
			let mut filters = Filters::default();
			filters.recipient_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE recipient_id=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_recipient_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.recipient_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (recipient_id IS NULL OR recipient_id=$1)");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_recipient_id_not() {
			let mut filters = Filters::default();
			filters.recipient_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE recipient_id!=$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_recipient_id_less() {
			let mut filters = Filters::default();
			filters.recipient_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE recipient_id<$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_recipient_id_more() {
			let mut filters = Filters::default();
			filters.recipient_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE recipient_id>$1");
			assert_eq!(format!("{res_values:?}"), format!("[{}]", Uuid::from_u128(10)));
		}

		#[test]
		fn filter_minor_in_major_exact() {
			let mut filters = Filters::default();
			filters.minor_in_major = Some((10, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE minor_in_major=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_minor_in_major_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.minor_in_major = Some((10, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (minor_in_major IS NULL OR minor_in_major=$1)");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_minor_in_major_not() {
			let mut filters = Filters::default();
			filters.minor_in_major = Some((10, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE minor_in_major!=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_minor_in_major_less() {
			let mut filters = Filters::default();
			filters.minor_in_major = Some((10, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE minor_in_major<$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_minor_in_major_more() {
			let mut filters = Filters::default();
			filters.minor_in_major = Some((10, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE minor_in_major>$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_parent_id_exact() {
			let mut filters = Filters::default();
			filters.parent_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE parent_id=$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_parent_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.parent_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (parent_id IS NULL OR parent_id=$1)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_parent_id_not() {
			let mut filters = Filters::default();
			filters.parent_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE parent_id!=$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_parent_id_less() {
			let mut filters = Filters::default();
			filters.parent_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE parent_id<$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_parent_id_more() {
			let mut filters = Filters::default();
			filters.parent_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE parent_id>$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_default_currency_id_exact() {
			let mut filters = Filters::default();
			filters.default_currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE default_currency_id=$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_default_currency_id_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.default_currency_id = Some((Uuid::from_u128(10), NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (default_currency_id IS NULL OR default_currency_id=$1)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_default_currency_id_not() {
			let mut filters = Filters::default();
			filters.default_currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE default_currency_id!=$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_default_currency_id_less() {
			let mut filters = Filters::default();
			filters.default_currency_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE default_currency_id<$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_default_currency_id_more() {
			let mut filters = Filters::default();
			filters.default_currency_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE default_currency_id>$1");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_balance_exact() {
			let mut filters = Filters::default();
			filters.balance = Some((10, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE balance=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_balance_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.balance = Some((10, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (balance IS NULL OR balance=$1)");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_balance_not() {
			let mut filters = Filters::default();
			filters.balance = Some((10, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE balance!=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_balance_less() {
			let mut filters = Filters::default();
			filters.balance = Some((10, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE balance<$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_balance_more() {
			let mut filters = Filters::default();
			filters.balance = Some((10, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE balance>$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_float_amount_exact() {
			let mut filters = Filters::default();
			filters.float_amount = Some((10.5, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount=$1");
			assert_eq!(format!("{res_values:?}"), "[10.5]");
		}

		#[test]
		fn filter_float_amount_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.float_amount = Some((10.5, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (amount IS NULL OR amount=$1)");
			assert_eq!(format!("{res_values:?}"), "[10.5]");
		}

		#[test]
		fn filter_float_amount_not() {
			let mut filters = Filters::default();
			filters.float_amount = Some((10.5, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount!=$1");
			assert_eq!(format!("{res_values:?}"), "[10.5]");
		}

		#[test]
		fn filter_float_amount_less() {
			let mut filters = Filters::default();
			filters.float_amount = Some((10.5, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount<$1");
			assert_eq!(format!("{res_values:?}"), "[10.5]");
		}

		#[test]
		fn filter_float_amount_more() {
			let mut filters = Filters::default();
			filters.float_amount = Some((10.5, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount>$1");
			assert_eq!(format!("{res_values:?}"), "[10.5]");
		}

		#[test]
		fn filter_int_amount_exact() {
			let mut filters = Filters::default();
			filters.int_amount = Some((10, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_int_amount_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.int_amount = Some((10, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (amount IS NULL OR amount=$1)");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_int_amount_not() {
			let mut filters = Filters::default();
			filters.int_amount = Some((10, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount!=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_int_amount_less() {
			let mut filters = Filters::default();
			filters.int_amount = Some((10, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount<$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_int_amount_more() {
			let mut filters = Filters::default();
			filters.int_amount = Some((10, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE amount>$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_value_per_unit_exact() {
			let mut filters = Filters::default();
			filters.value_per_unit = Some((10, NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE value_per_unit=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_value_per_unit_exact_or_also_null() {
			let mut filters = Filters::default();
			filters.value_per_unit = Some((10, NumberFilterModes::ExactOrAlsoNull));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE (value_per_unit IS NULL OR value_per_unit=$1)");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_value_per_unit_not() {
			let mut filters = Filters::default();
			filters.value_per_unit = Some((10, NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE value_per_unit!=$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_value_per_unit_less() {
			let mut filters = Filters::default();
			filters.value_per_unit = Some((10, NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE value_per_unit<$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_value_per_unit_more() {
			let mut filters = Filters::default();
			filters.value_per_unit = Some((10, NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE value_per_unit>$1");
			assert_eq!(format!("{res_values:?}"), "[10]");
		}

		#[test]
		fn filter_tag_id_exact() {
			let mut filters = Filters::default();
			filters.tag_id = Some((Uuid::from_u128(10), NumberFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE $1 = ANY(tags)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_tag_id_not() {
			let mut filters = Filters::default();
			filters.tag_id = Some((Uuid::from_u128(10), NumberFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE NOT $1 = ANY(tags)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_tag_id_less() {
			let mut filters = Filters::default();
			filters.tag_id = Some((Uuid::from_u128(10), NumberFilterModes::Less));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE $1 > ANY(tags)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_tag_id_more() {
			let mut filters = Filters::default();
			filters.tag_id = Some((Uuid::from_u128(10), NumberFilterModes::More));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE $1 < ANY(tags)");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-00000000000a]");
		}

		#[test]
		fn filter_rollover_is_true() {
			let mut filters = Filters::default();
			filters.rollover = Some((true, BoolFilterModes::Is));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE rollover=$1");
			assert_eq!(format!("{res_values:?}"), "[true]");
		}

		#[test]
		fn filter_rollover_is_false() {
			let mut filters = Filters::default();
			filters.rollover = Some((false, BoolFilterModes::Is));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE rollover=$1");
			assert_eq!(format!("{res_values:?}"), "[false]");
		}

		#[test]
		fn filter_rollover_not_true() {
			let mut filters = Filters::default();
			filters.rollover = Some((true, BoolFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE rollover!=$1");
			assert_eq!(format!("{res_values:?}"), "[true]");
		}

		#[test]
		fn filter_rollover_not_false() {
			let mut filters = Filters::default();
			filters.rollover = Some((false, BoolFilterModes::Not));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE rollover!=$1");
			assert_eq!(format!("{res_values:?}"), "[false]");
		}

		#[test]
		fn filter_comment_exact() {
			let mut filters = Filters::default();
			filters.comment = Some(("test".to_string(), StringFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE comment ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test\"]");
		}

		#[test]
		fn filter_comment_contains() {
			let mut filters = Filters::default();
			filters.comment = Some(("test".to_string(), StringFilterModes::Contains));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE comment ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_comment_begins_with() {
			let mut filters = Filters::default();
			filters.comment = Some(("test".to_string(), StringFilterModes::BeginsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE comment ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test%\"]");
		}

		#[test]
		fn filter_comment_ends_with() {
			let mut filters = Filters::default();
			filters.comment = Some(("test".to_string(), StringFilterModes::EndsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE comment ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test\"]");
		}

		#[test]
		fn filter_comment_doesnt_contain() {
			let mut filters = Filters::default();
			filters.comment = Some(("test".to_string(), StringFilterModes::DoesntContain));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE comment NOT ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_name_exact() {
			let mut filters = Filters::default();
			filters.name = Some(("test".to_string(), StringFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE name ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test\"]");
		}

		#[test]
		fn filter_name_contains() {
			let mut filters = Filters::default();
			filters.name = Some(("test".to_string(), StringFilterModes::Contains));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE name ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_name_begins_with() {
			let mut filters = Filters::default();
			filters.name = Some(("test".to_string(), StringFilterModes::BeginsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE name ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test%\"]");
		}

		#[test]
		fn filter_name_ends_with() {
			let mut filters = Filters::default();
			filters.name = Some(("test".to_string(), StringFilterModes::EndsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE name ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test\"]");
		}

		#[test]
		fn filter_name_doesnt_contain() {
			let mut filters = Filters::default();
			filters.name = Some(("test".to_string(), StringFilterModes::DoesntContain));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE name NOT ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_symbol_exact() {
			let mut filters = Filters::default();
			filters.symbol = Some(("test".to_string(), StringFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE symbol ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test\"]");
		}

		#[test]
		fn filter_symbol_contains() {
			let mut filters = Filters::default();
			filters.symbol = Some(("test".to_string(), StringFilterModes::Contains));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE symbol ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_symbol_begins_with() {
			let mut filters = Filters::default();
			filters.symbol = Some(("test".to_string(), StringFilterModes::BeginsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE symbol ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test%\"]");
		}

		#[test]
		fn filter_symbol_ends_with() {
			let mut filters = Filters::default();
			filters.symbol = Some(("test".to_string(), StringFilterModes::EndsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE symbol ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test\"]");
		}

		#[test]
		fn filter_symbol_doesnt_contain() {
			let mut filters = Filters::default();
			filters.symbol = Some(("test".to_string(), StringFilterModes::DoesntContain));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE symbol NOT ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_description_exact() {
			let mut filters = Filters::default();
			filters.description = Some(("test".to_string(), StringFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test\"]");
		}

		#[test]
		fn filter_description_contains() {
			let mut filters = Filters::default();
			filters.description = Some(("test".to_string(), StringFilterModes::Contains));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_description_begins_with() {
			let mut filters = Filters::default();
			filters.description = Some(("test".to_string(), StringFilterModes::BeginsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"test%\"]");
		}

		#[test]
		fn filter_description_ends_with() {
			let mut filters = Filters::default();
			filters.description = Some(("test".to_string(), StringFilterModes::EndsWith));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test\"]");
		}

		#[test]
		fn filter_description_doesnt_contain() {
			let mut filters = Filters::default();
			filters.description = Some(("test".to_string(), StringFilterModes::DoesntContain));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description NOT ILIKE $1");
			assert_eq!(format!("{res_values:?}"), "[\"%test%\"]");
		}

		#[test]
		fn filter_time_range_between() {
			let mut filters = Filters::default();
			filters.time_range = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Between));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE timestamp BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_time_range_outside() {
			let mut filters = Filters::default();
			filters.time_range = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Outside));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE timestamp NOT BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_active_from_between() {
			let mut filters = Filters::default();
			filters.active_from = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Between));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE active_from BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_active_from_outside() {
			let mut filters = Filters::default();
			filters.active_from = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Outside));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE active_from NOT BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_active_to_between() {
			let mut filters = Filters::default();
			filters.active_to = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Between));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE active_to BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_active_to_outside() {
			let mut filters = Filters::default();
			filters.active_to = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Outside));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE active_to NOT BETWEEN $1 AND $2");
			assert_eq!(format!("{res_values:?}"), "[-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn filter_description_and_time_range() {
			let mut filters = Filters::default();
			filters.time_range = Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Outside));
			filters.description = Some(("test".to_string(), StringFilterModes::Exact));
			let test = get_db_reader(QueryParameters { max_results: None, skip_results: None, sort_property: None, sort_direction: None, filters });
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE description ILIKE $1 AND timestamp NOT BETWEEN $2 AND $3");
			assert_eq!(format!("{res_values:?}"), "[\"test\", -262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z]");
		}

		#[test]
		fn all() {
			let test = get_db_reader(
				QueryParameters { 
					max_results: Some(100),
					skip_results: Some(50),
					sort_property: Some(FilterAndSortProperties::Id),
					sort_direction: Some(SortDirection::Asc),
					filters: Filters {
						id: Some((Uuid::nil(), NumberFilterModes::Exact)),
						total_amount: Some((2, NumberFilterModes::Exact)),
						asset_id: Some((Uuid::from_u128(3), NumberFilterModes::Exact)),
						user_id: Some((Uuid::from_u128(4), NumberFilterModes::Exact)),
						currency_id: Some((Uuid::from_u128(5), NumberFilterModes::Exact)),
						account_id: Some((Uuid::from_u128(6), NumberFilterModes::Exact)),
						recipient_id: Some((Uuid::from_u128(7), NumberFilterModes::Exact)),
						tag_id: Some((Uuid::from_u128(8), NumberFilterModes::Exact)),
						comment: Some(("9".to_string(), StringFilterModes::Exact)),
						time_range: Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Between)),
						name: Some(("10".to_string(), StringFilterModes::Exact)),
						symbol: Some(("11".to_string(), StringFilterModes::Exact)),
						minor_in_major: Some((12, NumberFilterModes::Exact)),
						parent_id: Some((Uuid::from_u128(13), NumberFilterModes::Exact)),
						balance: Some((14, NumberFilterModes::Exact)),
						default_currency_id: Some((Uuid::from_u128(15), NumberFilterModes::Exact)),
						description: Some(("16".to_string(), StringFilterModes::Exact)),
						float_amount: Some((17.5, NumberFilterModes::Exact)),
						int_amount: Some((17, NumberFilterModes::Exact)),
						value_per_unit: Some((18, NumberFilterModes::Exact)),
						rollover: Some((true, BoolFilterModes::Is)),
						active_from: Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Between)),
						active_to: Some((DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC, TimeRangeFilterModes::Outside)),
					}
				}
			);
	
			let (res_string, res_values) = test.get_formatted_query_parameters(None);
			
			assert_eq!(res_string, " WHERE id=$1 AND total_amount=$2 AND asset_id=$3 AND user_id=$4 AND currency_id=$5 AND account_id=$6 AND recipient_id=$7 AND minor_in_major=$8 AND parent_id=$9 AND default_currency_id=$10 AND balance=$11 AND amount=$12 AND amount=$13 AND value_per_unit=$14 AND $15 = ANY(tags) AND rollover=$16 AND comment ILIKE $17 AND name ILIKE $18 AND symbol ILIKE $19 AND description ILIKE $20 AND timestamp BETWEEN $21 AND $22 AND active_from BETWEEN $23 AND $24 AND active_to NOT BETWEEN $25 AND $26 ORDER BY id ASC OFFSET $27 LIMIT $28");
			assert_eq!(format!("{res_values:?}"), "[00000000-0000-0000-0000-000000000000, 2, 00000000-0000-0000-0000-000000000003, 00000000-0000-0000-0000-000000000004, 00000000-0000-0000-0000-000000000005, 00000000-0000-0000-0000-000000000006, 00000000-0000-0000-0000-000000000007, 12, 00000000-0000-0000-0000-00000000000d, 00000000-0000-0000-0000-00000000000f, 14, 17.5, 17, 18, 00000000-0000-0000-0000-000000000008, true, \"9\", \"10\", \"11\", \"16\", -262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z, -262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z, -262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z, 50, 100]");
		}
	}
}