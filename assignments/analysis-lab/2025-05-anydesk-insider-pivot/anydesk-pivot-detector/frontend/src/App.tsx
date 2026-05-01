import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
      <h1 className="text-4xl font-bold mb-8">AnyDesk Pivot Detector</h1>

      <div className="flex gap-4 mb-8">
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
          className="px-4 py-2 rounded bg-slate-800 border border-slate-700 focus:outline-none focus:border-blue-500"
        />
        <button 
          type="button" 
          onClick={() => greet()}
          className="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded transition-colors"
        >
          Greet
        </button>
      </div>

      <p className="text-xl text-blue-400">{greetMsg}</p>
    </div>
  );
}

export default App;
