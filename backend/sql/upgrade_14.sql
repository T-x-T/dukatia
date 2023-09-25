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


CREATE OR REPLACE VIEW public.asset_data
    AS
     SELECT a.id,
    a.name,
    a.description,
    a.user_id,
    a.currency_id,
    array_agg(t.tag_id) AS tags,
        CASE
            WHEN aa.amount IS NULL THEN 0::double precision
            ELSE aa.amount
        END AS amount,
        CASE
            WHEN av.value_per_unit IS NULL THEN 0
            ELSE av.value_per_unit
        END AS value_per_unit,
   	c.minor_in_major,
	c.symbol
   FROM assets a
     LEFT JOIN asset_amounts aa ON a.id = aa.asset_id AND aa."timestamp" = (( SELECT max(asset_amounts."timestamp") AS max
           FROM asset_amounts
          WHERE asset_amounts.asset_id = a.id
          GROUP BY asset_amounts.asset_id))
     LEFT JOIN asset_valuations av ON a.id = av.asset_id AND av."timestamp" = (( SELECT max(asset_valuations."timestamp") AS max
           FROM asset_valuations
          WHERE asset_valuations.asset_id = a.id
          GROUP BY asset_valuations.asset_id))
     LEFT JOIN asset_tags t ON a.id = t.asset_id
	 LEFT JOIN currencies c ON a.currency_id = c.id
  GROUP BY a.id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.id;

  CREATE OR REPLACE VIEW public.asset_valuation_history
    AS
     SELECT COALESCE(aa.asset_id, av.asset_id) AS asset_id,
    COALESCE(aa."timestamp", av."timestamp") AS "timestamp",
    aa.amount,
    av.value_per_unit,
	c.minor_in_major,
	c.symbol
   FROM asset_amounts aa
     FULL JOIN asset_valuations av ON aa.asset_id = av.asset_id AND aa."timestamp" = av."timestamp"
	 LEFT JOIN assets a ON aa.asset_id = a.id
	 LEFT JOIN currencies c ON a.currency_id = c.id
  ORDER BY (COALESCE(aa.asset_id, av.asset_id)), (COALESCE(aa."timestamp", av."timestamp"));