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

CREATE TABLE IF NOT EXISTS room("code" CHAR(9) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL,
    "room_type" INT NOT NULL,
    "description" TEXT NOT NULL
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
        'admin@strazkvaldo.com',
        'admin',
        encode(digest('admin123', 'sha512'), 'hex'),
        0, -- AppUserRole.Admin
        NOW(),
        NOW()
       ),
       ('USR-0001',
        'Janko',
        'Hrasko',
        'jankohrasko@strazkvaldo.com',
        'jankohrasko',
        encode(digest('jankohrasko', 'sha512'), 'hex'),
        1, -- AppUserRole.User
        NOW(),
        NOW()
       )
;

INSERT INTO one_time_activity("code",
                              "name",
                              "activity_type",
                              "criticality_type",
                              "duration_in_seconds",
                              "description",
                              "date"
)
VALUES ('OTA-0001',
        'Dirty clothes',
        0,
        0,
        1800,
        'Got some dirty clothes',
        NOW()
       ),
       ('OTA-0002',
        'Dirty floor',
        0,
        0,
        1800,
        'There is some dirty kitchen',
        NOW()
       )
;

INSERT INTO repeated_activity("code",
                              "name",
                              "activity_type",
                              "criticality_type",
                              "duration_in_seconds",
                              "description",
                              "periodicity",
                              "start_date",
                              "end_date"
)
VALUES ('RA-0001',
        'Washing',
        0,
        0,
        3600,
        'Periodical washing of clothes',
        'MONTH',
        NOW(),
        NOW()
       ),
       ('RA-0002',
        'Dirty floor mopping',
        0,
        0,
        1800,
        'Periodical floor mopping',
        'MONTH',
        NOW(),
        NOW()
       )
;

INSERT INTO room("code",
                 "name",
                 "room_type",
                 "description"
)
VALUES ('ROOM-0001',
        'Bathroom',
        0,
        'Just a one bathroom alright'
       ),
       ('ROOM-0002',
        'Bedroom',
        1,
        'Just mine bedroom'
       )
;
