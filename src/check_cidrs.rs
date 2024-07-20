use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_ec2::operation::describe_security_groups::{DescribeSecurityGroupsOutput};
use aws_sdk_ec2::types::Filter;

const DEFAULT_ROUTE_VALUE: &str = "0.0.0.0/0";

pub async fn find_sg_default_ingress(ec2client: Ec2Client) {


    let filter_ingress = Filter::builder()
        .name("ip-permission.cidr")
        .values(DEFAULT_ROUTE_VALUE)
        .build();

    let sg_descr = ec2client
        .describe_security_groups()
        .set_filters(Some(vec![filter_ingress]))
        .send()
        .await
        .unwrap();

    print_sg_description(sg_descr)
}

pub async fn find_sg_default_egress(ec2client: Ec2Client) {
    let filter_egress = Filter::builder()
        .name("egress.ip-permission.cidr")
        .values(DEFAULT_ROUTE_VALUE)
        .build();

    let sg_descr = ec2client
        .describe_security_groups()
        .set_filters(Some(vec![filter_egress]))
        .send()
        .await
        .unwrap();

    print_sg_description(sg_descr)
}

pub async fn find_sg_all_ports(ec2client: Ec2Client) {
    let filter_all_ports = Filter::builder()
        .name("ip-permission.to-port")
        .set_values(Some(vec!["-1".to_string(), "0".to_string()]))
        .build();

    let sg_descr = ec2client
        .describe_security_groups()
        .set_filters(Some(vec![filter_all_ports]))
        .send()
        .await
        .unwrap();

    print_sg_description(sg_descr)
}


pub async fn describe_all_sg(ec2_client: Ec2Client) {
    let sg_descr = ec2_client
        .describe_security_groups()
        .send()
        .await
        .unwrap();

    print_sg_description(sg_descr)
}

fn print_sg_description(sg_desc: DescribeSecurityGroupsOutput) {
    sg_desc.security_groups.unwrap().iter().for_each(|sg| {
        if let Some(group_name) = &sg.group_name {
            println!("Security Group: {:?}", group_name);
            println!("\tDescription: {:?}", sg.description.as_ref().unwrap());
            println!("\tID: {:?}", sg.group_id.as_ref().unwrap());
            println!("\tVPC ID: {:?}", sg.vpc_id.as_ref().unwrap());
        }

        // Inbound rules
        println!("Inbound rules:");
        if let Some(ip_permissions) = &sg.ip_permissions {
            ip_permissions.iter().for_each(|ip_perm| {
                println!("\tFrom port: {:?}", ip_perm.from_port.unwrap_or(-1));
                println!("\tTo port: {:?}", ip_perm.to_port.unwrap_or(-1));
                println!("\tProtocol: {:?}", ip_perm.ip_protocol.as_ref().unwrap_or(&"all".to_string()));

                if let Some(ip_ranges) = &ip_perm.ip_ranges {
                    ip_ranges.iter().for_each(|ip_range| {
                        println!("\tIP Range: {:?}", ip_range);
                        println!("\t\tCIDR: {:?}", ip_range.cidr_ip);
                    });
                }
            });
        }


        // Outbound rules
        println!("Outbound rules:");
        if let Some(ip_permissions_egress) = &sg.ip_permissions_egress {
            ip_permissions_egress.iter().for_each(|ip_perm_egress| {
                println!("\tFrom port: {:?}", ip_perm_egress.from_port.unwrap_or(-1));
                println!("\tTo port: {:?}", ip_perm_egress.to_port.unwrap_or(-1));
                println!("\tProtocol: {:?}", ip_perm_egress.ip_protocol.as_ref().unwrap_or(&"all".to_string()));

                if let Some(ip_ranges) = &ip_perm_egress.ip_ranges {
                    ip_ranges.iter().for_each(|ip_range| {
                        println!("\tIP Range: {:?}", ip_range);
                        println!("\t\tCIDR: {:?}", ip_range.cidr_ip);
                    });
                }
            });
        }

        println!("----------------------\n");
    })
}
