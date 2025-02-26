import { useState } from "react";
import "./App.css";
import { Button, ConfigProvider, Input, theme } from "antd";

function App() {
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
      <ConfigProvider theme={{ algorithm: theme.darkAlgorithm }}>
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
