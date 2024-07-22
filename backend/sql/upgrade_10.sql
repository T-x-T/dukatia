CREATE OR REPLACE VIEW public.transaction_data
 AS
 SELECT tr.id,
    tr.account_id,
    tr.currency_id,
    tr.recipient_id,
    tr.status,
    tr.user_id,
    tr."timestamp",
    tr.comment,
    array_agg(t.tag_id) AS tags,
    a.id AS asset_id,
    a.name AS asset_name,
    a.description AS asset_description,
    ( SELECT array_agg(p.id) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = tr.id) AS transaction_position_ids,
    ( SELECT array_agg(p.amount) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = tr.id) AS transaction_position_amounts,
    ( SELECT array_agg(p.comment) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = tr.id) AS transaction_position_comments,
    ( SELECT array_agg(p.tag_id) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = tr.id) AS transaction_position_tag_ids,
    ( SELECT sum(p.amount) AS sum
           FROM transaction_positions p
          WHERE p.transaction_id = tr.id) AS total_amount
   FROM transactions tr
     LEFT JOIN transaction_tags t ON tr.id = t.transaction_id
     LEFT JOIN asset_transactions at ON at.transaction_id = tr.id
     LEFT JOIN assets a ON a.id = at.asset_id
  GROUP BY tr.id, a.id
  ORDER BY tr.id;

DROP VIEW public.account_data;
CREATE OR REPLACE VIEW public.account_data
    AS
     SELECT a.id,
    a.name,
    a.default_currency_id,
    a.user_id,
    array_agg(DISTINCT t.tag_id) AS tags,
    SUM(tr.total_amount)::bigint as balance
   FROM accounts a
     LEFT JOIN account_tags t ON a.id = t.account_id
	 LEFT JOIN transaction_data tr ON a.id = tr.account_id
  GROUP BY a.id;

CREATE OR REPLACE VIEW public.deep_accounts
    AS
     SELECT a.id,
    a.name,
    c.id AS default_currency_id,
    c.minor_in_mayor AS default_currency_minor_in_mayor,
    c.name AS default_currency_name,
    c.symbol AS default_currency_symbol,
    u.id AS user_id,
    u.name AS user_name,
    u.superuser AS user_superuser,
    array_agg(DISTINCT t.id) AS tag_ids,
    array_agg(DISTINCT t.name) AS tag_names,
    array_agg(DISTINCT t.parent_id) AS tag_parent_ids,
    array_agg(DISTINCT t.parent_name) AS tag_parent_names,
    array_agg(DISTINCT t.parent_parent_id) AS tag_parent_parent_ids,
    array_agg(DISTINCT t.parent_user_id) AS tag_parent_user_ids,
    array_agg(DISTINCT t.user_id) AS tag_user_ids,
    array_agg(DISTINCT t.user_name) AS tag_user_names,
    array_agg(DISTINCT t.user_superuser) AS tag_user_superusers,
    SUM(tr.total_amount)::bigint as balance
   FROM accounts a
     LEFT JOIN currencies c ON a.default_currency_id = c.id
     LEFT JOIN users u ON a.user_id = u.id
     LEFT JOIN account_tags at ON a.id = at.account_id
     LEFT JOIN deep_tags t ON at.tag_id = t.id
	 LEFT JOIN transaction_data tr ON a.id = tr.account_id
  GROUP BY a.id, c.id, u.id
  ORDER BY a.id;