CREATE UNIQUE INDEX access_token_index_token
    ON public.access_tokens USING btree
    (token ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX accounts_index_user_id
    ON public.accounts USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX assets_index_user_id
    ON public.assets USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX chart_index_user_id
    ON public.charts USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX dashboards_index_user_id
    ON public.dashboards USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX recipients_index_user_id
    ON public.recipients USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX tags_index_user_id
    ON public.tags USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX transactions_index_account_id
    ON public.transactions USING btree
    (account_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX transactions_index_recipient_id
    ON public.transactions USING btree
    (recipient_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX transactions_index_timestamp
    ON public.transactions USING btree
    ("timestamp" ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE INDEX transactions_index_user_id
    ON public.transactions USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;

CREATE TABLE IF NOT EXISTS public.transaction_positions
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    transaction_id integer NOT NULL,
    amount integer NOT NULL,
    comment text COLLATE pg_catalog."default",
    tag_id integer,
    CONSTRAINT transaction_positions_pkey PRIMARY KEY (id),
    CONSTRAINT tag_id FOREIGN KEY (tag_id)
        REFERENCES public.tags (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE SET NULL
        NOT VALID,
    CONSTRAINT transaction_id FOREIGN KEY (transaction_id)
        REFERENCES public.transactions (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.transaction_positions
    OWNER to postgres;

CREATE INDEX IF NOT EXISTS transaction_positions_index_transaction_id
    ON public.transaction_positions USING btree
    (transaction_id ASC NULLS LAST)
    TABLESPACE pg_default;


INSERT INTO public.transaction_positions (transaction_id, amount)
SELECT id, amount FROM public.transactions ORDER BY id ASC;

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
          WHERE p.transaction_id = tr.id) AS transaction_position_tag_ids
   FROM transactions tr
     LEFT JOIN transaction_tags t ON tr.id = t.transaction_id
     LEFT JOIN asset_transactions at ON at.transaction_id = tr.id
     LEFT JOIN assets a ON a.id = at.asset_id
  GROUP BY tr.id, a.id
  ORDER BY tr.id;

ALTER TABLE public.transaction_data
    OWNER TO postgres;

GRANT ALL ON TABLE public.transaction_data TO postgres;

ALTER TABLE IF EXISTS public.transactions DROP COLUMN IF EXISTS amount;

GRANT ALL ON TABLE public.transaction_data TO postgres;

CREATE OR REPLACE VIEW public.deep_transactions
 AS
 SELECT t.id,
    t.status,
    t."timestamp",
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
    asset.tag_user_superusers AS asset_tag_superusers,
    ( SELECT array_agg(p.id) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = t.id) AS transaction_position_ids,
    ( SELECT array_agg(p.amount) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = t.id) AS transaction_position_amounts,
    ( SELECT array_agg(p.comment) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = t.id) AS transaction_position_comments,
    ( SELECT array_agg(p.tag_id) AS array_agg
           FROM transaction_positions p
          WHERE p.transaction_id = t.id) AS transaction_position_tag_ids
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

ALTER TABLE public.deep_transactions
    OWNER TO postgres;
