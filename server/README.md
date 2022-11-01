# Puncher

Puncher is a UDP server that listens a single UDP port (default is 4200) and performs the following tasks:
1. Accepts connections from file servers and register their address with provided name
2. Performs translation from Name to server address

# Packet format

Packet consists of one byte type field and the payload.

Type | Meaning 
---|---
0x00 | Server announcement
0x01 | Client discovery

## Server message

Since we're getting its address and port, the message only consists of a server name

## Client message

Query: server name

Answer: [address u8: 4], [port u8: 2]
