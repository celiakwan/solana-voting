import assert from 'assert';
import * as anchor from '@project-serum/anchor';
import { SolanaVoting } from '../target/types/solana_voting';
import { VoteCount } from '../target/types/vote_count';
import { VoteRecord } from '../target/types/vote_record';
import * as h from './helper';
import type { PublicKey } from '@solana/web3.js';

describe('solana-voting', () => {
  const mainProgram = anchor.workspace.SolanaVoting as anchor.Program<SolanaVoting>;
  const countProgram = anchor.workspace.VoteCount as anchor.Program<VoteCount>;
  const recordProgram = anchor.workspace.VoteRecord as anchor.Program<VoteRecord>;
  const authorityAccount = anchor.web3.Keypair.generate();
  const description = 'Adjust rewards rate';
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  let proposalAccount: PublicKey;
  let proposalAccountBump: number;
  let countAccount: PublicKey;
  let countAccountBump: number;
  let recordAccount: PublicKey;
  let recordAccountBump: number;

  before(async () => {
    await h.requestAirdrop(provider, authorityAccount.publicKey, 10);

    [proposalAccount, proposalAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('proposal')],
      mainProgram.programId
    );
    [countAccount, countAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('count'), proposalAccount.toBuffer()],
      countProgram.programId
    );
    [recordAccount, recordAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('record'), provider.wallet.publicKey.toBuffer()],
      recordProgram.programId
    );
  });

  it('Initialize count account', async () => {
    await countProgram.rpc.initialize(countAccountBump, proposalAccount, {
      accounts: {
        countAccount,
        authority: authorityAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [authorityAccount]
    });

    const countData = await countProgram.account.count.fetch(countAccount);
    assert.equal(countData.authority, authorityAccount.publicKey.toString(), 'Incorrect authority');
    assert.equal(countData.proposal, proposalAccount.toString(), 'Incorrect proposal');
    assert.equal(countData.agree, 0, 'Incorrect number of agree');
    assert.equal(countData.disagree, 0, 'Incorrect number of disagree');
  });

  it('Initialize record account', async () => {
    await recordProgram.rpc.initialize(recordAccountBump, {
      accounts: {
        recordAccount,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    });

    const recordData = await recordProgram.account.record.fetch(recordAccount);
    assert.equal(recordData.user, provider.wallet.publicKey.toString(), 'Incorrect user');
    assert.equal(recordData.votedProposals.length, 0, 'Incorrect number of voted proposals');
    assert.equal(recordData.rewards, 0, 'Incorrect rewards amount');
  });

  it('Create a proposal', async () => {
    await mainProgram.rpc.createProposal(proposalAccountBump, description, {
      accounts: {
        proposalAccount,
        authority: authorityAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [authorityAccount]
    });

    const proposalData = await mainProgram.account.proposal.fetch(proposalAccount);
    assert.equal(proposalData.id, 1, 'Incorrect id');
    assert.equal(proposalData.description, description, 'Incorrect description');
  });

  it('Vote', async () => {
    await mainProgram.rpc.vote(true, {
      accounts: {
        proposalAccount,
        countProgram: countProgram.programId,
        countAccount,
        authority: authorityAccount.publicKey,
        recordProgram: recordProgram.programId,
        recordAccount,
        user: provider.wallet.publicKey
      },
      signers: [authorityAccount]
    });

    const proposalData = await mainProgram.account.proposal.fetch(proposalAccount);
    assert.equal(proposalData.id, 1, 'Incorrect id');
    assert.equal(proposalData.description, description, 'Incorrect description');

    const countData = await countProgram.account.count.fetch(countAccount);
    assert.equal(countData.authority, authorityAccount.publicKey.toString(), 'Incorrect authority');
    assert.equal(countData.proposal, proposalAccount.toString(), 'Incorrect proposal');
    assert.equal(countData.agree, 1, 'Incorrect number of agree');
    assert.equal(countData.disagree, 0, 'Incorrect number of disagree');

    const recordData = await recordProgram.account.record.fetch(recordAccount);
    assert.equal(recordData.user, provider.wallet.publicKey.toString(), 'Incorrect user');
    assert.equal(recordData.votedProposals.length, 1, 'Incorrect number of voted proposals');
    assert.equal(recordData.rewards, 10, 'Incorrect rewards amount');
  });

  it('Vote again', async () => {
    let error: Error;
    try {
      await mainProgram.rpc.vote(true, {
        accounts: {
          proposalAccount,
          countProgram: countProgram.programId,
          countAccount,
          authority: authorityAccount.publicKey,
          recordProgram: recordProgram.programId,
          recordAccount,
          user: provider.wallet.publicKey
        },
        signers: [authorityAccount]
      });
    } catch (e) {
      error = e;
    } finally {
      assert.match(error.message, /Proposal has already been voted/);
    }

    const countData = await countProgram.account.count.fetch(countAccount);
    assert.equal(countData.agree, 1, 'Incorrect number of agree');
    assert.equal(countData.disagree, 0, 'Incorrect number of disagree');

    const recordData = await recordProgram.account.record.fetch(recordAccount);
    assert.equal(recordData.votedProposals.length, 1, 'Incorrect number of voted proposals');
    assert.equal(recordData.rewards, 10, 'Incorrect rewards amount');
  });
});
