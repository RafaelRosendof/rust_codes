# Build stage
FROM rust:latest as builder
WORKDIR /app

# Instala o compilador do Protocol Buffers necessário para o yfinance-rs
RUN apt-get update && apt-get install -y protobuf-compiler

# Copia tudo de dentro da pasta QuantFast para o container
COPY . .

# 1. Compila apontando DIRETAMENTE para o Cargo.toml do net_rabbit
RUN cargo build --release --manifest-path net_rabbit/Cargo.toml --bin producer_1

# 2. Pesca o binário gerado e salva em uma pasta /export segura
RUN mkdir -p /export && cp $(find /app -type f -name producer_1 -executable | head -n 1) /export/quant_backend

# Runtime stage (imagem mais leve)
FROM debian:bookworm-slim
WORKDIR /app

# Instala dependências de rede caso o Rust precise
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# 3. Copia o binário da nossa pasta /export para a imagem final
COPY --from=builder /export/quant_backend /app/quant_backend

# Roda o backend
CMD ["./quant_backend"]
