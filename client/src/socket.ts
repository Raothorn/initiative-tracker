import type { GameState } from "./models";
import useUser from "./stores/currentUser";

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

            let user = useUser()
            this.setGuid(user.playerGuid)
        };

        // TODO other types of messages besides updates
        this.socket.onmessage = (event) => {
            let parsedMessage = JSON.parse(event.data)
            let updatedState: GameState = parsedMessage.msgData
            this.onStateUpdate(updatedState)
        };
    }

    disconnect() {
        this.socket.close();
    }

    $onStateUpdate(fn: (gs: GameState) => void) {
        this.onStateUpdate = fn;
    }

    sendAction(actionType: string, actionData: object) {
        let actionMsg = { actionType: actionType, actionData: actionData }
        let message = { msgType: "action", msgData: actionMsg } 

        this.socket.send(JSON.stringify(message))
    }

    setGuid(guid: string) {
        let loginMsg = { msgType: "setGuid", msgData: guid}
        this.socket.send(JSON.stringify(loginMsg))
    }
}

export const $socket = new SocketManager()
export default $socket
