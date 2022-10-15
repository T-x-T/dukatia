CREATE TABLE IF NOT EXISTS public."Assets"
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    name text COLLATE pg_catalog."default" NOT NULL,
    description text COLLATE pg_catalog."default",
    "userId" integer,
    CONSTRAINT "Assets_pkey" PRIMARY KEY (id),
    CONSTRAINT "userId" FOREIGN KEY ("userId")
        REFERENCES public."Users" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."Assets"
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public."AssetCounts"
(
    "assetId" integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    count integer NOT NULL,
    CONSTRAINT "AssetCounts_pkey" PRIMARY KEY ("assetId", "timestamp"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."AssetCounts"
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public."AssetTags"
(
    "assetId" integer NOT NULL,
    "tagId" integer NOT NULL,
    CONSTRAINT "AssetTags_pkey" PRIMARY KEY ("assetId", "tagId"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT "tagId" FOREIGN KEY ("tagId")
        REFERENCES public."Tags" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."AssetTags"
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public."AssetTransactions"
(
    "assetId" integer NOT NULL,
    "transactionId" integer NOT NULL,
    CONSTRAINT "AssetTransactions_pkey" PRIMARY KEY ("assetId", "transactionId"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT "transactionId" FOREIGN KEY ("transactionId")
        REFERENCES public."Transactions" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."AssetTransactions"
    OWNER to postgres;

CREATE TABLE IF NOT EXISTS public."AssetValuations"
(
    "assetId" integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "valuePerCount" integer NOT NULL,
    CONSTRAINT "AssetValuations_pkey" PRIMARY KEY ("assetId", "timestamp"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public."AssetValuations"
    OWNER to postgres;