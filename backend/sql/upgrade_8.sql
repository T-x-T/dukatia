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
ALTER TABLE public.deep_tags
    OWNER TO postgres;

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

ALTER TABLE public.deep_accounts
    OWNER TO postgres;

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

ALTER TABLE public.deep_recipients
    OWNER TO postgres;


