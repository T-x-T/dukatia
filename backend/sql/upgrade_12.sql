CREATE OR REPLACE VIEW public.account_data
    AS
     SELECT a.id,
    a.name,
    a.default_currency_id,
    a.user_id,
    array_agg(DISTINCT t.tag_id) AS tags,
    sum(tr.total_amount)::bigint AS balance
   FROM accounts a
     LEFT JOIN account_tags t ON a.id = t.account_id
     LEFT JOIN transaction_data tr ON a.id = tr.account_id
  GROUP BY a.id
  ORDER BY a.id;

CREATE OR REPLACE VIEW public.recipient_data
  AS
	 SELECT r.id,
    r.name,
    r.user_id,
    array_agg(t.tag_id) AS tags
   FROM recipients r
     LEFT JOIN recipient_tags t ON r.id = t.recipient_id
  GROUP BY r.id
  ORDER BY r.id;