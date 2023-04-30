CREATE VIEW public.account_data
 AS
SELECT a.id, a.name, a.default_currency_id, a.user_id, array_agg(t.tag_id) as tags FROM public.accounts a LEFT JOIN public.account_tags t ON a.id = t.account_id GROUP BY a.id;

ALTER TABLE public.account_data
    OWNER TO postgres;


CREATE VIEW public.recipient_data
 AS
SELECT r.id, r.name, r.user_id, array_agg(t.tag_id) as tags FROM public.recipients r LEFT JOIN public.recipient_tags t ON r.id = t.recipient_id GROUP BY r.id;

ALTER TABLE public.recipient_data
    OWNER TO postgres;