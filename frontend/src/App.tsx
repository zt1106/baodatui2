import { RSocket } from 'rsocket-core';
import './App.css';

type AppProps = {
  rsocket: RSocket
}

function App(props: AppProps) {
  let rsocket = props.rsocket;
  const requester = rsocket.requestStream(
    {
      data: Buffer.from("Hello World"),
    },
    3,
    {
      onError: (e) => {

      },
      onNext: (payload, isComplete) => {

        if (isComplete) {

        }
      },
      onComplete: () => {

      },
      onExtension: () => { },
    }
  )
  return <h1>Hello World</h1>;
}

export default App;
