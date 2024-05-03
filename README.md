# Sakura OS

## 目標

- x86_64, ARM, RISC-V に対応したカーネルを作成する
- 外部クレートを使わずに,自前で実装できるようにする
- マルチタスク,メモリ管理,ファイルシステム,ネットワークスタックなどの機能を実装する

## 実行方法

Rust の Nightly と,対象アーキテクチャ向けの QEMU がインストールされている必要があります。

### x86_64

https://retrage.github.io/edk2-nightly/ から `RELEASEX64_OVMF.fd` を `thirdparty` ディレクトリにコピー

```sh
make x86-run
```

### RISC-V

OpenSBI のインストールが必要な場合があります。

```sh
make riscv-run
```
