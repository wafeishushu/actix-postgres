# actix-postgres
A simple project with actix and postgres


### Prerequisite
- Docker
- Rust

### Run
1. Start db container and initialize the database 
    ```bash
    docker compose up db -d
    docker ps
    # you can see the postgres container here
    
    docker exec -it tutor-postgres /bin/bash
    ```
   Inside the container, execute the sql that initializes the database
   ```bash
   psql -U postgres
   # execute in pgsql:
   \i /docker-entrypoint-initdb.d/initdb.sql
   \q
   ```
   then switch to truuser and initializes the tables
   ```bash
   psql -U truuser ezytutors
   # execute in pgsql:
   \i /docker-entrypoint-initdb.d/init-tables.sql
   \q
   ```
   now exit the container.

2. build tutor web-service image
   ```bash
   docker compose build api
   ```

3. change `.env` file, choose the database url which used in container network
   ```dotenv
   # for building image
   # DATABASE_URL=postgres://truuser:trupwd@localhost:5433/ezytutors

   # for running container
   DATABASE_URL=postgres://truuser:trupwd@tutor-postgres:5432/ezytutors
   HOST_PORT=0.0.0.0:3000
   ```

4. start api container
   ```bash
   source .env
   docker compose up -d
   ```
   
5. now check the docker ps
   ```
   $ docker ps -a                
   CONTAINER ID   IMAGE                COMMAND                  CREATED         STATUS         PORTS                    NAMES
   c57f10f6b250   actix-postgres-api   "./main"                 4 seconds ago   Up 3 seconds   0.0.0.0:3000->3000/tcp   tutor-webservice
   279d34c6e6f1   postgres:latest      "docker-entrypoint.s…"   5 minutes ago   Up 5 minutes   0.0.0.0:5433->5432/tcp   tutor-postgres
   ```
