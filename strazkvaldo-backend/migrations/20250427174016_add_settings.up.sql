CREATE TABLE app_settings
(
    _lock                              BIT(1)  NOT NULL
        CONSTRAINT DF_T1_LOCK DEFAULT B'1',
    auto_review_finished_activity      BOOLEAN NOT NULL,
    auto_review_finished_activity_days INT     NOT NULL,
    constraint PK_T1 PRIMARY KEY (_lock),
    constraint CK_T1_LOCKED CHECK (_lock = B'1')
);

INSERT INTO app_settings (auto_review_finished_activity, auto_review_finished_activity_days)
VALUES (false, 0);