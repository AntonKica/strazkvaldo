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
    "criticality_type" VARCHAR(16) NOT NULL,
    "duration_in_seconds" INT NOT NULL,
    "description" TEXT NOT NULL,
    "date" TIMESTAMPTZ NOT NULL
    );
CREATE TABLE IF NOT EXISTS repeated_activity("code" CHAR(7) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL UNIQUE,
    "activity_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "criticality_type" VARCHAR(16) NOT NULL,
    "duration_in_seconds" INT NOT NULL,
    "description" TEXT NOT NULL,
    "periodicity" VARCHAR(16) NOT NULL,
    "periodicity_unit" INT NOT NULL
    );
CREATE TABLE IF NOT EXISTS app_user("code" CHAR(8) PRIMARY KEY NOT NULL,
    "first_name" VARCHAR(80) NOT NULL,
    "last_name" VARCHAR(80) NOT NULL,
    "email" VARCHAR(80) NOT NULL,
    "username" VARCHAR(80) NOT NULL,
    "password_hash" CHAR(128) NOT NULL,
    "app_user_role" VARCHAR(16) NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    "updated" TIMESTAMPTZ NOT NULL
    );

CREATE TABLE IF NOT EXISTS room("code" CHAR(9) PRIMARY KEY NOT NULL,
    "name" VARCHAR(80) NOT NULL,
    "room_type" VARCHAR(16) NOT NULL REFERENCES enum_values("code"),
    "description" TEXT NOT NULL
    );

CREATE TABLE IF NOT EXISTS finished_activity("code" CHAR(9) PRIMARY KEY NOT NULL,
    "one_time_activity_code" CHAR(8) NULL REFERENCES one_time_activity("code"),
    "repeated_activity_code" CHAR(7) NULL REFERENCES repeated_activity("code"),
    "due_date" DATE NOT NULL,
    "description" TEXT NOT NULL,
    "reviewed" BOOLEAN NOT NULL,
    UNIQUE ("one_time_activity_code", "due_date"),
    UNIQUE ("repeated_activity_code", "due_date")
    );

INSERT INTO enum_values("name", "code", "text")
VALUES ('activity-type', 'Washing', 'umývanie'),
       ('activity-type', 'Mopping', 'mopovanbie'),
       ('activity-type', 'Cleaning', 'čistenie'),
       ('activity-type', 'Vacuuming', 'vysávanie'),
       ('activity-type', 'Dusting', 'utieranie prachu'),
       ('activity-type', 'Garbage-disposal', 'vynášanie smetí'),
       ('activity-type', 'Other-activity', 'iná aktivita'),
       ('room-type', 'Bathroom', 'kúpeľňa'),
       ('room-type', 'Bedroom', 'spálňa'),
       ('room-type', 'Living-room', 'obývačka'),
       ('room-type', 'Kitchen', 'kuchyňa'),
       ('room-type', 'Balcony', 'balkón'),
       ('room-type', 'Work-room', 'pracovná miestnosť'),
       ('room-type', 'Garage', 'garáž'),
       ('room-type', 'Cellar', 'pivnica'),
       ('room-type', 'Other-room', 'iná miestnosť');

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
        'Admin',
        NOW(),
        NOW()
       ),
       ('USR-0001',
        'Janko',
        'Hrasko',
        'jankohrasko@strazkvaldo.com',
        'jankohrasko',
        encode(digest('jankohrasko', 'sha512'), 'hex'),
        'User',
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
        'Washing',
        'Low',
        1800,
        'Got some dirty clothes',
        NOW()
       ),
       ('OTA-0002',
        'Dirty floor',
        'Cleaning',
        'Low',
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
                              "periodicity_unit"
)
VALUES ('RA-0001',
        'Washing',
        'Washing',
        'Low',
        3600,
        'Periodical washing of clothes',
        'Month',
        1
       ),
       ('RA-0002',
        'Dirty floor mopping',
        'Cleaning',
        'Low',
        1800,
        'Periodical floor mopping',
        'Day',
        1
       )
;

INSERT INTO room("code",
                 "name",
                 "room_type",
                 "description"
)
VALUES ('ROOM-0001',
        'Bathroom',
        'Bathroom',
        'Just a one bathroom alright'
       ),
       ('ROOM-0002',
        'Bedroom',
        'Bedroom',
        'Just mine bedroom'
       )
;
