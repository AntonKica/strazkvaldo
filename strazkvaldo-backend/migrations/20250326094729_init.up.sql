CREATE TABLE IF NOT EXISTS one_time_activity("code" CHAR(8) PRIMARY KEY NOT NULL,
                                             "name" VARCHAR(80) NOT NULL UNIQUE,
                                             "activity_type" INT NOT NULL,
                                             "criticality_type" INT NOT NULL,
                                             "duration_in_seconds" INT NOT NULL,
                                             "description" TEXT NOT NULL,
                                             "date" TIMESTAMPTZ NOT NULL
);
