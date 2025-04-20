CREATE TABLE IF NOT EXISTS enum_values(
    "name" VARCHAR(16) NOT NULL,
    "code" VARCHAR(16) NOT NULL,
    "text" VARCHAR(80) NOT NULL,
    PRIMARY KEY("code"),
    UNIQUE ("name", "code")
    );

CREATE TABLE IF NOT EXISTS one_time_activity("code" CHAR(8) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL UNIQUE,
    "activity_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "criticality_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "duration_in_seconds" INT NOT NULL,
    "description" TEXT NOT NULL,
    "date" TIMESTAMPTZ NOT NULL
    );
CREATE TABLE IF NOT EXISTS repeated_activity("code" CHAR(7) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL UNIQUE,
    "activity_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "criticality_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
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
    "app_user_role" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "created" TIMESTAMPTZ NOT NULL,
    "updated" TIMESTAMPTZ NOT NULL
    );

CREATE TABLE IF NOT EXISTS room("code" CHAR(9) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL,
    "room_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "description" TEXT NOT NULL
    );

INSERT INTO enum_values("name", "code", "text")
VALUES ('app-user-role', 'admin', 'administrátor'),
       ('app-user-role', 'user', 'používateľ'),
       ('criticality-type', 'low', 'nízka'),
       ('criticality-type', 'medium', 'stredná'),
       ('criticality-type', 'high', 'vysoká'),
       ('activity-type', 'washing', 'umývanie'),
       ('activity-type', 'mopping', 'mopovanbie'),
       ('activity-type', 'cleaning', 'čistenie'),
       ('activity-type', 'vacuuming', 'vysávanie'),
       ('activity-type', 'dusting', 'utieranie prachu'),
       ('activity-type', 'garbage-disposal', 'vynášanie smetí'),
       ('activity-type', 'other-activity', 'iná aktivita'),
       ('room-type', 'bathroom', 'kúpeľňa'),
       ('room-type', 'bedroom', 'spálňa'),
       ('room-type', 'living-room', 'obývačka'),
       ('room-type', 'kitchen', 'kuchyňa'),
       ('room-type', 'balcony', 'balkón'),
       ('room-type', 'work-room', 'pracovná miestnosť'),
       ('room-type', 'garage', 'garáž'),
       ('room-type', 'cellar', 'pivnica'),
       ('room-type', 'other-room', 'iná miestnosť');

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
        'admin',
        NOW(),
        NOW()
       ),
       ('USR-0001',
        'Janko',
        'Hrasko',
        'jankohrasko@strazkvaldo.com',
        'jankohrasko',
        encode(digest('jankohrasko', 'sha512'), 'hex'),
        'user',
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
        'washing',
        'low',
        1800,
        'Got some dirty clothes',
        NOW()
       ),
       ('OTA-0002',
        'Dirty floor',
        'cleaning',
        'low',
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
        'washing',
        'low',
        3600,
        'Periodical washing of clothes',
        'MONTH',
        NOW(),
        NOW()
       ),
       ('RA-0002',
        'Dirty floor mopping',
        'cleaning',
        'low',
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
        'bathroom',
        'Just a one bathroom alright'
       ),
       ('ROOM-0002',
        'Bedroom',
        'bedroom',
        'Just mine bedroom'
       )
;
