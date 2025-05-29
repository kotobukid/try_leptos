# Leptos カウンターアプリ(お勉強用)

このプロジェクトは、[Leptos](https://github.com/leptos-rs/leptos) フレームワークを使用したシンプルなRust Webアプリケーションです。このアプリケーションでは、複数のカウンターコンポーネントを動的に追加・操作できる機能を実装しています。

## 特徴

- 追加ボタンによる複数カウンターの動的生成
- シンプルなカウンターコンポーネント（加算、減算、リセット機能）
- カスタムボタンコンポーネント
- Leptosのリアクティブな状態管理を活用

## 技術スタック

- **言語**: Rust
- **フレームワーク**: Leptos 0.8.2
- **スタイリング**: leptos-style 0.2.0、leptos_meta 0.8.2
- **ビルドツール**: Trunk
- **その他の依存関係**:
  - log 0.4.27
  - console_log 1.0.0
  - gloo-timers 0.3.0
  - console_error_panic_hook 0.1.7

## セットアップと実行方法

### 前提条件

- Rustがインストールされていること
- Trunkがインストールされていること

## Axumサーバーとの統合実行手順

このプロジェクトでは、LeptosフロントエンドアプリケーションをAxumバックエンドサーバーから提供する統合環境を実装しています。以下の手順で実行できます。

### 1. Leptosアプリケーションのビルド

まず、Leptosフロントエンドアプリケーションをビルドします：

```bash
cd front
trunk build --release
cd ..
```

これにより、`front/dist` ディレクトリにビルド成果物が生成されます。

### 2. Axumサーバーの起動

次に、Axumサーバーを起動します：

```bash
cargo run -p webserver
```

サーバーは `http://localhost:8080` で起動します。

### 3. アプリケーションへのアクセス

ブラウザで `http://localhost:8080` にアクセスすると、Leptosアプリケーションが表示されます。

APIエンドポイントは `/api/hello` で利用可能です。

### 開発モードでの実行

開発中は、Leptosアプリケーションの変更を自動的に反映させるために、以下のコマンドを使用できます：

```bash
cd front
trunk watch
```

その後、別のターミナルでAxumサーバーを起動します：

```bash
cargo run -p webserver
```

変更するたびに `front/dist` ディレクトリが更新され、Axumサーバーが最新のビルドを提供します。
