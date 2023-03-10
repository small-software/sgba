-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    -- id uuid     DEFAULT uuid_generate_v4 (),
    id SERIAL,
    name        VARCHAR(100),
    email       VARCHAR(254),
    passwd      VARCHAR(128),
    created_at  timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at  timestamp with time zone DEFAULT (now() at time zone 'utc'),
    PRIMARY KEY(id),
    CONSTRAINT name_email_passwd_notnull CHECK (
       NOT (
                ( name IS NULL  OR  name = '' )
                AND
                ( email IS NULL  OR  email = '' )
                AND
                ( passwd IS NULL  OR  passwd = '' )
       )
     )
);