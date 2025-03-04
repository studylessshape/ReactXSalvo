import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { Button, ConfigProvider, Input, theme } from "antd";
import FeedbackForm from "./Tutorial";
import * as hello from "./api/hello.ts";
import "virtual:uno.css";
import TravelPlan from "./TravelPlan.tsx";

function App() {
  const [helloText, setHelloText] = useState("");
  const [responseText, setResponseText] = useState("");

  function handleSubmit() {
    hello.hello(helloText).then((res) => res.text()).then((res) =>
      setResponseText(res)
    );
  }

  return (
    <>
      <ConfigProvider>
        <Input
          value={helloText}
          onChange={(e) => setHelloText(e.target.value)}
        />
        <p>{responseText}</p>
        <Button onClick={handleSubmit}>Send</Button>
        <FeedbackForm />
        <TravelPlan />
      </ConfigProvider>
    </>
  );
}

export default App;
