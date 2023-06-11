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