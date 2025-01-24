CREATE TABLE IF NOT EXISTS "users"
(
    "id"            BIGINT GENERATED ALWAYS AS IDENTITY,
    "username"      VARCHAR(256) NOT NULL,
    "email"         VARCHAR(256) NOT NULL,
    "password_hash" VARCHAR(256) NOT NULL,
    "created_at"    TIMESTAMP WITH TIME ZONE DEFAULT (current_timestamp),
    "updated_at"    TIMESTAMP WITH TIME ZONE DEFAULT (current_timestamp),
    CONSTRAINT "pk_users" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_users_username" ON users ("username");
CREATE UNIQUE INDEX IF NOT EXISTS "uq_users_email" ON users ("email");

CREATE TRIGGER "tgr_users_updated_at"
    BEFORE UPDATE
    ON "users"
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

