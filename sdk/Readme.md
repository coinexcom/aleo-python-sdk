# Aleo Python SDK

精简版 SDK，用于构建 Aleo `transfer_public` 交易。

## 安装

需要 Rust **1.88+**。

```bash
bash install.sh
```

snarkvm 版本：`v4.7.4`（ProvableHQ/snarkVM git tag）。

## 测试

```bash
maturin develop --release --features openssl/vendored
python python/test.py
```

测试在本地生成证明，不会广播到链上。
