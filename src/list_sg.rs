use aws_sdk_ec2::Client as Ec2Client;

pub async fn list_security_groups(client: &Ec2Client) {
    let response = client.describe_security_groups().send().await.unwrap();
    let security_groups = response.security_groups.unwrap();
    for sg in security_groups {
        println!("Security Group: {:?}", sg.group_name);
    }
}
