import "./App.css";

import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [event, setEvent] = useState("");

  useEffect(() => {
    const unlisten = listen<{ message: string }>("keyEvent", (event) => {
      console.log("Received event:", event.payload.message);
      setEvent(event.payload.message);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <p>{event}</p>
    </div>
  );
}

export default App;
