localhost-db-recreate: localhost-db-clean localhost-db-setup

localhost-db-setup:
	sqlx migrate run --database-url postgres://dev_user@localhost/strazkvaldo_local

localhost-db-clean:
	sqlx migrate revert --database-url postgres://dev_user@localhost/strazkvaldo_local
