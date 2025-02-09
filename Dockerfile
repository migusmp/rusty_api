# rusty_api/Dockerfile
FROM rust:latest

WORKDIR /usr/src/app

# Copiar archivos de configuración y dependencias
COPY rusty_api/Cargo.toml rusty_api/Cargo.lock ./

# Instalar dependencias
RUN cargo build --release

# Copiar todo el código fuente
COPY rusty_api/ .

# Volver a compilar después de copiar el código
RUN cargo build --release

# Exponer el puerto del servicio
EXPOSE 3000

# Comando para ejecutar el servicio
CMD ["cargo", "run", "--release"]
