import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { Button, ConfigProvider, theme, Input } from "antd";

function App() {
  const [count, setCount] = useState(0);
  const [helloText, setHelloText] = useState("");
  const [responseText, setResponseText] = useState("");

  function handleSubmit(message: String | null) {
    var request;
    if (message == null || message.trim().length == 0) {
      request = fetch("/api/hello");
    } else {
      request = fetch(`/api/hello?name=${message}`);
    }

    return request.then((res) => res.text()).then((res) =>
      setResponseText(res)
    );
  }

  return (
    <>
      <ConfigProvider theme={{algorithm: theme.darkAlgorithm}}>
        <div>
          <a href="https://vite.dev" target="_blank">
            <img src={viteLogo} className="logo" alt="Vite logo" />
          </a>
          <a href="https://react.dev" target="_blank">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
        </div>
        <h1>Vite + React</h1>
        <div className="card">
          <Button onClick={() => setCount((count) => count + 1)}>
            count is {count}
          </Button>
          <p>
            Edit <code>src/App.tsx</code> and save to test HMR
          </p>
        </div>
        <p className="read-the-docs">
          Click on the Vite and React logos to learn more
        </p>
        <Input
          type="text"
          value={helloText}
          onChange={(e) => setHelloText(e.target.value)}
        />
        <div>{responseText}</div>
        <Button onClick={() => handleSubmit(helloText)}>submit</Button>
      </ConfigProvider>
    </>
  );
}

export default App;
