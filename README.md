[![codecov](https://codecov.io/gh/lonepeon/ipcalc/branch/main/graph/badge.svg?token=HC7WSVDRO2)](https://codecov.io/gh/lonepeon/ipcalc)

# ipcalc

Small tool based on https://linux.die.net/man/1/ipcalc which describes the given IPv4.

## Usage

```
$ ipcalc describe 10.12.23.43/20
Address:   10.12.23.43          00001010.00001100.00010111.00101011
Netmask:   255.255.240.0 = 20   11111111.11111111.11110000.00000000
Wildcard:  0.0.15.255           00000000.00000000.00001111.11111111
=>
Network:   10.12.16.0/20        00001010.00001100.00010000.00000000
HostMin:   10.12.16.1           00001010.00001100.00010000.00000001
HostMax:   10.12.31.254         00001010.00001100.00011111.11111110
Broadcast: 10.12.31.255         00001010.00001100.00011111.11111111
Hosts/Net: 4094                 class A, Private Internet
```

### Documentation

Documentation is available using `ipcalc help <command>`

```
ipcalc provides a simple way to display information about an IP and its network

Usage: ipcalc <COMMAND>

Commands:
    describe    Display host and network related information about the IPv4 CIDR
    split       Subdivide the CIDR in smaller networks and display them
    compare     Compare two CIDRs and display the relationship between them
    aggregate   List all possible de/aggregation from a given CIDR to a specified MASK
    help        Print this message or the help of the given subcommand(s)

Options:
    -h, --help       Print help information
    -V, --version    Print version information
```

#### Describe a CIDR

```
Display host and network related information about the IPv4 CIDR

Usage: ipcalc describe [OPTIONS] <CIDR>

Arguments:
    <CIDR>    Any valid host or network IPv4 CIDR

Options:
        --no-binary    Hide the binary representation
    -h, --help         Print help information (use `-h` for a summary)
```

#### Compare two CIDRs

```
Compare two CIDRs and display the relationship between the first and second CIDR:
same network, different network, subset or superset

Usage: ipcalc compare <CIDR> <OTHER>

Arguments:
    <CIDR>     Any valid host or network IPv4 CIDR.
               If an host CIDR is given, its related network will be used.
    <OTHER>    Any valid host or network IPv4 CIDR.
               If an host CIDR is given, its related network will be used.

Options:
    -h, --help    Print help information (use `-h` for a summary)
```

#### Split a CIDR in smaller networks

```
Subdivide the CIDR in smaller networks and display them

If the CIDR is a network address: display all available sub-networks
If the CIDR is a host address: display the new network in which the IP belongs

Usage: ipcalc split [OPTIONS] <CIDR> <NEW_MASK>

Arguments:
    <CIDR>        Any valid host or network IPv4 CIDR
    <NEW_MASK>    New prefix length to apply to the CIDR

Options:
    -h, --help         Print help information (use `-h` for a summary)
        --no-binary    Hide the binary representation
```

#### Aggregate/Deaggregate a prefix

```
List all possible de/aggregation from a given CIDR to a specified MASK

If the MASK length is bigger, the IP won't change only the mask will
If the MASK length is smaller, the IP will take the network address of the mask


Usage: ipcalc aggregate <CIDR> <MASK>

Arguments:
  <CIDR>
          Any valid host or network IPv4 CIDR

  <MASK>
          Lower/Upper bound of the de/aggregation

Options:
  -h, --help
          Print help (see a summary with '-h')
```
