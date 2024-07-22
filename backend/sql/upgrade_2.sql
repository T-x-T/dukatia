CREATE TABLE IF NOT EXISTS public."Assets"
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 0 MINVALUE 0 MAXVALUE 2147483647 CACHE 1 ),
    name text COLLATE pg_catalog."default" NOT NULL,
    description text COLLATE pg_catalog."default",
    "userId" integer,
    "currencyId" integer NOT NULL,
    CONSTRAINT "Assets_pkey" PRIMARY KEY (id),
    CONSTRAINT "currencyId" FOREIGN KEY ("currencyId")
        REFERENCES public."Currencies" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    CONSTRAINT "userId" FOREIGN KEY ("userId")
        REFERENCES public."Users" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
)

TABLESPACE pg_default;

CREATE TABLE IF NOT EXISTS public."AssetAmounts"
(
    "assetId" integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    amount double precision NOT NULL,
    CONSTRAINT "AssetAmounts_pkey" PRIMARY KEY ("assetId", "timestamp"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

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

CREATE TABLE IF NOT EXISTS public."AssetTransactions"
(
    "assetId" integer NOT NULL,
    "transactionId" integer NOT NULL,
    CONSTRAINT "AssetTransactions_pkey" PRIMARY KEY ("assetId", "transactionId"),
    CONSTRAINT "transactionIdUnique" UNIQUE ("transactionId"),
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

CREATE TABLE IF NOT EXISTS public."AssetValuations"
(
    "assetId" integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "valuePerUnit" integer NOT NULL,
    CONSTRAINT "AssetValuations_pkey" PRIMARY KEY ("assetId", "timestamp"),
    CONSTRAINT "assetId" FOREIGN KEY ("assetId")
        REFERENCES public."Assets" (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

CREATE VIEW public."AssetData"
 AS
SELECT a.id, a.name, a.description, a."userId", a."currencyId", array_agg(t."tagId") as tags, aa.amount, av."valuePerUnit"
	FROM public."Assets" a
	INNER JOIN public."AssetAmounts" aa 
		ON a.id = aa."assetId" 
		AND aa.timestamp = (
			SELECT max(timestamp)
				FROM public."AssetAmounts" 
				WHERE "assetId" = a.id
				GROUP BY "assetId"
		)
	INNER JOIN public."AssetValuations" av 
		ON a.id = av."assetId"
		AND av.timestamp = (
			SELECT max(timestamp)
				FROM public."AssetValuations"
				WHERE "assetId" = a.id
				GROUP BY "assetId" 
		)
	LEFT JOIN public."AssetTags" t 
		ON a.id = t."assetId"
	GROUP BY a.id, aa.amount, av."valuePerUnit"
	ORDER BY a.id;

CREATE OR REPLACE VIEW public."TransactionData"
 AS
 SELECT tr.id,
    tr.account,
    tr.currency,
    tr.recipient,
    tr.status,
    tr."user",
    tr."timestamp",
    tr.amount,
    tr.comment,
    array_agg(t.tag) AS tags,
    a.id AS "assetId",
    a.name AS "assetName",
	a.description AS "assetDescription"
   FROM "Transactions" tr
     LEFT JOIN "TransactionTags" t ON tr.id = t.transaction
     LEFT JOIN "AssetTransactions" at ON at."transactionId" = tr.id
     LEFT JOIN "Assets" a ON a.id = at."assetId"
  GROUP BY tr.id, a.id
  ORDER BY tr.id;