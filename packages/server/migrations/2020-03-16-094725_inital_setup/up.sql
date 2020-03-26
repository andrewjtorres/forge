CREATE EXTENSION IF NOT EXISTS "moddatetime";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
    "id" uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    "email" character varying NOT NULL UNIQUE,
    "hash" bytea NOT NULL,
    "created_at" timestamp with time zone DEFAULT now() NOT NULL,
    "updated_at" timestamp with time zone DEFAULT now() NOT NULL
);

COMMENT ON TABLE "users" IS 'Users of the application.';
COMMENT ON COLUMN "users"."id" IS 'The unique identifier of the user.';
COMMENT ON COLUMN "users"."email" IS 'The user''s primary email address.';
COMMENT ON COLUMN "users"."hash" IS 'The user''s encoded password hash.';
COMMENT ON COLUMN "users"."created_at" IS 'The date and time when the user was created.';
COMMENT ON COLUMN "users"."updated_at" IS 'The date and time when the user was last updated.';

CREATE TRIGGER "set_updated_at"
    BEFORE UPDATE ON "users"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime("updated_at");
