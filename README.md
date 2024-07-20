# Security Group Tools

This sub-project contains a set of tools to work with AWS Security Groups.

~~~sh
Usage: sg_tools [OPTIONS] --operation <operation>

Options:
  -o, --operation <operation>  Operation to perform [possible values: list, describe, find-vulnerable]
  -p, --profile <profile>      AWS Profile to use
  -r, --region <region>        AWS Region to use [default: eu-central-1]
  -h, --help                   Print help
  -V, --version                Print version
~~~

## Operations

### List

List all security groups in the selected region.

### Describe

Describe all security groups in the selected region.

### Find Vulnerable

Find security groups that are "too" open:

- Security groups with inbound rules that allow traffic from any IP address.
- Security groups with inbound rules that allow traffic to any IP address.
- Security groups with inbound rules that allow traffic from any port.
