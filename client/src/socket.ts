
class SocketManager {
    private socket: WebSocket;

    constructor() {
        this.socket = new WebSocket("ws://localhost:9001/")
    }

    connect() {
        this.socket.onopen = (_) => {
            console.log("connected")
        };

        this.socket.onmessage = (event) => {
            console.log(`Message recieved: ${event.data}`)
        };
    }
}

export const $socket = new SocketManager()
export default $socket
