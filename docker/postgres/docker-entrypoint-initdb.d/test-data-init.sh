#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" postgres <<-EOSQL
    create table users
    (
        name VARCHAR(255) not null,
        password VARCHAR(255) not null
    );
    INSERT INTO public.users (name, password) VALUES ('user', 'password123');
    INSERT INTO public.users (name, password) VALUES ('double', 'password123');
    INSERT INTO public.users (name, password) VALUES ('double', 'password123');
EOSQL