ALTER TABLE public.dashboards DISABLE TRIGGER ALL;
INSERT INTO public.dashboards VALUES (DEFAULT, 0, 'Default', 'The default Dashboard');
ALTER TABLE public.dashboards ENABLE TRIGGER ALL;