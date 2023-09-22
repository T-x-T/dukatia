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
          WHERE p.transaction_id = tr.id) AS total_amount,
   c.minor_in_major,
   c.symbol
   FROM transactions tr
     LEFT JOIN transaction_tags t ON tr.id = t.transaction_id
     LEFT JOIN asset_transactions at ON at.transaction_id = tr.id
     LEFT JOIN assets a ON a.id = at.asset_id
	 LEFT JOIN currencies c ON c.id = tr.currency_id
  GROUP BY tr.id, a.id, c.minor_in_major, c.symbol
  ORDER BY tr.id;