# Introduction

This project is a demo Actix web application to show how to get started with Actix and Diesel ORM.

## PostgreSQL Notes

PostgresDB Docker:

    * Link: <https://hub.docker.com/_/postgres>
    * Command: docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
    * username: postgres
    * password: mysecretpassword

Diesel CLI:
cargo install diesel_cli --no-default-features --features postgres

Diesel CLI Windows tips:

    * Get postgress from : https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
    * setx MYSQLCLIENT_LIB_DIR "C:\Program Files\MySQL\MySQL Connector C 6.1\lib\vs14"
    * setx PQ_LIB_DIR "C:\PostgreSQL\12\lib"
    * Finally you can install diesel client through below:
    cargo install diesel_cli --no-default-features --features "sqlite-bundled mysql postgres" --force
