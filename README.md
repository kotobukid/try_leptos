# Leptos カウンターアプリ(お勉強用)

このプロジェクトは、[Leptos](https://github.com/leptos-rs/leptos) フレームワークを使用したシンプルなRust Webアプリケーションです。このアプリケーションでは、複数のカウンターコンポーネントを動的に追加・操作できる機能を実装しています。

## 特徴

- 追加ボタンによる複数カウンターの動的生成
- シンプルなカウンターコンポーネント（加算、減算、リセット機能）
- カスタムボタンコンポーネント
- Leptosのリアクティブな状態管理を活用

## 技術スタック

- **言語**: Rust
- **フレームワーク**: Leptos
- **バックエンド**: Axum
- **フロントエンドビルドツール**: Trunk
- **ビルド管理**: cargo-make

## セットアップと実行方法

### 前提条件

- Rustがインストールされていること
- cargo-makeがインストールされていること (`cargo install cargo-make`)
- Trunkがインストールされていること (`cargo install trunk`)

## ビルドと実行手順

このプロジェクトでは、`cargo-make`を使用してビルドと実行を管理しています。以下のタスクが利用可能です：

### 開発モード

#### バックエンド開発サーバーの起動

```bash
cargo make run-back
```

バックエンドの開発サーバーが起動し、`http://localhost:8080`でアクセス可能になります。

#### フロントエンド開発サーバーの起動

```bash
cargo make run-front
```

フロントエンドの開発サーバーが起動し、ホットリロードが有効になります。

### ビルド

#### フロントエンドのビルド

```bash
cargo make build-front
```

フロントエンドをリリースモードでビルドします。ビルド成果物は`front/dist`ディレクトリに生成されます。

#### バックエンドのビルド

```bash
cargo make build-back
```

フロントエンドがビルド済みであることを前提に、バックエンドをリリースモードでビルドします。

#### 完全リリースビルド

```bash
cargo make full-release
```

フロントエンドとバックエンドの両方を一度にリリースモードでビルドします。このコマンドは`build-front`と`build-back`タスクを順番に実行します。

## アプリケーションへのアクセス

ビルド後、バックエンドサーバーを起動すると、ブラウザで`http://localhost:8080`にアクセスしてアプリケーションを利用できます。
