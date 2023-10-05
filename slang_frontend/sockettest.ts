//sockettest.ts
//Sockettest.ts is a typescript file intended on testing messenger functionality with the websocket built in Rust. The rust websocket
//starts the listener on port 9000 using the slang-ws websocket protocol. This file is to test the functionality.

import { Socket } from "node:net";

// Create the listener
