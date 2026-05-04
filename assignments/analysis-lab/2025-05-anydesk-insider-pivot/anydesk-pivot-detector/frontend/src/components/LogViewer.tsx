import { useEffect, useRef } from "react";
import { Terminal, Cpu } from "lucide-react";
import { useStore } from "../store/useStore";
import PageHeader from "./PageHeader";

export default function LogViewer() {
  const { logs } = useStore();
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs]);

  const displayLogs = logs.length > 0 ? logs : [
    "Waiting for live logs...",
    "System monitoring active. Every AnyDesk trace entry will appear here.",
  ];

  return (
    <div className="p-10 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <div className="flex justify-between items-start">
        <PageHeader 
          title="Terminal Logs" 
          subtitle="Real-time monitoring of AnyDesk trace and system events." 
        />
        <div className="flex items-center gap-3 px-4 py-2 bg-cyan-400/10 border border-cyan-400/20 rounded-xl">
          <Cpu size={18} className="text-cyan-400 animate-pulse" />
          <span className="text-xs font-bold text-cyan-400 uppercase tracking-widest">Live Streaming</span>
        </div>
      </div>

      <div className="bg-slate-950 border border-slate-800 rounded-2xl overflow-hidden font-mono text-sm leading-relaxed">
        <div className="p-3 border-b border-slate-800 bg-slate-900 flex items-center gap-2">
          <Terminal size={16} className="text-slate-500" />
          <span className="text-slate-500 font-bold uppercase text-[10px]">ad.trace output</span>
        </div>
        <div ref={scrollRef} className="p-6 h-[500px] overflow-y-auto space-y-1">
          {displayLogs.map((log, i) => (
            <div key={i} className="group flex gap-4">
              <span className="text-slate-700 select-none">{i + 1}</span>
              <span className={log.includes("ERROR") || log.includes("Critical") ? "text-red-400" : log.includes("WARN") || log.includes("High") ? "text-yellow-400" : "text-slate-300"}>
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
