use clap::Parser;

#[derive(Parser)]
#[command(name = "sakasu-relayer", version)]
struct Args {
    /// HTTP listen address.
    #[arg(long, default_value = "0.0.0.0:7780")]
    listen: String,
    /// Solana RPC endpoint.
    #[arg(long, default_value = "https://api.mainnet-beta.solana.com")]
    solana_rpc: String,
}

fn main() {
    let args = Args::parse();
    println!(
        "starting sakasu-relayer  listen={}  rpc={}",
        args.listen, args.solana_rpc
    );
    sakasu_relayer::network::serve(&args.listen);
}
