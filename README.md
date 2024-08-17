# SeaORM の使い方

1. docker-compose.yaml を用意 (DB を用意)

| Database           | Example Database URL                |
| ------------------ | ----------------------------------- |
| MySQL              | mysql://root:root@localhost:3306    |
| PostgreSQL         | postgres://root:root@localhost:5432 |
| SQLite (in file)   | sqlite:./sqlite.db?mode=rwc         |
| SQLite (in memory) | sqlite::memory:                     |

PostgreSQL の場合は以下のようになる。

データベース名: bakeries_db

ポート: 5433

プロジェクト配下に設置したマウント用のディレクトリ: db

```yaml
version: "3.8"

services:
  db:
    container_name: postgres
    image: postgres:latest
    volumes:
      - ./db:/var/lib/postgresql
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: bakeries_db
    ports:
      - "5433:5432"
```

2. sea-orm-cli をインストール

```bash
cargo install sea-orm-cli
```

3. プロジェクトのセットアップ

```bash
cargo new tutorial
cd tutorial
```

| Database          | DATABASE_DRIVER |
| ----------------- | --------------- |
| MySQL and MariaDB | sqlx-mysql      |
| PostgreSQL        | sqlx-postgres   |
| SQLite            | sqlx-sqlite     |

| ASYNC_RUNTIME                | Web Framework             |
| ---------------------------- | ------------------------- |
| runtime-async-std-native-tls | Tide                      |
| runtime-async-std-rustls     | Tide                      |
| runtime-tokio-native-tls     | Axum, Actix, Poem, Rocket |
| runtime-tokio-rustls         | Axum, Actix, Poem, Rocket |

`Cargo.toml`

```toml
[dependencies]
sea-orm = { version = "^0.12.0", features = [ "<DATABASE_DRIVER>", "<ASYNC_RUNTIME>", "macros" ] }
```

設定の例

```toml
sea-orm = { version = "^0.12.0", features = [ "sqlx-postgres", "runtime-async-std-native-tls", "macros", "mock" ] }
```

4. マイグレーションの初期化

```bash
sea-orm-cli migrate init
```

以下のような構成になっていたら OK である。

```
.
├── Cargo.toml
├── migration
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       └── main.rs
└── src
    └── main.rs
```

5. マイグレーションファイルの生成

`migration/src/`に以下のルールに従って各モデルごとにファイルを生成する。

`m<date>_<6-digit-index>_<description>.rs`

ファイルに中身を書いていく。

`migration/Cargo.toml`

```toml
[dependencies.sea-orm-migration]
features = [
  "sqlx-postgres",
  "runtime-async-std-native-tls",
]
```

6. マイグレーションの実行

データベース URL は、一例である。

```bash
ATABASE_URL="postgres://postgres:podtgres@localhost:5433/bakeries_db" sea-orm-cli migrate refresh
```

7. エンティティの生成

```bash
sea-orm-cli generate entity \
    -u postgres://postgres:podtgres@localhost:5433/bakeries_db \
    -o src/entities
```

以下のような構成になっていたら OK である。

```
bakery-backend
├── Cargo.toml
├── migration
│   └── ...
└── src
    ├── entities
    │   ├── chef.rs
    │   ├── bakery.rs
    │   ├── mod.rs
    │   └── prelude.rs
    └── main.rs
```

8. 準備完了！

## 参考サイト

https://www.sea-ql.org/sea-orm-tutorial
