import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { DvpnClient } from '../target/types/dvpn_program'
import { PublicKey } from '@solana/web3.js'
import * as assert from 'assert'

describe('dvpn-program', () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.DvpnClient as Program<DvpnClient>

  const planAccount = anchor.web3.Keypair.generate()

  it('Create plan!', async () => {
    const provider = anchor.getProvider()
    const payer = provider.wallet

    const BTC_FEED = new PublicKey(
      'HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J'
    )

    const title = 'Sample Plan'
    const expirationDate = Date.now() + 86400000 // 24 hours from now

    const tx = await program.methods
      .createPlan(title, new anchor.BN(expirationDate))
      .accounts({
        plan: planAccount.publicKey,
        user: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        solPriceFeed: BTC_FEED
      })
      .signers([planAccount])
      .rpc()

    console.log('Transaction Signature:', tx)

    // Fetch the account data
    const accountData = await program.account.plan.fetch(planAccount.publicKey)
    console.log('Plan Account Data:', accountData)

    // Add assertions
    assert.ok(
      accountData.owner.equals(payer.publicKey),
      'Owner should match payer'
    )
    assert.strictEqual(accountData.title, title, 'Title should match')
    assert.strictEqual(
      accountData.expirationDate.toNumber(),
      expirationDate,
      'Expiration date should match'
    )
  })
})
