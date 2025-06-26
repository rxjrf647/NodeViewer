# egui学習

## 概要

- 入れ子で反復的なデータ構造を持つ情報を表示するGUIアプリを設計する
- UIはWindowsの`Explorer`をイメージ
  - 右ペインに選択肢のツリーを表示する
  - 選択されたツリーに応じて中央ペインにデータをテーブル表示する

## 開発環境

- Rust = "1.87"
- eframe = "0.31.1"
- egui_extras = "0.31.1" (TableBuilderを利用)

## 扱うデータ構造
```rust

type NodesInfo = Vec<Nodes>;

struct Nodes {
    name: String    
    nodes = Vec<Node>;
}

struct Node {
    name: NodeType
    contents: Vec<Content>
}

enum NodeType {
    TypeA
    TypeB
    TypeC
}

struct Content: {
    index: String
    caption: String
    status: StatCode
}

enum StatCode {
    Ok
    Ng
    Warning
}
```

## UI仕様
### 左ペイン(ツリー部)

- データ`NodesInfo`のname属性のみを階層表示する
- ツリー深度は`Node.name`までとする
- 以下ツリー表示例

```
  + Nodes.name
    - Node.name
    - Node.name
    - Node.name
  - Nodes.name
  + Nodes.name
    - Node.name
    - Node.name
    - Node.name
  - Nodes.name
  - Nodes.name
  - Nodes.name
```

### 中央ペイン

- 右ペインのツリー選択により、表示内容を切り替える
- `Nodes.name`を選択した場合
  - `Nodes`が保持している`Node`の`.name`属性の一覧を表示する
- `Node.name`を選択した場合
  - `Node`が保持している`.contents`情報の一覧を表示する

#### `Nodes.name`を選択した場合の表示イメージ
  - Statusはまだ実装していない

|NodeName| Status |
|---|---|
|Node.mane| |
|Node.mane| |

#### `Node.name`を選択した場合

|Content| caption | status |
|---|---|---|
|index| caption| status |
|index| caption| status |
|index| caption| status |
|index| caption| status |


