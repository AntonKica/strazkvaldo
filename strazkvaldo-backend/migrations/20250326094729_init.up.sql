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
