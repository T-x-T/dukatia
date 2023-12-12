ALTER TABLE IF EXISTS public.charts DROP COLUMN IF EXISTS text_template;

ALTER TABLE IF EXISTS public.charts
	ADD COLUMN only_positive boolean;

ALTER TABLE IF EXISTS public.charts
	ADD COLUMN only_negative boolean;

UPDATE public.charts
	SET filter_collection = 'get_single_asset_total_value_over_time'
	WHERE filter_collection LIKE 'asset_total_value';

UPDATE public.charts
	SET filter_collection = 'get_single_asset_single_value_over_time'
	WHERE filter_collection LIKE 'asset_single_value';

UPDATE public.charts
	SET filter_collection = 'get_single_asset_amount_over_time'
	WHERE filter_collection LIKE 'asset_amount';

UPDATE public.charts
	SET filter_collection = 'get_single_asset_amount_over_time'
	WHERE filter_collection LIKE 'asset_amount';

UPDATE public.charts
	SET filter_collection = 'get_per_recipient_over_time'
	WHERE filter_collection LIKE 'recipients';

UPDATE public.charts
	SET filter_collection = 'get_per_account_over_time'
	WHERE filter_collection LIKE 'accounts';

UPDATE public.charts
	SET filter_collection = 'get_per_currency_over_time'
	WHERE filter_collection LIKE 'currencies';

UPDATE public.charts
	SET filter_collection = 'get_earning_spending_net_over_time'
	WHERE filter_collection LIKE 'earning_spending_net';

UPDATE public.charts
	SET filter_collection = 'get_per_tag_over_time'
	WHERE filter_collection LIKE 'tags';
