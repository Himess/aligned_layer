#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AlignedArgs {
    #[command(subcommand)]
    pub command: AlignedCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum AlignedCommands {
    #[command(about = "Submit proof to the batcher", name = "submit")]
    Submit(SubmitArgs),
    #[command(about = "Verify the proof was included in a verified batch on Ethereum", name = "verify-proof-onchain")]
    VerifyProofOnchain(VerifyProofOnchainArgs),
    #[command(about = "Get commitment for file", name = "get-vk-commitment")]
    GetVkCommitment(GetVkCommitmentArgs),
    #[command(about = "Deposits Ethereum in the batcher to pay for proofs", name = "deposit-to-batcher")]
    DepositToBatcher(DepositToBatcherArgs),
    #[command(about = "Get user balance from the batcher", name = "get-user-balance")]
    GetUserBalance(GetUserBalanceArgs),
    #[command(about = "Get user nonce from the batcher", name = "get-user-nonce")]
    GetUserNonce(GetUserNonceArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SubmitArgs {
    #[arg(name = "Batcher connection address", long = "batcher_url", default_value = "ws://localhost:8080")]
    batcher_url: String,
    #[arg(name = "Ethereum RPC provider connection address", long = "rpc_url", default_value = "http://localhost:8545")]
    eth_rpc_url: String,
    #[arg(name = "Proving system", long = "proving_system")]
    proving_system_flag: ProvingSystemArg,
    #[arg(name = "Proof file path", long = "proof")]
    proof_file_name: PathBuf,
    #[arg(name = "Public input file name", long = "public_input")]
    pub_input_file_name: Option<PathBuf>,
    #[arg(name = "Verification key file name", long = "vk")]
    verification_key_file_name: Option<PathBuf>,
    #[arg(name = "VM program code file name", long = "vm_program")]
    vm_program_code_file_name: Option<PathBuf>,
    #[arg(name = "Number of repetitions", long = "repetitions", default_value = "1")]
    repetitions: usize,
    #[arg(name = "Proof generator address", long = "proof_generator_addr", default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")]
    proof_generator_addr: String,
    #[arg(name = "Aligned verification data directory Path", long = "aligned_verification_data_path", default_value = "./aligned_verification_data/")]
    batch_inclusion_data_directory_path: String,
    #[arg(name = "Path to local keystore", long = "keystore_path")]
    keystore_path: Option<PathBuf>,
    #[arg(name = "Private key", long = "private_key")]
    private_key: Option<String>,
    #[arg(name = "Max Fee (ether)", long = "max_fee", default_value = "0.0013ether")]
    max_fee: String,
    #[arg(name = "Nonce", long = "nonce")]
    nonce: Option<String>,
    #[arg(name = "The working network's name", long = "network", default_value = "devnet")]
    network: NetworkArg,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
#[value_enum(rename_all = "PascalCase")]
enum NetworkArg {
    Devnet,
    Holesky,
    HoleskyStage,
    Mainnet,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProvingSystemArg {
    #[value_enum(rename_all = "PascalCase")]
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
    Risc0,
}
