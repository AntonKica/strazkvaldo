CREATE TABLE IF NOT EXISTS one_time_activity("code" CHAR(8) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL UNIQUE,
    "activity_type" INT NOT NULL,
    "criticality_type" INT NOT NULL,
    "duration_in_seconds" INT NOT NULL,
    "description" TEXT NOT NULL,
    "date" TIMESTAMPTZ NOT NULL
    );
CREATE TABLE IF NOT EXISTS repeated_activity("code" CHAR(7) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL UNIQUE,
    "activity_type" INT NOT NULL,
    "criticality_type" INT NOT NULL,
    "duration_in_seconds" INT NOT NULL,
    "description" TEXT NOT NULL,
    "periodicity" VARCHAR(80) NOT NULL,
    "start_date" TIMESTAMPTZ NOT NULL,
    "end_date" TIMESTAMPTZ NOT NULL
    );
CREATE TABLE IF NOT EXISTS app_user("code" CHAR(8) PRIMARY KEY NOT NULL,
    "first_name" VARCHAR(80) NOT NULL,
    "last_name" VARCHAR(80) NOT NULL,
    "email" VARCHAR(80) NOT NULL,
    "username" VARCHAR(80) NOT NULL,
    "password_hash" CHAR(128) NOT NULL,
    "app_user_role" INT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    "updated" TIMESTAMPTZ NOT NULL
    );

INSERT INTO app_user(code,
                     first_name,
                     last_name,
                     email,
                     username,
                     password_hash,
                     app_user_role,
                     created,
                     updated
)
VALUES ('USR-0000',
        'Admin',
        'Admin',
        'admin@admin.com',
        'admin',
        encode(digest('admin123', 'sha512'), 'hex'),
        0, -- AppUserRole.Admin
        NOW(),
        NOW()
       )
