import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { Button, ConfigProvider, Input, theme } from "antd";
import FeedbackForm from "./Tutorial";
import * as hello from "./api/hello.js";
import "virtual:uno.css";

function App() {
  const [helloText, setHelloText] = useState("");
  const [responseText, setResponseText] = useState("");

  function handleSubmit(message: String | null) {
    hello.hello(message).then((res) => res.text()).then((res) =>
      setResponseText(res)
    );
  }

  return (
    <>
      <ConfigProvider>
        <Input value={helloText} onChange={(e) => setHelloText(e.target.value) }/>
        <p>{responseText}</p>
        <Button onClick={handleSubmit}>Send</Button>
        <FeedbackForm />
      </ConfigProvider>
    </>
  );
}

export default App;
