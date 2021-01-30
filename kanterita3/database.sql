DROP TABLE IF EXISTS db_people;

create table db_people {
	id serial PRIMARY KEY,
	identificacion VARCHAR(20)
	nombre VARCHAR(150) NOT NULL,
	genero CHAR(1),
	estado_civil VARCHAR(20),
	fecha_nacimiento DATE,
	numero_telefono BIGINT NOT NULL ,
	direccion VARCHAR(150),
	correo VARCHAR(150) NOT NULL,
	validado BOOLEAN NOT NULL DEFAULT FALSE
}

INSERT INTO db_people (identificacion, nombre, genero, estado_civil,
	fecha_nacimiento, numero_telefono, direccion, correo, validado)
	VALUES ('1111111111', 'USUARIO 1', 'M', 'SOLTERO', '1990-01-01', '1234567890', 'CALLE 1 CARRERA 2',
	'ASD@EXAMPLE.COM', FALSE );
