import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair, PublicKey } from '@solana/web3.js'
import { Voting } from '../target/types/voting'
import { start } from 'repl'
import { BankrunProvider, startAnchor } from 'anchor-bankrun'

const IDL = require("../target/idl/voting.json")
const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

describe('Voting', () => {

  let context;
  let provider;
  let votingProgram: Program<Voting>;

  beforeAll(async () => {
    context = await startAnchor("", [{ name: "voting", programId: votingAddress }], []);
    provider = new BankrunProvider(context);

    votingProgram = new Program<Voting>(
      IDL,
      provider
    );
  });


  it('Initialize Poll', async () => {
    await votingProgram.methods.initializePoll(
      new anchor.BN(1),
      "what is your favorite type of peanut butter?",
      new anchor.BN(0),
      new anchor.BN(1835565014),
    ).rpc();

    // check
    // 获取pda地址
    const [pollAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8)],
      votingAddress
    )

    const poll = await votingProgram.account.poll.fetch(pollAddress);
    console.log(poll);


    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.description).toEqual("what is your favorite type of peanut butter?");
  });

  it("initialize candidate", async () => {
    await votingProgram.methods.initializeCandidate(
      "Smooth",
      new anchor.BN(1)
    ).rpc();
    await votingProgram.methods.initializeCandidate(
      "Crunchy",
      new anchor.BN(1)
    ).rpc();

    // check
    // 获取pda地址
    const [smoothAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Crunchy")], votingAddress
    )
    const smoothCandidate = await votingProgram.account.candidate.fetch(smoothAddress);
    console.log(smoothCandidate);
    expect(smoothCandidate.candidateVote.toNumber()).toEqual(0);

    const [crunhyAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Crunchy")], votingAddress
    )
    const crunhyCandidate = await votingProgram.account.candidate.fetch(crunhyAddress);
    console.log(crunhyCandidate);
    expect(crunhyCandidate.candidateVote.toNumber()).toEqual(0);
  });

  it("vote", async () => {
    await votingProgram.methods
      .vote(new anchor.BN(1), "Crunchy")
      .rpc();

    const [crunhyAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Crunchy")], votingAddress
    )
    const crunhyCandidate = await votingProgram.account.candidate.fetch(crunhyAddress);
    expect(crunhyCandidate.candidateVote.toNumber()).toEqual(1);

  });

})
