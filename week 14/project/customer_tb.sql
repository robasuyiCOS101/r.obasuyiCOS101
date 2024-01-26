--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

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

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_email text NOT NULL,
    c_mobile bigint NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	m_karim@gmail.com	8055089112	102	5
111	Lilian Jaiya	I_jaiye@gmail.com	8055185341	100	3
112	Arthur Musa	A_musa@gmail.com	7055282813	107	10
113	Philip Akonjo	p_akonjo@gmail.com	9052356772	100	2
114	Marylene Mapa	m_mapa@gmail.com	8053333551	120	5
115	Oghenero Agor	o_agor@gmail.com	7055566774	117	11
116	Adams Bree	a_bree@gmail.com	8056765424	102	1
117	Okafor Mathias	o_mathais@gmail.com	8056763367	102	1
118	Samson Adeleke	s_adeleke@gmail.com	7056774423	117	11
119	Lawal Tamire	l_tamire@gmail.com	9052111101	107	15
120	James Job	j_job@gmail.com	8059693919	100	8
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

