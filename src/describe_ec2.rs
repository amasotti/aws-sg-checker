use aws_sdk_ec2::types::HttpTokensState;
use aws_sdk_ec2::Client as Ec2Client;

pub async fn ec2_instances_without_sec_token(ec2_client: Ec2Client) {
    let mut reservations = vec![];
    let mut next_token = None;

    loop {
        let mut request = ec2_client.describe_instances();
        if let Some(token) = next_token {
            request = request.set_next_token(Some(token));
        }

        let response = request.send().await.unwrap();
        reservations.extend(response.reservations.unwrap());

        if response.next_token.is_none() {
            break;
        }

        next_token = response.next_token;
    }

    for reservation in reservations.iter() {
        for instance in reservation.instances.as_ref().unwrap().iter() {
            if let Some(metadata) = &instance.metadata_options {
                let http_tokens = metadata.http_tokens.as_ref().unwrap();

                if http_tokens == &HttpTokensState::Required {
                    continue;
                }

                let name = instance
                    .tags
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|tag| tag.key.as_ref().unwrap() == "Name")
                    .unwrap();
                let state = metadata.state.as_ref().unwrap();
                println!(
                    "Instance ID: {} -- Name {:?}",
                    instance.instance_id.as_ref().unwrap(),
                    name
                );
                println!("\tState: {:?}", state);
                println!("\tHttp Tokens: {:?}", http_tokens);
            }
        }
    }
}
