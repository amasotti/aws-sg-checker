use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client as Ec2Client;

/// Connect to AWS using the provided profile and region
async fn connect_to_aws(profile: String, region: String) -> SdkConfig {

    let region = Region::new(region.to_owned());
    let provider_region = RegionProviderChain::first_try(region)
        .or_else("eu-central-1");

    let creds = DefaultCredentialsChain::builder()
        .profile_name(profile.as_str())
        .region(provider_region)
        .build()
        .await;

    aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .load()
        .await
}

pub async fn get_ec2_client(args: &clap::ArgMatches) -> Ec2Client {

    let profile =  args.get_one::<String>("profile").expect("Profile is required").to_owned();
    let region = args.get_one::<String>("region").expect("Region is required").to_owned();

    let aws_config = connect_to_aws(profile, region).await;
    Ec2Client::new(&aws_config)
}
