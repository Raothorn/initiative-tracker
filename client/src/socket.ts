import type { GameState } from "./models";

class SocketManager {
    private onStateUpdate: (gs: GameState) => void;
    private socket: WebSocket;

    constructor() {
        this.onStateUpdate = (_) => {};
        this.socket = new WebSocket("ws://localhost:9001/")
    }

    connect() {
        this.socket.onopen = (_) => {
            console.log("connected")
        };

        this.socket.onmessage = (event) => {
            console.log(`Message recieved: ${event.data}`)
            let parsedMessage = JSON.parse(event.data)
            console.log(`Update parsed: `, parsedMessage)

            let updatedState: GameState = parsedMessage.msgData
            this.onStateUpdate(updatedState)
        };
    }

    $onStateUpdate(fn: (gs: GameState) => void) {
        this.onStateUpdate = fn;
    }
}

export const $socket = new SocketManager()
export default $socket
