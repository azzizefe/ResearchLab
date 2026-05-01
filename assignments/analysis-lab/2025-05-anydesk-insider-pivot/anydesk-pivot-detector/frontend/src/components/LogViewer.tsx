import { Terminal } from "lucide-react";

export default function LogViewer() {
  const mockLogs = [
    "[2025-05-01 19:30:12.443] INFO  anydesk - Starting service...",
    "[2025-05-01 19:31:05.112] INFO  anydesk - Incoming connection from 123 456 789",
    "[2025-05-01 19:31:07.882] WARN  anydesk - Password accepted (Unattended Access)",
    "[2025-05-01 19:32:15.001] INFO  anydesk - File transfer started: passwords.txt",
    "[2025-05-01 19:32:45.332] ERROR detector - Suspicious process 'cmd.exe' started by AnyDesk",
  ];

  return (
    <div className="p-8 space-y-6">
      <header>
        <h1 className="text-3xl font-bold">Real-time Logs</h1>
        <p className="text-slate-400">Monitoring AnyDesk trace files and system events.</p>
      </header>

      <div className="bg-slate-950 border border-slate-800 rounded-2xl overflow-hidden font-mono text-sm leading-relaxed">
        <div className="p-3 border-b border-slate-800 bg-slate-900 flex items-center gap-2">
          <Terminal size={16} className="text-slate-500" />
          <span className="text-slate-500 font-bold uppercase text-[10px]">ad.trace output</span>
        </div>
        <div className="p-6 h-[500px] overflow-y-auto space-y-1">
          {mockLogs.map((log, i) => (
            <div key={i} className="group flex gap-4">
              <span className="text-slate-700 select-none">{i + 1}</span>
              <span className={log.includes("ERROR") ? "text-red-400" : log.includes("WARN") ? "text-yellow-400" : "text-slate-300"}>
                {log}
              </span>
            </div>
          ))}
          <div className="animate-pulse inline-block w-2 h-4 bg-blue-500 ml-10"></div>
        </div>
      </div>
    </div>
  );
}
