CREATE TABLE IF NOT EXISTS "user"
(
    id         text PRIMARY KEY not null DEFAULT gen_random_uuid(),
    created_at date             not null,
    updated_at date             not null,
    deleted_at date,
    email      varchar(255)     not null unique,
    password   varchar(255)     not null
);

ALTER TABLE IF EXISTS "user"
    ADD IF NOT EXISTS created_at date not null default now(),
    ADD IF NOT EXISTS updated_at date not null default now(),
    ADD IF NOT EXISTS deleted_at date;

CREATE OR REPLACE FUNCTION f_set_creation_date()
    RETURNS TRIGGER AS
$$
BEGIN
    New.created_at = NOW();
    New.updated_at = NOW();
    RETURN New;
END;
$$ language 'plpgsql';

CREATE OR REPLACE FUNCTION f_set_update_date()
    RETURNS TRIGGER AS
$$
BEGIN
    New.updated_at = NOW();
    RETURN New;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER t_auto_creation_date
    BEFORE INSERT
    ON "user"
    FOR EACH ROW
EXECUTE PROCEDURE f_set_creation_date();


CREATE OR REPLACE TRIGGER t_auto_updated_date
    BEFORE UPDATE
    ON "user"
    FOR EACH ROW
EXECUTE PROCEDURE f_set_update_date();
