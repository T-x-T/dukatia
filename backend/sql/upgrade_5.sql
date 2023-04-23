ALTER TABLE IF EXISTS public."AccessTokens"
    RENAME TO access_tokens;
ALTER TABLE IF EXISTS public.access_tokens
    RENAME "user" TO user_id;
ALTER TABLE IF EXISTS public.access_tokens
    RENAME CONSTRAINT "user" TO user_id;
ALTER TABLE IF EXISTS public.access_tokens
    RENAME CONSTRAINT "accessTokens_pkey" TO access_tokens_pkey;


ALTER TABLE IF EXISTS public."AccountTags"
    RENAME TO account_tags;
ALTER TABLE IF EXISTS public.account_tags
    RENAME account TO account_id;
ALTER TABLE IF EXISTS public.account_tags
    RENAME tag TO tag_id;
ALTER TABLE IF EXISTS public.account_tags
    RENAME CONSTRAINT account TO account_id;
ALTER TABLE IF EXISTS public.account_tags
    RENAME CONSTRAINT tag TO tag_id;
ALTER TABLE IF EXISTS public.account_tags
    RENAME CONSTRAINT "AccountTags_pkey" TO account_tags_pkey;


ALTER TABLE IF EXISTS public."Accounts"
    RENAME TO accounts;
ALTER TABLE IF EXISTS public.accounts
    RENAME defaultcurrency TO default_currency_id;
ALTER TABLE IF EXISTS public.accounts
    RENAME "user" TO user_id;
ALTER TABLE IF EXISTS public.accounts
    RENAME CONSTRAINT defaultcurrency TO default_currency_id;
ALTER TABLE IF EXISTS public.accounts
    RENAME CONSTRAINT "user" TO user_id;
ALTER TABLE IF EXISTS public.accounts
    RENAME CONSTRAINT "Account_pkey" TO accounts_pkey;


ALTER TABLE IF EXISTS public."AssetAmounts"
    RENAME TO asset_amounts;
ALTER TABLE IF EXISTS public.asset_amounts
    RENAME "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_amounts
    RENAME CONSTRAINT "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_amounts
    RENAME CONSTRAINT "AssetAmounts_pkey" TO asset_amounts_pkey;


ALTER TABLE IF EXISTS public."AssetTags"
    RENAME TO asset_tags;
ALTER TABLE IF EXISTS public.asset_tags
    RENAME "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_tags
    RENAME "tagId" TO tag_id;
ALTER TABLE IF EXISTS public.asset_tags
    RENAME CONSTRAINT "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_tags
    RENAME CONSTRAINT "tagId" TO tag_id;
ALTER TABLE IF EXISTS public.asset_tags
    RENAME CONSTRAINT "AssetTags_pkey" TO asset_tags_pkey;


ALTER TABLE IF EXISTS public."AssetTransactions"
    RENAME TO asset_transactions;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME "transactionId" TO transaction_id;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME CONSTRAINT "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME CONSTRAINT "transactionId" TO transaction_id;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME CONSTRAINT "transactionIdUnique" TO transaction_id_unique;
ALTER TABLE IF EXISTS public.asset_transactions
    RENAME CONSTRAINT "AssetTransactions_pkey" TO asset_transactions_pkey;


ALTER TABLE IF EXISTS public."AssetValuations"
    RENAME TO asset_valuations;
ALTER TABLE IF EXISTS public.asset_valuations
		RENAME "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_valuations
		RENAME "valuePerUnit" TO value_per_unit;
ALTER TABLE IF EXISTS public.asset_valuations
		RENAME CONSTRAINT "assetId" TO asset_id;
ALTER TABLE IF EXISTS public.asset_valuations
		RENAME CONSTRAINT "AssetValuations_pkey" TO asset_valuations_pkey;


ALTER TABLE IF EXISTS public."Assets"
    RENAME TO assets;
ALTER TABLE IF EXISTS public.assets
		RENAME "currencyId" TO currency_id;
ALTER TABLE IF EXISTS public.assets
		RENAME "userId" TO user_id;
ALTER TABLE IF EXISTS public.assets
		RENAME CONSTRAINT "currencyId" TO currency_id;
ALTER TABLE IF EXISTS public.assets
		RENAME CONSTRAINT "userId" TO user_id;
ALTER TABLE IF EXISTS public.assets
		RENAME CONSTRAINT "Assets_pkey" TO assets_pkey;


ALTER TABLE IF EXISTS public."Currencies"
    RENAME TO currencies;
ALTER TABLE IF EXISTS public.currencies
		RENAME minorinmayor TO minor_in_mayor;
ALTER TABLE IF EXISTS public.currencies
		RENAME CONSTRAINT "Currency_pkey" TO currencies_pkey;


ALTER TABLE IF EXISTS public."Meta"
    RENAME TO meta;


ALTER TABLE IF EXISTS public."RecipientTags"
    RENAME TO recipient_tags;
ALTER TABLE IF EXISTS public.recipient_tags
		RENAME recipient TO recipient_id;
ALTER TABLE IF EXISTS public.recipient_tags
		RENAME tag TO tag_id;
ALTER TABLE IF EXISTS public.recipient_tags
		RENAME CONSTRAINT recipient TO recipient_id;
ALTER TABLE IF EXISTS public.recipient_tags
		RENAME CONSTRAINT tag TO tag_id;
