ALTER TABLE IF EXISTS public.access_tokens
    ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT (now() at time zone 'utc');