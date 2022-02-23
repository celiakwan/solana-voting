import * as anchor from '@project-serum/anchor';

export const requestAirdrop = async (
    provider: anchor.Provider,
    publicKey: anchor.web3.PublicKey,
    amount: number
) => {
    const signature = await provider.connection.requestAirdrop(
        publicKey,
        anchor.web3.LAMPORTS_PER_SOL * amount
    );
    await provider.connection.confirmTransaction(signature);
}
