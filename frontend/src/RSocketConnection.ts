import { RSocket } from "rsocket-core";

export class RSocketConnection {
    private readonly rsocket: RSocket;

    constructor(rsocket: RSocket) {
        this.rsocket = rsocket;
    }
}