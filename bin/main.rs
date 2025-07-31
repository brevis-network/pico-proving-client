use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;
use pico_proving_client::{ProveTaskRequest, proving_network_client::ProvingNetworkClient};
use std::{fs, path::PathBuf, sync::Arc};
use tonic::codec::CompressionEncoding;

#[derive(Clone, Debug, ValueEnum)]
enum Command {
    ProveTask,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "prove-task", help = "Command")]
    pub cmd: Command,

    #[clap(long, default_values = &["fixtures/input_1.bin", "fixtures/input_2.bin"])]
    input_files: Vec<PathBuf>,

    #[clap(
        long,
        env = "GRPC_ADDR",
        default_value = "http://[::1]:50052",
        help = "gRPC address"
    )]
    grpc_addr: String,

    #[clap(
        long,
        env = "MAX_GRPC_MSG_SIZE",
        default_value = "629145600",
        help = "Max gRPC message size (bytes)"
    )]
    max_grpc_msg_size: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let cfg = Arc::new(Args::parse());

    let mut client = ProvingNetworkClient::connect(cfg.grpc_addr.clone())
        .await?
        .max_encoding_message_size(cfg.max_grpc_msg_size)
        .max_decoding_message_size(cfg.max_grpc_msg_size)
        .accept_compressed(CompressionEncoding::Zstd)
        .send_compressed(CompressionEncoding::Zstd);

    match cfg.cmd {
        Command::ProveTask => {
            let inputs = cfg
                .input_files
                .iter()
                .map(|path| fs::read_to_string(path).unwrap())
                .collect();
            let req = ProveTaskRequest { inputs };
            let res = client.prove_task(req).await?;
            let task_id = &res.get_ref().task_id;
            println!("processing task-{task_id}...");
            let proof_url =
                format!("https://pico-proofs.s3.us-west-2.amazonaws.com/task-{task_id}/proof.bin");
            println!("after proving, you could find the generated proof by URL:\n{proof_url}",);
        }
    };

    Ok(())
}
