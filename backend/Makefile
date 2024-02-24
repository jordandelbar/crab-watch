app_run:
	cargo run

surrealdb_run:
	docker run -d --rm --pull always --name surrealdb_dev \
	-p 8000:8000 surrealdb/surrealdb:latest \
	start --log trace --auth \
	--user root --pass root

run: surrealdb_run app_run

stop:
	docker stop surrealdb_dev

