FROM rust:latest

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# cargo-generateをインストール
RUN cargo install --locked cargo-generate

# 作業ディレクトリを作成
WORKDIR /app

COPY . /app

# デフォルトのシェルを実行
CMD ["/bin/bash"]
