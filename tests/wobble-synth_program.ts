import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"

import { expect } from "chai"
import { WobbleSynth } from "../target/types/wobble_synth"

describe("wobble-synth_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider)
  const user0Account = anchor.web3.Keypair.generate()
  const user0Program = anchor.workspace.WobbleSynth as Program<WobbleSynth>

  it("buy_first_song", async () => {
    await user0Program.rpc.buyFirstSong({
      accounts: {
        user0: user0Account.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [user0Account],
    })
    const fetchedUser0 = await user0Program.account.user0.fetch(
      user0Account.publicKey
    )
    expect(fetchedUser0.songsCount).to.eq(1)
  })

  it("buy_song", async () => {
    const allUsers = await user0Program.account.user0.all()
    const currUser = allUsers[0]
    await user0Program.rpc.buySong({
      accounts: {
        user0: currUser.publicKey,
        wallet: provider.wallet.publicKey,
      },
    })

    const fetchedUsers = await user0Program.account.user0.fetch(
      currUser.publicKey
    )
    expect(fetchedUsers.songsCount).to.eq(2)
  })

  it("cannot buy song for other user", async () => {
    const otherUser = anchor.web3.Keypair.generate()
    try {
      await user0Program.rpc.buySong({
        accounts: {
          user0: user0Account.publicKey,
          wallet: otherUser.publicKey,
        },
      })
    } catch (err) {
      const currUser = await user0Program.account.user0.fetch(
        user0Account.publicKey
      )
      expect(currUser.songsCount).to.eq(2)
      expect(err.message).to.eq("Signature verification failed")
    }
  })
})
