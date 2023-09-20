DROP VIEW deep_transactions;
DROP VIEW deep_assets;
DROP VIEW deep_accounts;
DROP VIEW deep_recipients;
DROP VIEW deep_tags;

ALTER TABLE IF EXISTS public.currencies
    RENAME minor_in_mayor TO minor_in_major;