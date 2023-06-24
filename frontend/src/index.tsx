import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import { WebsocketClientTransport } from "rsocket-websocket-client";
import { RSocketConnector } from 'rsocket-core';
const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
function makeConnector() {
  return new RSocketConnector({
    transport: new WebsocketClientTransport({
      url: "ws://localhost:8080",
      wsCreator: (url) => new WebSocket(url) as any,
    }),
  });
}
const connector = makeConnector();
connector.connect().then(rsocket => {
  root.render(<App rsocket={rsocket} />);
}).catch(err => {
  root.render(<h1>init error: {err}</h1>);
});