ALTER TABLE IF EXISTS public.recipient_tags
		RENAME CONSTRAINT "RecipientTags_pkey" TO recipient_tags_pkey;


ALTER TABLE IF EXISTS public."Recipients"
    RENAME TO recipients;
ALTER TABLE IF EXISTS public.recipients
		RENAME "user" TO user_id;
ALTER TABLE IF EXISTS public.recipients
		RENAME CONSTRAINT "user" TO user_id;
ALTER TABLE IF EXISTS public.recipients
		RENAME CONSTRAINT "Recipients_pkey" TO recipients_pkey;


ALTER TABLE IF EXISTS public."Tags"
    RENAME TO tags;
ALTER TABLE IF EXISTS public.tags
		RENAME CONSTRAINT "Tag_pkey" TO tag_pkey;
ALTER TABLE IF EXISTS public.tags
		RENAME "user" TO user_id;
ALTER TABLE IF EXISTS public.tags
		RENAME parent TO parent_id;
ALTER TABLE IF EXISTS public.tags
		RENAME CONSTRAINT "user" TO user_id;


ALTER TABLE IF EXISTS public."TransactionTags"
    RENAME TO transaction_tags;
ALTER TABLE IF EXISTS public.transaction_tags
		RENAME tag TO tag_id;
ALTER TABLE IF EXISTS public.transaction_tags
		RENAME "transaction" TO transaction_id;
ALTER TABLE IF EXISTS public.transaction_tags
		RENAME CONSTRAINT tag TO tag_id;
ALTER TABLE IF EXISTS public.transaction_tags
		RENAME CONSTRAINT "transaction" TO transaction_id;
ALTER TABLE IF EXISTS public.transaction_tags
		RENAME CONSTRAINT "TransactionTags_pkey" TO transaction_tags_pkey;


ALTER TABLE IF EXISTS public."Transactions"
    RENAME TO transactions;
ALTER TABLE IF EXISTS public.transactions
		RENAME account TO account_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME currency TO currency_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME recipient TO recipient_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME "user" TO user_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME CONSTRAINT account TO account_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME CONSTRAINT currency TO currency_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME CONSTRAINT recipient TO recipient_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME CONSTRAINT "user" TO user_id;
ALTER TABLE IF EXISTS public.transactions
		RENAME CONSTRAINT "Transactions_pkey" TO transactions_pkey;


ALTER TABLE IF EXISTS public."Users"
    RENAME TO users;
ALTER TABLE IF EXISTS public.users
		RENAME CONSTRAINT "Users_id_key" TO users_id_key;
ALTER TABLE IF EXISTS public.users
		RENAME CONSTRAINT "Users_pkey" TO users_pkey;



DROP VIEW public."AssetData";
CREATE OR REPLACE VIEW public."asset_data"
    AS
     SELECT a.id,
    a.name,
    a.description,
    a."user_id",
    a."currency_id",
    array_agg(t."tag_id") AS tags,
    aa.amount,
    av."value_per_unit"
   FROM "assets" a
     LEFT JOIN "asset_amounts" aa ON a.id = aa."asset_id" AND aa."timestamp" = (( SELECT max("asset_amounts"."timestamp") AS max
           FROM "asset_amounts"
          WHERE "asset_amounts"."asset_id" = a.id
          GROUP BY "asset_amounts"."asset_id"))
     LEFT JOIN "asset_valuations" av ON a.id = av."asset_id" AND av."timestamp" = (( SELECT max("asset_valuations"."timestamp") AS max
           FROM "asset_valuations"
          WHERE "asset_valuations"."asset_id" = a.id
          GROUP BY "asset_valuations"."asset_id"))
     LEFT JOIN "asset_tags" t ON a.id = t."asset_id"
  GROUP BY a.id, aa.amount, av."value_per_unit"
  ORDER BY a.id;


DROP VIEW public."AssetValuationHistory";
CREATE OR REPLACE VIEW public.asset_valuation_history
		AS
				SELECT COALESCE(aa."asset_id", av."asset_id") AS asset_id,
					COALESCE(aa."timestamp", av."timestamp") AS "timestamp",
					aa.amount,
					av."value_per_unit"
					FROM "asset_amounts" aa
						FULL JOIN "asset_valuations" av ON aa."asset_id" = av."asset_id" AND aa."timestamp" = av."timestamp"
					ORDER BY (COALESCE(aa."asset_id", av."asset_id")), (COALESCE(aa."timestamp", av."timestamp"));


DROP VIEW public."TransactionData";
CREATE OR REPLACE VIEW public.transaction_data
		AS
				SELECT tr.id,
						tr.account_id,
						tr.currency_id,
						tr.recipient_id,
						tr.status,
						tr.user_id,
						tr."timestamp",
						tr.amount,
						tr.comment,
						array_agg(t.tag_id) AS tags,
						a.id AS "asset_id",
						a.name AS "asset_name",
						a.description AS "asset_description"
					FROM "transactions" tr
						LEFT JOIN "transaction_tags" t ON tr.id = t.transaction_id
						LEFT JOIN "asset_transactions" at ON at."transaction_id" = tr.id
						LEFT JOIN "assets" a ON a.id = at."asset_id"
					GROUP BY tr.id, a.id
					ORDER BY tr.id;