CREATE TABLE IF NOT EXISTS public.budgets
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    name text COLLATE pg_catalog."default" NOT NULL,
    user_id integer NOT NULL,
    amount integer NOT NULL,
    rollover boolean NOT NULL,
    period integer NOT NULL,
    currency_id integer NOT NULL,
    active_from timestamp with time zone NOT NULL,
    active_to timestamp with time zone,
    CONSTRAINT budgets_pkey PRIMARY KEY (id),
    CONSTRAINT currency_id FOREIGN KEY (currency_id)
        REFERENCES public.currencies (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);


CREATE TABLE public.budget_filter_tags
(
    budget_id integer NOT NULL,
    tag_id integer NOT NULL,
    PRIMARY KEY (budget_id, tag_id),
    CONSTRAINT budget_id FOREIGN KEY (budget_id)
        REFERENCES public.budgets (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT tag_id FOREIGN KEY (tag_id)
        REFERENCES public.tags (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);


CREATE VIEW public.budget_data
 AS
SELECT b.id, b.name, b.user_id, b.amount, b.rollover, b.period, array_agg(DISTINCT bft.tag_id) AS filter_tag_ids, b.active_from, b.active_to, c.minor_in_major, c.symbol, b.currency_id
FROM public.budgets b
LEFT JOIN public.budget_filter_tags bft ON b.id = bft.budget_id
LEFT JOIN public.currencies c ON c.id = b.currency_id
GROUP BY b.id, c.id
ORDER BY b.id;