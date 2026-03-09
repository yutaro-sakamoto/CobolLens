# CobolLens

COBOL の構文解析器を Rust + rowan で構築するプロジェクト。ロスレスな具象構文木 (CST) を生成する。

## ビルド・テスト

```bash
cargo build    # ビルド
cargo test     # 全テスト実行
cargo run      # Hello World COBOL プログラムの解析デモ
```

## アーキテクチャ

rowan クレート (v0.16) を使った再帰下降パーサー。

```
src/
  syntax_kind.rs  -- SyntaxKind enum (#[repr(u16)]): トークン・キーワード・ノード種別
  language.rs     -- CobolLanguage (rowan::Language impl) + 型エイリアス (SyntaxNode 等)
  lexer.rs        -- 手書きレキサー: lex(input) -> Vec<(SyntaxKind, String)>
  parser.rs       -- 再帰下降パーサー: parse(input) -> SyntaxNode (GreenNodeBuilder 駆動)
  ast.rs          -- 型付き AST ラッパー (ast_node! マクロ)
  lib.rs          -- クレートルート + テスト
  main.rs         -- デモ実行
```

## コーディング規約

- SyntaxKind の variant 名は `SCREAMING_SNAKE_CASE` (rowan の慣例)
- キーワードには `_KW` サフィックス、ノードには型名そのまま
- レキサーは大文字小文字を区別しない (`to_ascii_uppercase` で判定)
- パーサーはロスレス: 入力の全バイトがツリーに保持される (`tree.text() == input`)
- トリビア (WHITESPACE, NEWLINE) はパーサーの `bump_trivia()` で自動挿入

## 参考資料

- `opensourcecobol4j/cobj/parser.y` — 文法定義の参考元
