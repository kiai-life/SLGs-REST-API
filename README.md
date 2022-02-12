[![Build Status][ci-badge]][ci]
[![source badge][source-badge]][source]
[![rustc badge][rustc-badge]][rustc]


[ci]: https://github.com/kiai-life/SLGs-REST-API/actions/workflows/rust-ci.yaml
[ci-badge]: https://github.com/kiai-life/SLGs-REST-API/actions/workflows/rust-ci.yaml/badge.svg
[source]: https://github.com/kiai-life/SLGs-REST-API
[source-badge]: https://img.shields.io/badge/source-github-blue
[rustc]: https://github.com/kiai-life/SLGs-REST-API
[rustc-badge]: https://img.shields.io/badge/rustc-1.58.1-blue


# 必要なソフトウェア

- [Rust](https://www.rust-lang.org/tools/install)：1.58.1以上
- openssl
- mdbook：ドキュメントビルド用
- PostgreSQL：データベース操作用


# 必要な準備

PostgreSQLのユーザーを作成しパスワードを設定する。

その後PostgreSQLでdatabaseを作成し、環境変数で

- ホスト名 (`SLGs_POSTGRESQL_HOST`)
- ユーザー名 (`SLGs_POSTGRESQL_USER`)
- パスワード (`SLGs_POSTGRESQL_PASSWORD`)
- databaseの名前 (`SLGs_POSTGRESQL_DB_NAME`)

を登録してください。


