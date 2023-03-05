CREATE TABLE public."Meta"
(
    schema_version integer
);

ALTER TABLE IF EXISTS public."Meta"
    OWNER to postgres;

INSERT INTO public."Meta" (schema_version) VALUES (0);