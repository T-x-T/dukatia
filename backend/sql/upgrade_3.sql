CREATE OR REPLACE VIEW public."AssetData"
    AS
     SELECT a.id,
    a.name,
    a.description,
    a."userId",
    a."currencyId",
    array_agg(t."tagId") AS tags,
    aa.amount,
    av."valuePerUnit"
   FROM "Assets" a
     LEFT JOIN "AssetAmounts" aa ON a.id = aa."assetId" AND aa."timestamp" = (( SELECT max("AssetAmounts"."timestamp") AS max
           FROM "AssetAmounts"
          WHERE "AssetAmounts"."assetId" = a.id
          GROUP BY "AssetAmounts"."assetId"))
     LEFT JOIN "AssetValuations" av ON a.id = av."assetId" AND av."timestamp" = (( SELECT max("AssetValuations"."timestamp") AS max
           FROM "AssetValuations"
          WHERE "AssetValuations"."assetId" = a.id
          GROUP BY "AssetValuations"."assetId"))
     LEFT JOIN "AssetTags" t ON a.id = t."assetId"
  GROUP BY a.id, aa.amount, av."valuePerUnit"
  ORDER BY a.id;