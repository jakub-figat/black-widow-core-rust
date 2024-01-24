# Black widow core rust

Rewritten to Rust from [python version](https://github.com/jakub-figat/black-widow-core).


## Setup

1. With hot reload (for server development)

`$ make start-dev`

2. Without hot reload

`$ make start`

## Usage

All communication is WebSocket based, as of now game requires providing
`X-User` header for authentication. 

Payload and response schemas can be found in `/bindings` dir.