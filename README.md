# Security Group Tools

This sub-project contains a set of tools to work with AWS Security Groups.

~~~sh
Usage: sg_tools [OPTIONS] --operation <operation>

Options:
  -o, --operation <operation>  Operation to perform 
                               [possible values: list, describe, find-default-ingress, 
                                                 find-default-egress, find-all-ports, 
                                                 ec2-no-sec-token]
  -p, --profile <profile>      AWS Profile to use [default: default]
  -r, --region <region>        AWS Region to use [default: eu-central-1]
  -h, --help                   Print help
  -V, --version                Print version
~~~

## Operations

### List

List all security groups in the selected region.

### Describe

Describe all security groups in the selected region.

### Find Default Ingress

Find all security groups with default ingress rules (i.e. 0.0.0.0/0)

### Find Default Egress

Find all security groups with default egress rules (i.e.  0.0.0.0/0)

### Find All Ports

Find all security groups with rules that allow all ports.

### Describe EC2

Describe all EC2 instances in the selected region that do not have the HTTP_TOKEN_ENDPOINT as required.
