import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Sidebar from "./components/Sidebar";
import Dashboard from "./components/Dashboard";
import AlertsTable from "./components/AlertsTable";
import LogViewer from "./components/LogViewer";
import Settings from "./components/Settings";

function App() {
  return (
    <Router>
      <div className="flex h-screen bg-slate-950 text-slate-100 overflow-hidden font-sans selection:bg-cyan-500/30">
        <Sidebar />
        <main className="flex-1 overflow-y-auto bg-[radial-gradient(circle_at_top_right,_var(--tw-gradient-stops))] from-slate-900 via-slate-950 to-slate-950">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/alerts" element={<AlertsTable />} />
            <Route path="/network" element={<div className="p-10"><h1>Network Map (Placeholder)</h1></div>} />
            <Route path="/logs" element={<LogViewer />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
