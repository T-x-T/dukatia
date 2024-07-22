-- Recipients

ALTER TABLE IF EXISTS public.recipients
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE IF EXISTS public.recipients
	ADD CONSTRAINT unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.recipient_tags
	ADD COLUMN new_recipient_id uuid;

UPDATE public.recipient_tags
	SET new_recipient_id = (
		SELECT new_id FROM public.recipients WHERE id = recipient_id
	);

DROP VIEW public.recipient_data;

ALTER TABLE IF EXISTS public.recipient_tags 
	DROP COLUMN IF EXISTS recipient_id;
ALTER TABLE IF EXISTS public.recipient_tags
	RENAME new_recipient_id TO recipient_id;
ALTER TABLE IF EXISTS public.recipient_tags
	ALTER COLUMN recipient_id SET NOT NULL;
ALTER TABLE IF EXISTS 
	public.recipient_tags DROP CONSTRAINT IF EXISTS recipient_id;
ALTER TABLE IF EXISTS public.recipient_tags
	ADD CONSTRAINT recipient_id FOREIGN KEY (recipient_id)
	REFERENCES public.recipients (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;

CREATE OR REPLACE VIEW public.recipient_data
 AS
 SELECT r.id,
    r.name,
    r.user_id,
    array_agg(t.tag_id) AS tags
   FROM recipients r
     LEFT JOIN recipient_tags t ON r.new_id = t.recipient_id
  GROUP BY r.id
  ORDER BY r.id;

ALTER TABLE IF EXISTS public.recipient_tags
	ADD PRIMARY KEY (recipient_id, tag_id);

DROP VIEW public.account_data;
DROP VIEW public.transaction_data;

ALTER TABLE IF EXISTS public.transactions
	ADD COLUMN new_recipient_id uuid;
UPDATE public.transactions
	SET new_recipient_id = (SELECT new_id FROM public.recipients WHERE id = recipient_id);
ALTER TABLE IF EXISTS public.transactions 
	DROP COLUMN IF EXISTS recipient_id;
ALTER TABLE IF EXISTS public.transactions
	RENAME new_recipient_id TO recipient_id;
ALTER TABLE IF EXISTS public.transactions
	ALTER COLUMN recipient_id SET NOT NULL;
ALTER TABLE IF EXISTS 
	public.transactions DROP CONSTRAINT IF EXISTS recipient_id;
ALTER TABLE IF EXISTS public.transactions
	ADD CONSTRAINT recipient_id FOREIGN KEY (recipient_id)
	REFERENCES public.recipients (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

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


DROP VIEW public.recipient_data;

ALTER TABLE IF EXISTS public.recipients DROP COLUMN IF EXISTS id;

ALTER TABLE IF EXISTS public.recipients
	RENAME new_id TO id;

ALTER TABLE IF EXISTS public.recipients
	ADD PRIMARY KEY (id);

CREATE OR REPLACE VIEW public.recipient_data
 AS
 SELECT r.id,
    r.name,
    r.user_id,
    array_agg(t.tag_id) AS tags
  FROM recipients r
    LEFT JOIN recipient_tags t ON r.id = t.recipient_id
  GROUP BY r.id
  ORDER BY r.name;

ALTER TABLE public.recipients
  RENAME CONSTRAINT unique_new_id TO recipients_unique_new_id;




-- Budgets

ALTER TABLE IF EXISTS public.budgets
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE IF EXISTS public.budgets
	ADD CONSTRAINT budgets_unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD COLUMN new_budget_id uuid;

UPDATE public.budget_filter_tags
	SET new_budget_id = (
		SELECT new_id FROM public.budgets WHERE id = budget_id
	);

DROP VIEW public.budget_data;

ALTER TABLE IF EXISTS public.budget_filter_tags 
	DROP COLUMN IF EXISTS budget_id;
ALTER TABLE IF EXISTS public.budget_filter_tags
	RENAME new_budget_id TO budget_id;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ALTER COLUMN budget_id SET NOT NULL;
ALTER TABLE IF EXISTS 
	public.budget_filter_tags DROP CONSTRAINT IF EXISTS budget_id;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD CONSTRAINT budget_id FOREIGN KEY (budget_id)
	REFERENCES public.budgets (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD PRIMARY KEY (budget_id, tag_id);

CREATE OR REPLACE VIEW public.budget_data
 AS
 SELECT b.id,
    b.name,
    b.user_id,
    b.amount,
    b.rollover,
    b.period,
    array_agg(DISTINCT bft.tag_id) AS filter_tag_ids,
    b.active_from,
    b.active_to,
    c.minor_in_major,
    c.symbol,
    b.currency_id
   FROM budgets b
     LEFT JOIN budget_filter_tags bft ON b.new_id = bft.budget_id
     LEFT JOIN currencies c ON c.id = b.currency_id
  GROUP BY b.id, c.id
  ORDER BY b.id;

DROP VIEW public.budget_data;

ALTER TABLE IF EXISTS public.budgets DROP COLUMN IF EXISTS id;

ALTER TABLE IF EXISTS public.budgets
	RENAME new_id TO id;

ALTER TABLE IF EXISTS public.budgets
	ADD PRIMARY KEY (id);
	
CREATE OR REPLACE VIEW public.budget_data
 AS
 SELECT b.id,
    b.name,
    b.user_id,
    b.amount,
    b.rollover,
    b.period,
    array_agg(DISTINCT bft.tag_id) AS filter_tag_ids,
    b.active_from,
    b.active_to,
    c.minor_in_major,
    c.symbol,
    b.currency_id
   FROM budgets b
     LEFT JOIN budget_filter_tags bft ON b.id = bft.budget_id
     LEFT JOIN currencies c ON c.id = b.currency_id
  GROUP BY b.id, c.id
  ORDER BY b.name;




-- Assets
ALTER TABLE IF EXISTS public.assets
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE IF EXISTS public.assets
	ADD CONSTRAINT assets_unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.asset_transactions
	ADD COLUMN new_asset_id uuid;

UPDATE public.asset_transactions
	SET new_asset_id = (
		SELECT new_id FROM public.assets WHERE id = asset_id
	);

ALTER TABLE IF EXISTS public.asset_amounts
	ADD COLUMN new_asset_id uuid;

UPDATE public.asset_amounts
	SET new_asset_id = (
		SELECT new_id FROM public.assets WHERE id = asset_id
	);

ALTER TABLE IF EXISTS public.asset_valuations
	ADD COLUMN new_asset_id uuid;

UPDATE public.asset_valuations
	SET new_asset_id = (
		SELECT new_id FROM public.assets WHERE id = asset_id
	);

ALTER TABLE IF EXISTS public.asset_tags
	ADD COLUMN new_asset_id uuid;

UPDATE public.asset_tags
	SET new_asset_id = (
		SELECT new_id FROM public.assets WHERE id = asset_id
	);

DROP VIEW IF EXISTS public.asset_data;
DROP VIEW IF EXISTS public.asset_valuation_history;
DROP VIEW IF EXISTS public.account_data;
DROP VIEW IF EXISTS public.transaction_data;

ALTER TABLE IF EXISTS public.asset_transactions 
	DROP COLUMN IF EXISTS asset_id;
ALTER TABLE IF EXISTS public.asset_transactions
	RENAME new_asset_id TO asset_id;
ALTER TABLE IF EXISTS public.asset_transactions
	ALTER COLUMN asset_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_transactions
	ADD CONSTRAINT asset_id FOREIGN KEY (asset_id)
	REFERENCES public.assets (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_transactions
	ADD PRIMARY KEY (asset_id, transaction_id);

ALTER TABLE IF EXISTS public.asset_amounts 
	DROP COLUMN IF EXISTS asset_id;
ALTER TABLE IF EXISTS public.asset_amounts
	RENAME new_asset_id TO asset_id;
ALTER TABLE IF EXISTS public.asset_amounts
	ALTER COLUMN asset_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_amounts
	ADD CONSTRAINT asset_id FOREIGN KEY (asset_id)
	REFERENCES public.assets (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_amounts
	ADD PRIMARY KEY (asset_id, timestamp);

ALTER TABLE IF EXISTS public.asset_valuations 
	DROP COLUMN IF EXISTS asset_id;
ALTER TABLE IF EXISTS public.asset_valuations
	RENAME new_asset_id TO asset_id;
ALTER TABLE IF EXISTS public.asset_valuations
	ALTER COLUMN asset_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_valuations
	ADD CONSTRAINT asset_id FOREIGN KEY (asset_id)
	REFERENCES public.assets (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_valuations
	ADD PRIMARY KEY (asset_id, timestamp);

ALTER TABLE IF EXISTS public.asset_tags 
	DROP COLUMN IF EXISTS asset_id;
ALTER TABLE IF EXISTS public.asset_tags
	RENAME new_asset_id TO asset_id;
ALTER TABLE IF EXISTS public.asset_tags
	ALTER COLUMN asset_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_tags
	ADD CONSTRAINT asset_id FOREIGN KEY (asset_id)
	REFERENCES public.assets (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_tags
	ADD PRIMARY KEY (asset_id, tag_id);

CREATE OR REPLACE VIEW public.asset_data
    AS
     SELECT a.new_id,
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
     LEFT JOIN asset_amounts aa ON a.new_id = aa.asset_id AND aa."timestamp" = (( SELECT max(asset_amounts."timestamp") AS max
           FROM asset_amounts
          WHERE asset_amounts.asset_id = a.new_id
          GROUP BY asset_amounts.asset_id))
     LEFT JOIN asset_valuations av ON a.new_id = av.asset_id AND av."timestamp" = (( SELECT max(asset_valuations."timestamp") AS max
           FROM asset_valuations
          WHERE asset_valuations.asset_id = a.new_id
          GROUP BY asset_valuations.asset_id))
     LEFT JOIN asset_tags t ON a.new_id = t.asset_id
	 LEFT JOIN currencies c ON a.currency_id = c.id
  GROUP BY a.new_id, a.name, a.description, a.user_id, a.currency_id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.new_id;

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
    a.new_id AS asset_id,
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
     LEFT JOIN assets a ON a.new_id = at.asset_id
     LEFT JOIN currencies c ON c.id = tr.currency_id
  GROUP BY tr.id, a.new_id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr.id;

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
     LEFT JOIN assets a ON aa.asset_id = a.new_id
     LEFT JOIN currencies c ON a.currency_id = c.id
  ORDER BY (COALESCE(aa.asset_id, av.asset_id)), (COALESCE(aa."timestamp", av."timestamp"));

DROP VIEW IF EXISTS public.asset_data;
DROP VIEW IF EXISTS public.asset_valuation_history;

ALTER TABLE IF EXISTS public.assets DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.assets
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.assets
	ADD PRIMARY KEY (id);

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
  GROUP BY a.id, a.name, a.description, a.user_id, a.currency_id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.name;

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




-- Accounts
ALTER TABLE IF EXISTS public.accounts
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.accounts
	ADD CONSTRAINT accounts_unique_new_id UNIQUE (new_id);


ALTER TABLE IF EXISTS public.account_tags
	ADD COLUMN new_account_id uuid;
UPDATE public.account_tags
	SET new_account_id = (
		SELECT new_id FROM public.accounts WHERE id = account_id
	);

ALTER TABLE IF EXISTS public.transactions
	ADD COLUMN new_account_id uuid;
UPDATE public.transactions
	SET new_account_id = (
		SELECT new_id FROM public.accounts WHERE id = account_id
	);


DROP VIEW IF EXISTS public.account_data;
DROP VIEW IF EXISTS public.transaction_data;

ALTER TABLE IF EXISTS public.account_tags 
	DROP COLUMN IF EXISTS account_id;
ALTER TABLE IF EXISTS public.account_tags
	RENAME new_account_id TO account_id;
ALTER TABLE IF EXISTS public.account_tags
	ALTER COLUMN account_id SET NOT NULL;
ALTER TABLE IF EXISTS public.account_tags
	ADD CONSTRAINT account_id FOREIGN KEY (account_id)
	REFERENCES public.accounts (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.account_tags
	ADD PRIMARY KEY (account_id, tag_id);

ALTER TABLE IF EXISTS public.transactions 
	DROP COLUMN IF EXISTS account_id;
ALTER TABLE IF EXISTS public.transactions
	RENAME new_account_id TO account_id;
ALTER TABLE IF EXISTS public.transactions
	ALTER COLUMN account_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transactions
	ADD CONSTRAINT account_id FOREIGN KEY (account_id)
	REFERENCES public.accounts (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;

ALTER TABLE IF EXISTS public.accounts DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.accounts
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.accounts
	ADD PRIMARY KEY (id);

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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr.id;

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
  ORDER BY a.name;




-- Transaction
ALTER TABLE IF EXISTS public.transactions
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.transactions
	ADD CONSTRAINT transactions_unique_new_id UNIQUE (new_id);


ALTER TABLE IF EXISTS public.transaction_tags
	ADD COLUMN new_transaction_id uuid;
UPDATE public.transaction_tags
	SET new_transaction_id = (
		SELECT new_id FROM public.transactions WHERE id = transaction_id
	);

ALTER TABLE IF EXISTS public.transaction_positions
	ADD COLUMN new_transaction_id uuid;
UPDATE public.transaction_positions
	SET new_transaction_id = (
		SELECT new_id FROM public.transactions WHERE id = transaction_id
	);

ALTER TABLE IF EXISTS public.asset_transactions
	ADD COLUMN new_transaction_id uuid;
UPDATE public.asset_transactions
	SET new_transaction_id = (
		SELECT new_id FROM public.transactions WHERE id = transaction_id
	);


DROP VIEW IF EXISTS public.account_data;
DROP VIEW IF EXISTS public.transaction_data;

ALTER TABLE IF EXISTS public.transaction_tags 
	DROP COLUMN IF EXISTS transaction_id;
ALTER TABLE IF EXISTS public.transaction_tags
	RENAME new_transaction_id TO transaction_id;
ALTER TABLE IF EXISTS public.transaction_tags
	ALTER COLUMN transaction_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transaction_tags
	ADD CONSTRAINT transaction_id FOREIGN KEY (transaction_id)
	REFERENCES public.transactions (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.transaction_tags
	ADD PRIMARY KEY (transaction_id, tag_id);

ALTER TABLE IF EXISTS public.transaction_positions 
	DROP COLUMN IF EXISTS transaction_id;
ALTER TABLE IF EXISTS public.transaction_positions
	RENAME new_transaction_id TO transaction_id;
ALTER TABLE IF EXISTS public.transaction_positions
	ALTER COLUMN transaction_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transaction_positions
	ADD CONSTRAINT transaction_id FOREIGN KEY (transaction_id)
	REFERENCES public.transactions (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;

ALTER TABLE IF EXISTS public.asset_transactions 
	DROP COLUMN IF EXISTS transaction_id;
ALTER TABLE IF EXISTS public.asset_transactions
	RENAME new_transaction_id TO transaction_id;
ALTER TABLE IF EXISTS public.asset_transactions
	ALTER COLUMN transaction_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_transactions
	ADD CONSTRAINT transaction_id FOREIGN KEY (transaction_id)
	REFERENCES public.transactions (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_transactions
	ADD PRIMARY KEY (transaction_id, asset_id);


ALTER TABLE IF EXISTS public.transactions DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.transactions
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.transactions
	ADD PRIMARY KEY (id);

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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr."timestamp" DESC, tr.comment;

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
  ORDER BY a.name;

CREATE INDEX transaction_positions_index_transaction_id
  ON public.transaction_positions USING btree
  (transaction_id ASC NULLS LAST)
  WITH (deduplicate_items=True)
  TABLESPACE pg_default;




-- Positions
ALTER TABLE IF EXISTS public.transaction_positions
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.transaction_positions
	ADD CONSTRAINT transaction_positions_unique_new_id UNIQUE (new_id);


DROP VIEW IF EXISTS public.account_data;
DROP VIEW IF EXISTS public.transaction_data;

ALTER TABLE IF EXISTS public.transaction_positions DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.transaction_positions
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.transaction_positions
	ADD PRIMARY KEY (id);

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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr."timestamp" DESC, tr.comment;

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
  ORDER BY a.name;



-- access_tokens
ALTER TABLE IF EXISTS public.access_tokens
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.access_tokens
	ADD CONSTRAINT access_tokens_unique_new_id UNIQUE (new_id);
ALTER TABLE IF EXISTS public.access_tokens DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.access_tokens
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.access_tokens
	ADD PRIMARY KEY (id);



-- charts
DELETE FROM public.charts WHERE id <= 6;

ALTER TABLE IF EXISTS public.charts
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.charts
	ADD CONSTRAINT charts_unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.dashboard_charts
	ADD COLUMN new_chart_id uuid;
UPDATE public.dashboard_charts
	SET new_chart_id = (
		SELECT new_id FROM public.charts WHERE id = chart_id
	);
ALTER TABLE IF EXISTS public.dashboard_charts 
	DROP COLUMN IF EXISTS chart_id;
ALTER TABLE IF EXISTS public.dashboard_charts
	RENAME new_chart_id TO chart_id;
ALTER TABLE IF EXISTS public.dashboard_charts
	ALTER COLUMN chart_id SET NOT NULL;
ALTER TABLE IF EXISTS public.dashboard_charts
	ADD CONSTRAINT chart_id FOREIGN KEY (chart_id)
	REFERENCES public.charts (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.dashboard_charts
	ADD PRIMARY KEY (chart_id, dashboard_id);

ALTER TABLE IF EXISTS public.charts DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.charts
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.charts
	ADD PRIMARY KEY (id);



-- dashboards
ALTER TABLE IF EXISTS public.dashboards
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.dashboards
	ADD CONSTRAINT dashboards_unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.dashboard_charts
	ADD COLUMN new_dashboard_id uuid;
UPDATE public.dashboard_charts
	SET new_dashboard_id = (
		SELECT new_id FROM public.dashboards WHERE id = dashboard_id
	);
ALTER TABLE IF EXISTS public.dashboard_charts 
	DROP COLUMN IF EXISTS dashboard_id;
ALTER TABLE IF EXISTS public.dashboard_charts
	RENAME new_dashboard_id TO dashboard_id;
ALTER TABLE IF EXISTS public.dashboard_charts
	ALTER COLUMN dashboard_id SET NOT NULL;
ALTER TABLE IF EXISTS public.dashboard_charts
	ADD CONSTRAINT dashboard_id FOREIGN KEY (dashboard_id)
	REFERENCES public.dashboards (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.dashboard_charts
	ADD PRIMARY KEY (chart_id, dashboard_id);

ALTER TABLE IF EXISTS public.dashboards DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.dashboards
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.dashboards
	ADD PRIMARY KEY (id);



-- tags
ALTER TABLE IF EXISTS public.tags
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.tags
	ADD CONSTRAINT tags_unique_new_id UNIQUE (new_id);

ALTER TABLE IF EXISTS public.tags
	ADD COLUMN new_parent_id uuid;
UPDATE public.tags
	SET new_parent_id = (
		SELECT new_id FROM public.tags t2 WHERE t2.id = public.tags.parent_id AND t2.new_id IS NOT NULL
	)
  WHERE parent_id IS NOT NULL;
ALTER TABLE IF EXISTS public.tags
	DROP COLUMN parent_id;
ALTER TABLE IF EXISTS public.tags
  RENAME new_parent_id to parent_id;


DROP VIEW account_data;
DROP VIEW asset_data;
DROP VIEW budget_data;
DROP VIEW transaction_data;
DROP VIEW recipient_data;

ALTER TABLE IF EXISTS public.account_tags
	ADD COLUMN new_tag_id uuid;
UPDATE public.account_tags
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.account_tags 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.account_tags
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.account_tags
	ALTER COLUMN tag_id SET NOT NULL;
ALTER TABLE IF EXISTS public.account_tags
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.account_tags
	ADD PRIMARY KEY (tag_id, account_id);

ALTER TABLE IF EXISTS public.asset_tags
	ADD COLUMN new_tag_id uuid;
UPDATE public.asset_tags
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.asset_tags 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.asset_tags
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.asset_tags
	ALTER COLUMN tag_id SET NOT NULL;
ALTER TABLE IF EXISTS public.asset_tags
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.asset_tags
	ADD PRIMARY KEY (tag_id, asset_id);

ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD COLUMN new_tag_id uuid;
UPDATE public.budget_filter_tags
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.budget_filter_tags 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.budget_filter_tags
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ALTER COLUMN tag_id SET NOT NULL;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.budget_filter_tags
	ADD PRIMARY KEY (tag_id, budget_id);

ALTER TABLE IF EXISTS public.transaction_tags
	ADD COLUMN new_tag_id uuid;
UPDATE public.transaction_tags
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.transaction_tags 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.transaction_tags
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.transaction_tags
	ALTER COLUMN tag_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transaction_tags
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.transaction_tags
	ADD PRIMARY KEY (tag_id, transaction_id);

ALTER TABLE IF EXISTS public.transaction_positions
	ADD COLUMN new_tag_id uuid;
UPDATE public.transaction_positions
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.transaction_positions 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.transaction_positions
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.transaction_positions
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;

ALTER TABLE IF EXISTS public.recipient_tags
	ADD COLUMN new_tag_id uuid;
UPDATE public.recipient_tags
	SET new_tag_id = (
		SELECT new_id FROM public.tags WHERE id = tag_id
	);
ALTER TABLE IF EXISTS public.recipient_tags 
	DROP COLUMN IF EXISTS tag_id;
ALTER TABLE IF EXISTS public.recipient_tags
	RENAME new_tag_id TO tag_id;
ALTER TABLE IF EXISTS public.recipient_tags
	ALTER COLUMN tag_id SET NOT NULL;
ALTER TABLE IF EXISTS public.recipient_tags
	ADD CONSTRAINT tag_id FOREIGN KEY (tag_id)
	REFERENCES public.tags (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE CASCADE
	NOT VALID;
ALTER TABLE IF EXISTS public.recipient_tags
	ADD PRIMARY KEY (tag_id, recipient_id);


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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr."timestamp" DESC, tr.comment;

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
  ORDER BY a.name;

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
  GROUP BY a.id, a.name, a.description, a.user_id, a.currency_id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.name;

CREATE OR REPLACE VIEW public.budget_data
 AS
 SELECT b.id,
    b.name,
    b.user_id,
    b.amount,
    b.rollover,
    b.period,
    array_agg(DISTINCT bft.tag_id) AS filter_tag_ids,
    b.active_from,
    b.active_to,
    c.minor_in_major,
    c.symbol,
    b.currency_id
   FROM budgets b
     LEFT JOIN budget_filter_tags bft ON b.id = bft.budget_id
     LEFT JOIN currencies c ON c.id = b.currency_id
  GROUP BY b.id, c.id
  ORDER BY b.name;

CREATE OR REPLACE VIEW public.recipient_data
 AS
 SELECT r.id,
    r.name,
    r.user_id,
    array_agg(t.tag_id) AS tags
   FROM recipients r
     LEFT JOIN recipient_tags t ON r.id = t.recipient_id
  GROUP BY r.id
  ORDER BY r.name;


ALTER TABLE IF EXISTS public.tags DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.tags
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.tags
	ADD PRIMARY KEY (id);



-- users

-- first insert admin user if no users exist
INSERT INTO public.users (name, secret, permissions, superuser, active, last_logon)
  SELECT 'default', '', NULL, true, true, NULL
  WHERE NOT EXISTS (SELECT * FROM public.users);

ALTER TABLE IF EXISTS public.users
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.users
	ADD CONSTRAINT users_unique_new_id UNIQUE (new_id);

DROP VIEW public.account_data;
DROP VIEW public.asset_data;
DROP VIEW public.budget_data;
DROP VIEW public.recipient_data;
DROP VIEW public.transaction_data;

ALTER TABLE IF EXISTS public.access_tokens
	ADD COLUMN new_user_id uuid;
UPDATE public.access_tokens
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.access_tokens 
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.access_tokens
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.access_tokens
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.access_tokens
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.accounts
	ADD COLUMN new_user_id uuid;
ALTER TABLE IF EXISTS public.accounts
  DROP CONSTRAINT IF EXISTS user_id;
UPDATE public.accounts
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.accounts
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.accounts
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.accounts
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.accounts
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.assets
	ADD COLUMN new_user_id uuid;
UPDATE public.assets
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.assets
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.assets
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.assets
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.assets
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.budgets
	ADD COLUMN new_user_id uuid;
UPDATE public.budgets
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.budgets
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.budgets
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.budgets
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.budgets
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.charts
	ADD COLUMN new_user_id uuid;
UPDATE public.charts
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.charts
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.charts
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.charts
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.charts
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.dashboards
	ADD COLUMN new_user_id uuid;
UPDATE public.dashboards
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.dashboards
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.dashboards
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.dashboards
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.dashboards
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.recipients
	ADD COLUMN new_user_id uuid;
UPDATE public.recipients
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.recipients
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.recipients
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.recipients
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.tags
	ADD COLUMN new_user_id uuid;
UPDATE public.tags
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.tags
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.tags
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.tags
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.tags
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.transactions
	ADD COLUMN new_user_id uuid;
UPDATE public.transactions
	SET new_user_id = (
		SELECT new_id FROM public.users WHERE id = user_id
	);
ALTER TABLE IF EXISTS public.transactions
	DROP COLUMN IF EXISTS user_id;
ALTER TABLE IF EXISTS public.transactions
	RENAME new_user_id TO user_id;
ALTER TABLE IF EXISTS public.transactions
	ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transactions
	ADD CONSTRAINT user_id FOREIGN KEY (user_id)
	REFERENCES public.users (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;


ALTER TABLE IF EXISTS public.users DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.users
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.users
	ADD PRIMARY KEY (id);


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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr."timestamp" DESC, tr.comment;

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
  ORDER BY a.name;

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
  GROUP BY a.id, a.name, a.description, a.user_id, a.currency_id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.name;

CREATE OR REPLACE VIEW public.budget_data
 AS
 SELECT b.id,
    b.name,
    b.user_id,
    b.amount,
    b.rollover,
    b.period,
    array_agg(DISTINCT bft.tag_id) AS filter_tag_ids,
    b.active_from,
    b.active_to,
    c.minor_in_major,
    c.symbol,
    b.currency_id
   FROM budgets b
     LEFT JOIN budget_filter_tags bft ON b.id = bft.budget_id
     LEFT JOIN currencies c ON c.id = b.currency_id
  GROUP BY b.id, c.id
  ORDER BY b.name;

CREATE OR REPLACE VIEW public.recipient_data
 AS
 SELECT r.id,
    r.name,
    r.user_id,
    array_agg(t.tag_id) AS tags
   FROM recipients r
     LEFT JOIN recipient_tags t ON r.id = t.recipient_id
  GROUP BY r.id
  ORDER BY r.name;



-- currency
ALTER TABLE IF EXISTS public.currencies
	ADD COLUMN new_id uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE IF EXISTS public.currencies
	ADD CONSTRAINT currencies_unique_new_id UNIQUE (new_id);

DROP VIEW public.account_data;
DROP VIEW public.transaction_data;
DROP VIEW public.budget_data;
DROP VIEW public.asset_valuation_history;
DROP VIEW public.asset_data;

ALTER TABLE IF EXISTS public.accounts
	ADD COLUMN new_default_currency_id uuid;
UPDATE public.accounts
	SET new_default_currency_id = (
		SELECT new_id FROM public.currencies WHERE id = default_currency_id
	);
ALTER TABLE IF EXISTS public.accounts
	DROP COLUMN IF EXISTS default_currency_id;
ALTER TABLE IF EXISTS public.accounts
	RENAME new_default_currency_id TO default_currency_id;
ALTER TABLE IF EXISTS public.accounts
	ALTER COLUMN default_currency_id SET NOT NULL;
ALTER TABLE IF EXISTS public.accounts
	ADD CONSTRAINT default_currency_id FOREIGN KEY (default_currency_id)
	REFERENCES public.currencies (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.transactions
	ADD COLUMN new_currency_id uuid;
UPDATE public.transactions
	SET new_currency_id = (
		SELECT new_id FROM public.currencies WHERE id = currency_id
	);
ALTER TABLE IF EXISTS public.transactions
	DROP COLUMN IF EXISTS currency_id;
ALTER TABLE IF EXISTS public.transactions
	RENAME new_currency_id TO currency_id;
ALTER TABLE IF EXISTS public.transactions
	ALTER COLUMN currency_id SET NOT NULL;
ALTER TABLE IF EXISTS public.transactions
	ADD CONSTRAINT currency_id FOREIGN KEY (currency_id)
	REFERENCES public.currencies (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.budgets
	ADD COLUMN new_currency_id uuid;
UPDATE public.budgets
	SET new_currency_id = (
		SELECT new_id FROM public.currencies WHERE id = currency_id
	);
ALTER TABLE IF EXISTS public.budgets
	DROP COLUMN IF EXISTS currency_id;
ALTER TABLE IF EXISTS public.budgets
	RENAME new_currency_id TO currency_id;
ALTER TABLE IF EXISTS public.budgets
	ALTER COLUMN currency_id SET NOT NULL;
ALTER TABLE IF EXISTS public.budgets
	ADD CONSTRAINT currency_id FOREIGN KEY (currency_id)
	REFERENCES public.currencies (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;

ALTER TABLE IF EXISTS public.assets
	ADD COLUMN new_currency_id uuid;
UPDATE public.assets
	SET new_currency_id = (
		SELECT new_id FROM public.currencies WHERE id = currency_id
	);
ALTER TABLE IF EXISTS public.assets
	DROP COLUMN IF EXISTS currency_id;
ALTER TABLE IF EXISTS public.assets
	RENAME new_currency_id TO currency_id;
ALTER TABLE IF EXISTS public.assets
	ALTER COLUMN currency_id SET NOT NULL;
ALTER TABLE IF EXISTS public.assets
	ADD CONSTRAINT currency_id FOREIGN KEY (currency_id)
	REFERENCES public.currencies (new_id) MATCH SIMPLE
	ON UPDATE CASCADE
	ON DELETE RESTRICT
	NOT VALID;



ALTER TABLE IF EXISTS public.currencies DROP COLUMN IF EXISTS id;
ALTER TABLE IF EXISTS public.currencies
	RENAME new_id TO id;
ALTER TABLE IF EXISTS public.currencies
	ADD PRIMARY KEY (id);



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
  GROUP BY tr.id, a.id, a.name, a.description, c.minor_in_major, c.symbol
  ORDER BY tr."timestamp" DESC, tr.comment;

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
  ORDER BY a.name;

CREATE OR REPLACE VIEW public.budget_data
 AS
 SELECT b.id,
    b.name,
    b.user_id,
    b.amount,
    b.rollover,
    b.period,
    array_agg(DISTINCT bft.tag_id) AS filter_tag_ids,
    b.active_from,
    b.active_to,
    c.minor_in_major,
    c.symbol,
    b.currency_id
   FROM budgets b
     LEFT JOIN budget_filter_tags bft ON b.id = bft.budget_id
     LEFT JOIN currencies c ON c.id = b.currency_id
  GROUP BY b.id, c.id
  ORDER BY b.name;

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
  GROUP BY a.id, a.name, a.description, a.user_id, a.currency_id, aa.amount, av.value_per_unit, c.minor_in_major, c.symbol
  ORDER BY a.name;