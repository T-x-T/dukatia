CREATE OR REPLACE VIEW public.deep_tags
 AS
  SELECT t.id,
      t.name,
      u.id AS user_id,
      u.name AS user_name,
      u.superuser AS user_superuser,
      pt.id AS parent_id,
      pt.name AS parent_name,
      pt.user_id AS parent_user_id,
      pt.parent_id AS parent_parent_id
    FROM tags t
      LEFT JOIN users u ON t.user_id = u.id
      LEFT JOIN tags pt ON t.parent_id = pt.id
    ORDER BY t.id;

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
    array_agg(t.id) AS tag_ids,
    array_agg(t.name) AS tag_names,
    array_agg(t.parent_id) AS tag_parent_ids,
    array_agg(t.parent_name) AS tag_parent_names,
    array_agg(t.parent_parent_id) AS tag_parent_parent_ids,
    array_agg(t.parent_user_id) AS tag_parent_user_ids,
    array_agg(t.user_id) AS tag_user_ids,
    array_agg(t.user_name) AS tag_user_names,
    array_agg(t.user_superuser) AS tag_user_superusers
   FROM accounts a
     LEFT JOIN currencies c ON a.default_currency_id = c.id
     LEFT JOIN users u ON a.user_id = u.id
     LEFT JOIN account_tags at ON a.id = at.account_id
     LEFT JOIN deep_tags t ON at.tag_id = t.id
  GROUP BY a.id, c.id, u.id
  ORDER BY a.id;

CREATE OR REPLACE VIEW public.deep_recipients
 AS
 SELECT r.id,
    r.name,
    u.id AS user_id,
    u.name AS user_name,
    u.superuser AS user_superuser,
    array_agg(t.id) AS tag_ids,
    array_agg(t.name) AS tag_names,
    array_agg(t.parent_id) AS tag_parent_ids,
    array_agg(t.parent_name) AS tag_parent_names,
    array_agg(t.parent_parent_id) AS tag_parent_parent_ids,
    array_agg(t.parent_user_id) AS tag_parent_user_ids,
    array_agg(t.user_id) AS tag_user_ids,
    array_agg(t.user_name) AS tag_user_names,
    array_agg(t.user_superuser) AS tag_user_superusers
   FROM recipients r
     LEFT JOIN users u ON r.user_id = u.id
     LEFT JOIN recipient_tags rt ON r.id = rt.recipient_id
     LEFT JOIN deep_tags t ON rt.tag_id = t.id
  GROUP BY t.id, u.id, r.id
  ORDER BY r.id;


