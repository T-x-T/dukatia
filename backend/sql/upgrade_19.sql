ALTER TABLE IF EXISTS public.users
    ADD COLUMN active boolean NOT NULL DEFAULT true;

ALTER TABLE IF EXISTS public.users
    ADD COLUMN last_logon timestamp with time zone;