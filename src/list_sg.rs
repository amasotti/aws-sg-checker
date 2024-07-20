use aws_sdk_ec2::Client as Ec2Client;

/// List the security groups in the AWS account by name
///
/// # Arguments
///
/// * `ec2_client` - The EC2 client to use
pub async fn list_security_groups(ec2_client: &Ec2Client) {
    let response = ec2_client.describe_security_groups().send().await.unwrap();
    let security_groups = response.security_groups.unwrap();
    for sg in security_groups {
        println!("Security Group: {:?}", sg.group_name);
    }
}
