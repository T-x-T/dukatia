ALTER TABLE IF EXISTS public.users
    ADD CONSTRAINT users_name_unique UNIQUE (name)
    INCLUDE (name);