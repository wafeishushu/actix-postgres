CREATE DATABASE ezytutors; 
CREATE USER truuser;
GRANT ALL PRIVILEGES ON DATABASE ezytutors TO truuser;
GRANT ALL ON SCHEMA public TO truuser;
ALTER USER postgres WITH PASSWORD 'postgres';
ALTER USER truuser  with PASSWORD 'trupwd';


