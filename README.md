# Aleo Python SDK

用于 Aleo 链上 **公开转账（transfer_public）** 的 Python SDK。

底层为 Rust + snarkVM 绑定，负责在本地完成授权、执行、零知识证明生成与交易组装。

## 安装

需要 **Rust 1.88+**（snarkvm 4.7.4 要求）。

```bash
cd sdk
bash install.sh
```

依赖版本：`snarkvm v4.7.4`（git tag `v4.7.4`）。

## 使用示例

```python
import json
from decimal import Decimal

import aleo

private_key = aleo.PrivateKey.from_string("APrivateKey1zkp...")
destination = aleo.Address.from_string("aleo1...")
amount = aleo.Credits(Decimal("1.5"))

query = aleo.Query.rest("https://api.explorer.provable.com/v1")
process = aleo.Process.load()
credits = aleo.Program.credits()
process.add_program(credits)

transfer_auth = process.authorize(
    private_key,
    credits.id(),
    aleo.Identifier.from_string("transfer_public"),
    [
        aleo.Value.from_literal(aleo.Literal.from_address(destination)),
        aleo.Value.from_literal(
            aleo.Literal.from_u64(aleo.U64(int(amount.micro())))
        ),
    ],
)
(_transfer_resp, transfer_trace) = process.execute(transfer_auth)
transfer_trace.prepare(query)
transfer_execution = transfer_trace.prove_execution(
    aleo.Locator(credits.id(), aleo.Identifier.from_string("transfer"))
)
execution_id = transfer_execution.execution_id()
process.verify_execution(transfer_execution)

fee = aleo.Credits(Decimal("0.01"))
fee_auth = process.authorize_fee_public(
    private_key, fee.micro(), execution_id, None
)
(_fee_resp, fee_trace) = process.execute(fee_auth)
fee_trace.prepare(query)
fee_proof = fee_trace.prove_fee()
process.verify_fee(fee_proof, execution_id)

transaction = aleo.Transaction.from_execution(transfer_execution, fee_proof)
params = json.loads(transaction.to_json())
```

## 公开 API

`PrivateKey`, `Address`, `Credits`, `Query`, `Process`, `Program`, `Identifier`, `Value`, `Literal`, `U64`, `Locator`, `Transaction` 等，见 `sdk/python/aleo/__init__.py`。

## 目录

```
sdk/
  src/           # Rust 绑定
  python/aleo/   # Python 包
```
