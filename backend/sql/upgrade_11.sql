CREATE OR REPLACE VIEW public.asset_data
    AS
     SELECT a.id,
    a.name,
    a.description,
    a.user_id,
    a.currency_id,
    array_agg(t.tag_id) AS tags,
    CASE WHEN aa.amount IS NULL THEN 0 ELSE aa.amount END,
	CASE WHEN av.value_per_unit IS NULL THEN 0 ELSE av.value_per_unit END
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
  GROUP BY a.id, aa.amount, av.value_per_unit
  ORDER BY a.id;