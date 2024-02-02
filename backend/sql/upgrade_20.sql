SET session_replication_role = 'replica';

UPDATE public.accounts 
    SET user_id=0 
    WHERE user_id IS NULL;

ALTER TABLE IF EXISTS public.accounts
    ALTER COLUMN user_id SET NOT NULL;

SET session_replication_role = 'origin';

ALTER TABLE IF EXISTS public.charts
    ALTER COLUMN user_id SET NOT NULL;