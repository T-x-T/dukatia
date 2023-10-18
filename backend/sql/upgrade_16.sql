CREATE TABLE public.budgets
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 ),
    name text NOT NULL,
    user_id integer NOT NULL,
    amount integer NOT NULL,
    rollover boolean NOT NULL,
    period integer NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);

ALTER TABLE IF EXISTS public.budgets
    OWNER to postgres;


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

ALTER TABLE IF EXISTS public.budget_filter_tags
    OWNER to postgres;


CREATE VIEW public.budget_data
 AS
SELECT b.id, b.name, b.user_id, b.amount, b.rollover, b.period, array_agg(DISTINCT bft.tag_id) AS filter_tag_ids
FROM public.budgets b
LEFT JOIN public.budget_filter_tags bft ON b.id = bft.budget_id
GROUP BY b.id
ORDER BY b.id;

ALTER TABLE public.budget_data
    OWNER TO postgres;