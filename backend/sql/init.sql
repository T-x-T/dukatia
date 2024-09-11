SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;
SET default_tablespace = '';
SET default_table_access_method = heap;
CREATE TABLE public."AccessTokens" (
    id integer NOT NULL,
    "user" integer NOT NULL,
    token text NOT NULL
);
CREATE TABLE public."AccountTags" (
    account integer NOT NULL,
    tag integer NOT NULL
);
CREATE TABLE public."Accounts" (
    id integer NOT NULL,
    name text NOT NULL,
    defaultcurrency integer NOT NULL,
    "user" integer
);
ALTER TABLE public."Accounts" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Accounts_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
CREATE TABLE public."Currencies" (
    id integer NOT NULL,
    name text NOT NULL,
    "minorinmayor" integer,
    symbol text NOT NULL
);
ALTER TABLE public."Currencies" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Currencies_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
CREATE TABLE public."RecipientTags" (
    recipient integer NOT NULL,
    tag integer NOT NULL
);
CREATE TABLE public."Recipients" (
    id integer NOT NULL,
    name text NOT NULL,
    "user" integer
);
ALTER TABLE public."Recipients" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Recipients_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
CREATE TABLE public."Tags" (
    id integer NOT NULL,
    name text NOT NULL,
    parent integer,
    "user" integer
);
ALTER TABLE public."Tags" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Tags_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
CREATE TABLE public."Transactions" (
    id integer NOT NULL,
    "user" integer NOT NULL,
    account integer NOT NULL,
    currency integer NOT NULL,
    recipient integer NOT NULL,
    status integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    amount integer NOT NULL,
    comment text
);
CREATE TABLE public."TransactionTags" (
    transaction integer NOT NULL,
    tag integer NOT NULL
);
ALTER TABLE public."Transactions" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Transactions_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
CREATE TABLE public."Users" (
    id integer UNIQUE NOT NULL,
    name text NOT NULL,
    secret text NOT NULL,
    permissions json,
    superuser boolean DEFAULT false NOT NULL
);
ALTER TABLE public."Users" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Users_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
ALTER TABLE public."AccessTokens" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."AccessTokens_id_seq"
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);
SELECT pg_catalog.setval('public."Accounts_id_seq"', 0, false);
SELECT pg_catalog.setval('public."Currencies_id_seq"', 0, false);
SELECT pg_catalog.setval('public."Recipients_id_seq"', 0, false);
SELECT pg_catalog.setval('public."Tags_id_seq"', 0, false);
SELECT pg_catalog.setval('public."Transactions_id_seq"', 0, false);
SELECT pg_catalog.setval('public."Users_id_seq"', 0, false);
SELECT pg_catalog.setval('public."AccessTokens_id_seq"', 0, false);
ALTER TABLE ONLY public."AccountTags"
    ADD CONSTRAINT "AccountTags_pkey" PRIMARY KEY (account, tag);
ALTER TABLE ONLY public."Accounts"
    ADD CONSTRAINT "Account_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."Accounts"
    ADD CONSTRAINT "user" FOREIGN KEY ("user") REFERENCES public."Users"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."Currencies"
    ADD CONSTRAINT "Currency_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."RecipientTags"
    ADD CONSTRAINT "RecipientTags_pkey" PRIMARY KEY (recipient, tag);
ALTER TABLE ONLY public."Recipients"
    ADD CONSTRAINT "Recipients_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."Recipients"
    ADD CONSTRAINT "user" FOREIGN KEY ("user") REFERENCES public."Users"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."Tags"
    ADD CONSTRAINT "Tag_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."Tags"
    ADD CONSTRAINT "user" FOREIGN KEY ("user") REFERENCES public."Users"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."TransactionTags"
    ADD CONSTRAINT "TransactionTags_pkey" PRIMARY KEY (transaction, tag);
ALTER TABLE ONLY public."Transactions"
    ADD CONSTRAINT "Transactions_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."Users"
    ADD CONSTRAINT "Users_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."AccessTokens"
    ADD CONSTRAINT "accessTokens_pkey" PRIMARY KEY (id);
ALTER TABLE ONLY public."AccessTokens"
    ADD CONSTRAINT "user" FOREIGN KEY ("user") REFERENCES public."Users"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."AccountTags"
    ADD CONSTRAINT account FOREIGN KEY (account) REFERENCES public."Accounts"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."Transactions"
    ADD CONSTRAINT account FOREIGN KEY (account) REFERENCES public."Accounts"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."Transactions"
    ADD CONSTRAINT currency FOREIGN KEY (currency) REFERENCES public."Currencies"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."Accounts"
    ADD CONSTRAINT defaultcurrency FOREIGN KEY (defaultcurrency) REFERENCES public."Currencies"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."RecipientTags"
    ADD CONSTRAINT recipient FOREIGN KEY (recipient) REFERENCES public."Recipients"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."Transactions"
    ADD CONSTRAINT recipient FOREIGN KEY (recipient) REFERENCES public."Recipients"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
ALTER TABLE ONLY public."AccountTags"
    ADD CONSTRAINT tag FOREIGN KEY (tag) REFERENCES public."Tags"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."RecipientTags"
    ADD CONSTRAINT tag FOREIGN KEY (tag) REFERENCES public."Tags"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."TransactionTags"
    ADD CONSTRAINT tag FOREIGN KEY (tag) REFERENCES public."Tags"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."TransactionTags"
    ADD CONSTRAINT transaction FOREIGN KEY (transaction) REFERENCES public."Transactions"(id) ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE ONLY public."Transactions"
    ADD CONSTRAINT "user" FOREIGN KEY ("user") REFERENCES public."Users"(id) ON UPDATE CASCADE ON DELETE RESTRICT;
INSERT INTO public."Currencies" (id, name, minorinmayor, symbol) VALUES (DEFAULT, 'Euro', 100, '€'), (DEFAULT, 'USD', 100, '$');
INSERT INTO public."Accounts" (id, name, defaultcurrency) VALUES (DEFAULT, 'Default', 0);
INSERT INTO public."Recipients" (id, name) VALUES (DEFAULT, 'Default');