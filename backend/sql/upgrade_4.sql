CREATE OR REPLACE VIEW public."AssetValuationHistory"
 AS
SELECT COALESCE(aa."assetId", av."assetId") AS assetId, COALESCE(aa.timestamp, av.timestamp) AS timestamp, aa.amount, av."valuePerUnit" FROM public."AssetAmounts" aa
FULL OUTER JOIN public."AssetValuations" av ON aa."assetId" = av."assetId" AND aa.timestamp = av.timestamp
ORDER BY assetId ASC, timestamp ASC;