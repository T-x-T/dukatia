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


CREATE TABLE IF NOT EXISTS public.dashboards
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    user_id integer NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    description text COLLATE pg_catalog."default",
    CONSTRAINT dashboards_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.dashboards
    OWNER to postgres;
ALTER TABLE IF EXISTS public.dashboards
    ADD CONSTRAINT dashboard_id_unique UNIQUE (id);

CREATE TABLE IF NOT EXISTS public.charts
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    user_id integer,
    grid_size text COLLATE pg_catalog."default" NOT NULL,
    chart_type text COLLATE pg_catalog."default" NOT NULL,
    title text COLLATE pg_catalog."default" NOT NULL,
    text_template text COLLATE pg_catalog."default",
    filter_from timestamp with time zone,
    filter_to timestamp with time zone,
    filter_collection text COLLATE pg_catalog."default",
    date_period text COLLATE pg_catalog."default",
    CONSTRAINT charts_pkey PRIMARY KEY (id),
    CONSTRAINT chart_id_unique UNIQUE (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.charts
    OWNER to postgres;
ALTER TABLE IF EXISTS public.charts
    ADD CONSTRAINT chart_id_unique UNIQUE (id);


CREATE TABLE IF NOT EXISTS public.dashboard_charts
(
    dashboard_id integer NOT NULL,
    chart_id integer NOT NULL,
    CONSTRAINT dashboard_charts_pkey PRIMARY KEY (dashboard_id, chart_id),
    CONSTRAINT chart_id FOREIGN KEY (chart_id)
        REFERENCES public.charts (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT dashboard_id FOREIGN KEY (dashboard_id)
        REFERENCES public.dashboards (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.dashboard_charts
    OWNER to postgres;


ALTER TABLE public.charts DISABLE TRIGGER ALL;
INSERT INTO public.charts VALUES (DEFAULT, 0, 'small', 'text', 'Current balance', '%foreach_currency{$name: $current_balance$symbol*}%', NULL, NULL, NULL, 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'medium', 'pie', 'Spending per Recipient in Date Range', NULL, NULL, NULL, 'recipients', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'medium', 'pie', 'Spending per Tag in Date Range', NULL, NULL, NULL, 'tags', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Balance per Recipient over Time', NULL, NULL, NULL, 'recipients', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Balance per Account over Time', NULL, NULL, NULL, 'accounts', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Balance per currency over Time', NULL, NULL, NULL, 'currencies', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Earning and spending over Time', NULL, NULL, NULL, 'earning_spending_net', 'monthly');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Total value over time', NULL, NULL, NULL, 'asset_total_value', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Value over time per single unit', NULL, NULL, NULL, 'asset_single_value', 'daily');
INSERT INTO public.charts VALUES (DEFAULT, 0, 'large', 'line', 'Amount over time', NULL, NULL, NULL, 'asset_amount', 'daily');
ALTER TABLE public.charts ENABLE TRIGGER ALL;

ALTER TABLE public.dashboard_charts DISABLE TRIGGER ALL;
INSERT INTO public.dashboard_charts VALUES (0, 0);
INSERT INTO public.dashboard_charts VALUES (0, 1);
INSERT INTO public.dashboard_charts VALUES (0, 2);
INSERT INTO public.dashboard_charts VALUES (0, 3);
INSERT INTO public.dashboard_charts VALUES (0, 4);
INSERT INTO public.dashboard_charts VALUES (0, 5);
INSERT INTO public.dashboard_charts VALUES (0, 6);
ALTER TABLE public.dashboard_charts ENABLE TRIGGER ALL;