Examples
=================

.. testsetup:: *

   import aleo

Calling a **transfer_public** function
**************************************

.. doctest::
    :options: +ELLIPSIS

    >>> private_key = aleo.PrivateKey.from_string(
    ...     "APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6")
    >>> destination = aleo.Address.from_string(
    ...     "aleo16u4ecz4yqq0udtnmsy8qzvj8emnua24n27c264f2t3unekdlpy8sh4hat2")
    >>> amount = aleo.Credits(0.3)
    >>> query = aleo.Query.rest("https://explorer.hamp.app")
    >>> process = aleo.Process.load()
    >>> credits = aleo.Program.credits()
    >>> process.add_program(credits)
    >>> transfer_name = aleo.Identifier.from_string("transfer_public")
    >>> transfer_auth = process.authorize(private_key, credits.id(), transfer_name, [
    ...     aleo.Value.from_literal(aleo.Literal.from_address(destination)),
    ...     aleo.Value.from_literal(aleo.Literal.from_u64(
    ...         aleo.U64(int(amount.micro())))),
    ... ])
    >>> (_transfer_resp, transfer_trace) = process.execute(transfer_auth)
    >>> transfer_trace.prepare(query)
    >>> transfer_execution = transfer_trace.prove_execution(
    ...     aleo.Locator(credits.id(), aleo.Identifier.from_string("transfer")))
    >>> execution_id = transfer_execution.execution_id()
    >>> process.verify_execution(transfer_execution)

    >>> fee = aleo.Credits(0.01)
    >>> fee_auth = process.authorize_fee_public(
    ...     private_key, fee.micro(), execution_id, None)
    >>> (_fee_resp, fee_trace) = process.execute(fee_auth)
    >>> fee_trace.prepare(query)
    >>> fee = fee_trace.prove_fee()
    >>> process.verify_fee(fee, execution_id)

    >>> transaction = aleo.Transaction.from_execution(transfer_execution, fee)
    >>> transaction.to_json()
    '{"type":"execute","id":"at...