CREATE OR REPLACE VIEW public.deep_assets
 AS
 SELECT a.id,
    a.name,
    a.description,
    a.value_per_unit,
    a.amount,
    c.id AS currency_id,
    c.minor_in_mayor AS currency_minor_in_mayor,
    c.name AS currency_name,
    c.symbol AS currency_symbol,
    u.id AS user_id,
    u.name AS user_name,
    u.superuser AS user_superuser,
    array_agg(t.id) AS tag_ids,
    array_agg(t.name) AS tag_names,
    array_agg(t.parent_id) AS tag_parent_ids,
    array_agg(t.parent_name) AS tag_parent_names,
    array_agg(t.parent_parent_id) AS tag_parent_parent_ids,
    array_agg(t.parent_user_id) AS tag_parent_user_ids,
    array_agg(t.user_id) AS tag_user_ids,
    array_agg(t.user_name) AS tag_user_names,
    array_agg(t.user_superuser) AS tag_user_superusers
   FROM asset_data a
     LEFT JOIN currencies c ON a.currency_id = c.id
     LEFT JOIN users u ON a.user_id = u.id
     LEFT JOIN asset_tags at ON a.id = at.asset_id
     LEFT JOIN deep_tags t ON at.tag_id = t.id
  GROUP BY a.id, a.name, a.description, a.value_per_unit, a.amount, c.id, u.id
  ORDER BY a.id;

    CREATE OR REPLACE VIEW public.deep_transactions
 AS
 SELECT t.id,
    t.status,
    t."timestamp",
    t.amount,
    t.comment,
    c.id AS currency_id,
    c.minor_in_mayor AS currency_minor_in_mayor,
    c.name AS currency_name,
    c.symbol AS currency_symbol,
    u.id AS user_id,
    u.name AS user_name,
    u.superuser AS user_superuser,
    a.id AS account_id,
    a.name AS account_name,
    a.default_currency_id AS account_default_currency_id,
    a.default_currency_name AS account_default_currency_name,
    a.default_currency_minor_in_mayor AS account_default_currency_minor_in_mayor,
    a.default_currency_symbol AS account_default_currency_symbol,
    a.user_id AS account_user_id,
    a.user_name AS account_user_name,
    a.user_superuser AS account_user_superuser,
    a.tag_ids AS account_tag_ids,
    a.tag_names AS account_tag_names,
    a.tag_parent_ids AS account_tag_parent_ids,
    a.tag_parent_names AS account_tag_parent_names,
    a.tag_parent_parent_ids AS account_tag_parent_parent_ids,
    a.tag_parent_user_ids AS account_tag_parent_user_ids,
    a.tag_user_ids AS account_tag_user_ids,
    a.tag_user_names AS account_tag_user_names,
    a.tag_user_superusers AS account_tag_superusers,
    r.id AS recipient_id,
    r.name AS recipient_name,
    r.user_id AS recipient_user_id,
    r.user_name AS recipient_user_name,
    r.user_superuser AS recipient_user_superuser,
    r.tag_ids AS recipient_tag_ids,
    r.tag_names AS recipient_tag_names,
    r.tag_parent_ids AS recipient_tag_parent_ids,
    r.tag_parent_names AS recipient_tag_parent_names,
    r.tag_parent_parent_ids AS recipient_tag_parent_parent_ids,
    r.tag_parent_user_ids AS recipient_tag_parent_user_ids,
    r.tag_user_ids AS recipient_tag_user_ids,
    r.tag_user_names AS recipient_tag_user_names,
    r.tag_user_superusers AS recipient_tag_superusers,
    array_agg(tag.id) AS tag_ids,
    array_agg(tag.name) AS tag_names,
    array_agg(tag.parent_id) AS tag_parent_ids,
    array_agg(tag.parent_name) AS tag_parent_names,
    array_agg(tag.parent_parent_id) AS tag_parent_parent_ids,
    array_agg(tag.parent_user_id) AS tag_parent_user_ids,
    array_agg(tag.user_id) AS tag_user_ids,
    array_agg(tag.user_name) AS tag_user_names,
    array_agg(tag.user_superuser) AS tag_user_superusers,
    asset.id AS asset_id,
    asset.name AS asset_name,
    asset.description AS asset_description,
    asset.value_per_unit AS asset_value_per_unit,
    asset.amount AS asset_amount,
    asset.currency_id AS asset_currency_id,
    asset.currency_minor_in_mayor AS asset_currency_minor_in_mayor,
    asset.currency_name AS asset_currency_name,
    asset.currency_symbol AS asset_currency_symbol,
    asset.user_id AS asset_user_id,
    asset.user_name AS asset_user_name,
    asset.user_superuser AS asset_user_superuser,
    asset.tag_ids AS asset_tag_ids,
    asset.tag_names AS asset_tag_names,
    asset.tag_parent_ids AS asset_tag_parent_ids,
    asset.tag_parent_names AS asset_tag_parent_names,
    asset.tag_parent_parent_ids AS asset_tag_parent_parent_ids,
    asset.tag_parent_user_ids AS asset_tag_parent_user_ids,
    asset.tag_user_ids AS asset_tag_user_ids,
    asset.tag_user_names AS asset_tag_user_names,
    asset.tag_user_superusers AS asset_tag_superusers
   FROM transactions t
     LEFT JOIN currencies c ON t.currency_id = c.id
     LEFT JOIN users u ON t.user_id = u.id
     LEFT JOIN deep_accounts a ON t.account_id = a.id
     LEFT JOIN deep_recipients r ON t.recipient_id = r.id
     LEFT JOIN transaction_tags tt ON t.id = tt.transaction_id
     LEFT JOIN deep_tags tag ON tt.tag_id = tag.id
     LEFT JOIN asset_transactions at ON at.transaction_id = t.id
     LEFT JOIN deep_assets asset ON at.asset_id = asset.id
  GROUP BY t.id, c.id, u.id, a.id, a.name, a.default_currency_id, a.default_currency_name, a.default_currency_minor_in_mayor, a.default_currency_symbol, a.user_id, a.user_name, a.user_superuser, r.id, r.name, r.user_id, r.user_name, r.user_superuser, a.tag_ids, a.tag_names, a.tag_parent_ids, a.tag_parent_names, a.tag_parent_parent_ids, a.tag_parent_user_ids, a.tag_user_ids, a.tag_user_names, a.tag_user_superusers, r.tag_ids, r.tag_names, r.tag_parent_ids, r.tag_parent_names, r.tag_parent_parent_ids, r.tag_parent_user_ids, r.tag_user_ids, r.tag_user_names, r.tag_user_superusers, asset.id, asset.name, asset.description, asset.value_per_unit, asset.amount, asset.currency_id, asset.currency_name, asset.currency_symbol, asset.currency_minor_in_mayor, asset.user_id, asset.user_name, asset.user_superuser, asset.tag_ids, asset.tag_names, asset.tag_parent_ids, asset.tag_parent_names, asset.tag_parent_parent_ids, asset.tag_parent_user_ids, asset.tag_user_ids, asset.tag_user_names, asset.tag_user_superusers
  ORDER BY t.id;
