# Usa una imagen base de Rust oficial
FROM rust:latest as builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de Cargo.toml y Cargo.lock primero
COPY Cargo.toml Cargo.lock ./

# Crear un espacio vacío para dependencias y construir las dependencias
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar el resto del código
COPY . .

# Compilar el proyecto para la release final
RUN cargo build --release

# Segunda etapa: configuración del contenedor ligero
FROM debian:buster-slim

# Crear usuario para seguridad
RUN useradd -m rustacean

# Copiar binario desde la primera fase
COPY --from=builder /app/target/release/holamundo /usr/local/bin/holamundo

# Configurar el puerto de la aplicación
EXPOSE 8000

# Establecer usuario y comando de inicio
USER rustacean
CMD ["holamundo"]
