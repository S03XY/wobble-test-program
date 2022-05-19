import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";

import { expect } from "chai";
import { WobbleSynth } from "../target/types/wobble_synth";

describe("wobble-synth_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const user0Account = anchor.web3.Keypair.generate();
  const user0Program = anchor.workspace.WobbleSynth as Program<WobbleSynth>;

  it("buy_first_song", async () => {
  });

  it("buy_song", async () => {
    
  });

  it("cannot buy song for other user", async () => {});
});
