-- Your SQL goes here
CREATE TABLE rust_logos(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    image_path VARCHAR NOT NULL
);



insert into rust_logos(name,image_path)values ('Speed','1.jpeg');
insert into rust_logos(name,image_path)values ('Secure','2.jpeg');
insert into rust_logos(name,image_path)values ('No GC','3.jpeg');
