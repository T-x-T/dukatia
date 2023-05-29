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

ALTER TABLE IF EXISTS public.charts DROP COLUMN IF EXISTS grid_size;

UPDATE public.charts SET top_left_x=0, top_left_y=0, bottom_right_x=1, bottom_right_y=2 WHERE id=0;
UPDATE public.charts SET max_items=5, top_left_x=1, top_left_y=0, bottom_right_x=3, bottom_right_y=2 WHERE id=1;
UPDATE public.charts SET max_items=5, top_left_x=3, top_left_y=0, bottom_right_x=5, bottom_right_y=2 WHERE id=2;
UPDATE public.charts SET max_items=10, date_range=6, top_left_x=5, top_left_y=0, bottom_right_x=10, bottom_right_y=2 WHERE id=3;
UPDATE public.charts SET max_items=10, date_range=6, top_left_x=0, top_left_y=2, bottom_right_x=5, bottom_right_y=4 WHERE id=4;
UPDATE public.charts SET max_items=10, date_range=6, top_left_x=5, top_left_y=2, bottom_right_x=10, bottom_right_y=4 WHERE id=5;
UPDATE public.charts SET date_range=7, top_left_x=0, top_left_y=4, bottom_right_x=10, bottom_right_y=6 WHERE id=6;