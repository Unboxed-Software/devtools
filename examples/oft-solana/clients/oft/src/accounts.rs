use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::shared::DecodedAccount;
use crate::generated::oft::accounts::peer_config::PeerConfig;

pub async fn fetch_peer_config_async(
    rpc: &RpcClient,
    address: &Pubkey,
) -> Result<DecodedAccount<PeerConfig>, std::io::Error> {
    let accounts = fetch_all_peer_config_async(rpc, &[*address]).await?;
    Ok(accounts[0].clone())
}

pub async fn fetch_all_peer_config_async(
    rpc: &RpcClient,
    addresses: &[Pubkey],
) -> Result<Vec<DecodedAccount<PeerConfig>>, std::io::Error> {
    let accounts = rpc.get_multiple_accounts(addresses)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<DecodedAccount<PeerConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref()
            .ok_or(std::io::Error::new(std::io::ErrorKind::Other, format!("Account not found: {}", address)))?;
        let data = PeerConfig::from_bytes(&account.data)?;
        decoded_accounts.push(DecodedAccount { address, account: account.clone(), data });
    }
    Ok(decoded_accounts)
} 