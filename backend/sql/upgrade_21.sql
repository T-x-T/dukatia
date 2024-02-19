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

ALTER TABLE public.recipient_data
	OWNER TO postgres;

GRANT ALL ON TABLE public.recipient_data TO postgres;

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

ALTER TABLE public.transaction_data
    OWNER TO postgres;

GRANT ALL ON TABLE public.transaction_data TO postgres;

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

ALTER TABLE public.account_data
    OWNER TO postgres;

GRANT ALL ON TABLE public.account_data TO postgres;
--
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

ALTER TABLE public.budget_data
    OWNER TO postgres;
	
GRANT ALL ON TABLE public.budget_data TO postgres;

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

ALTER TABLE public.budget_data
    OWNER TO postgres;
	
GRANT ALL ON TABLE public.budget_data TO postgres;




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

ALTER TABLE public.asset_data
  OWNER TO postgres;

GRANT ALL ON TABLE public.asset_data TO postgres;

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

ALTER TABLE public.transaction_data
    OWNER TO postgres;

GRANT ALL ON TABLE public.transaction_data TO postgres;

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

ALTER TABLE public.account_data
    OWNER TO postgres;

GRANT ALL ON TABLE public.account_data TO postgres;

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

ALTER TABLE public.asset_valuation_history
    OWNER TO postgres;

GRANT ALL ON TABLE public.asset_valuation_history TO postgres;

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

ALTER TABLE public.asset_data
  OWNER TO postgres;

GRANT ALL ON TABLE public.asset_data TO postgres;

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

ALTER TABLE public.asset_valuation_history
    OWNER TO postgres;

GRANT ALL ON TABLE public.asset_valuation_history TO postgres;




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
ALTER TABLE public.transaction_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.transaction_data TO postgres;

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
ALTER TABLE public.account_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.account_data TO postgres;




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
ALTER TABLE public.transaction_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.transaction_data TO postgres;

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
ALTER TABLE public.account_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.account_data TO postgres;

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
ALTER TABLE public.transaction_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.transaction_data TO postgres;

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
ALTER TABLE public.account_data
    OWNER TO postgres;
GRANT ALL ON TABLE public.account_data TO postgres;