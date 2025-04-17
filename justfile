set dotenv-required := true
set dotenv-filename := ".env"
set dotenv-load := true

build:
    spin build

up:
    spin up

deploy:
    spin cloud deploy \
        --variable surrealdb_host="$SPIN_VARIABLE_SURREALDB_HOST" \
        --variable surrealdb_user="$SPIN_VARIABLE_SURREALDB_USER" \
        --variable surrealdb_password="$SPIN_VARIABLE_SURREALDB_PASSWORD" \
        --variable surrealdb_namespace="$SPIN_VARIABLE_SURREALDB_NAMESPACE" \
        --variable surrealdb_database="$SPIN_VARIABLE_SURREALDB_DATABASE"

all: build up
