import * as anchor from "@coral-xyz/anchor"
import { BN } from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Pakt } from "../target/types/pakt"

describe("vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Pakt as Program<Pakt>

  const provider = anchor.getProvider()

  const connection = provider.connection

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
      signature,
      ...block,
    })
    return signature
  }

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    )
    return signature
  }

  it("Deposit", async () => {
    const tx = await program.methods
      .deposit(new BN(1_000_000))
      .accounts({
        signer: provider.publicKey!,
      })
      .rpc()
      .then(confirm)
      .then(log)
  })

  it("Withdraw", async () => {
    const tx = await program.methods
      .withdraw(new BN(1_000_000))
      .accounts({
        signer: provider.publicKey!,
      })
      .rpc()
      .then(confirm)
      .then(log)
  })
})
