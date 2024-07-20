use sg_tools::{process_command, sg_tools_command};

mod ec2_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = sg_tools_command().get_matches();

    let ec2_client = ec2_client::get_ec2_client(&args).await;
    process_command(&args, ec2_client).await;
}
