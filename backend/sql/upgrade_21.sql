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