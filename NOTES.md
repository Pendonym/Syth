# Type Sizes:
| Type | Size (bytes) | Type |
|---|---|---|
| Byte | 1 | u8 |
| Short | 2 | i16 |
| String | 64 | [u8; 64] |


# Heartbeat
TODO

# Client → server packets

## Player Identification
When the player tries connecting to the server it will immeditely send a Player Identification packet which looks like:

| Field | Type  | Example Data |
|---|---|---|
| Packet ID | Byte | `0x00` |
| Protocol version | Byte | 7 |
| Username | String | "Pendonym" |
| Verification key | String | Can be none |
| Unused | Byte | |

# Server → client packets

## Server Identification
This is the response sent to a player joining.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x00` |
| Protocol version | Byte | 7 |
| Server name | String | "Syth Server" |
| Server MOTD | String | "Welcome to my server!" |
| User type | Byte | OP (0x64) or Not (0x00) |

## Ping
This should be sent to the client every ~30s to let the client know the server is still open.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x01` |

## Level Initialize 
Lets the client know level data is going to be sent.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x02` |