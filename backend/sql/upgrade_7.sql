ALTER TABLE IF EXISTS public.charts
    ADD COLUMN max_items integer;
ALTER TABLE IF EXISTS public.charts
    ADD COLUMN date_range integer;
ALTER TABLE IF EXISTS public.charts
    ADD COLUMN top_left_x integer;
ALTER TABLE IF EXISTS public.charts
    ADD COLUMN top_left_y integer;
ALTER TABLE IF EXISTS public.charts
    ADD COLUMN bottom_right_x integer;
ALTER TABLE IF EXISTS public.charts
    ADD COLUMN bottom_right_y integer;

UPDATE public.charts SET max_items=5 WHERE id=1;
UPDATE public.charts SET max_items=5 WHERE id=2;
UPDATE public.charts SET max_items=10 WHERE id=3;
UPDATE public.charts SET max_items=10 WHERE id=4;
UPDATE public.charts SET max_items=10 WHERE id=5;