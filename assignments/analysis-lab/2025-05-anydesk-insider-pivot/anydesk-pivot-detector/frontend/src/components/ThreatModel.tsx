import { useState } from "react";
import { Shield, Zap, Database, Lock, Clock, Search, ChevronRight, Binary, Fingerprint, Eye, Power, Settings, Activity } from "lucide-react";
import PageHeader from "./PageHeader";
import { useLanguage } from "../store/useLanguage";

export default function ThreatModel() {
  const { t } = useLanguage();
  const [activeTab, setActiveTab] = useState(0);
  const [systemStates, setSystemStates] = useState<Record<string, boolean>>({
    bypass: true,
    persistence: true,
    exfiltration: true,
    realtime: true,
    scorer: true,
    forensics: true,
    cli: false,
    config: true,
    reporting: true
  });

  const toggleSystem = (id: string) => {
    setSystemStates(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const capabilities = [
    { 
      id: "bypass",
      title: t("bypass_title"), 
      what: t("bypass_what"),
      how: t("bypass_how"),
      icon: Zap, 
      color: "text-yellow-400", 
      bg: "bg-yellow-400/10",
      border: "border-yellow-400/20",
      tag: "BYPASS",
      technical: "Frame Header Analysis & Protocol Tunneling Detection"
    },
    { 
      id: "persistence",
      title: t("persistence_title"), 
      what: t("persistence_what"),
      how: t("persistence_how"),
      icon: Lock, 
      color: "text-purple-400", 
      bg: "bg-purple-400/10",
      border: "border-purple-400/20",
      tag: "PERSISTENCE",
      technical: "Registry & service.conf Integrity Monitoring"
    },
    { 
      id: "exfiltration",
      title: t("exfiltration_title"), 
      what: t("exfiltration_what"),
      how: t("exfiltration_how"),
      icon: Database, 
      color: "text-blue-400", 
      bg: "bg-blue-400/10",
      border: "border-blue-400/20",
      tag: "EXFILTRATION",
      technical: "Network Throughput & File Extension Correlation"
    },
    { 
      id: "realtime",
      title: t("realtime_title"), 
      what: t("realtime_what"),
      how: t("realtime_how"),
      icon: Clock, 
      color: "text-cyan-400", 
      bg: "bg-cyan-400/10",
      border: "border-cyan-400/20",
      tag: "REAL-TIME",
      technical: "Kernel-Level FileWatcher (Sub-second Latency)"
    },
    { 
      id: "scorer",
      title: t("scorer_title"), 
      what: t("scorer_what"),
      how: t("scorer_how"),
      icon: Shield, 
      color: "text-green-400", 
      bg: "bg-green-400/10",
      border: "border-green-400/20",
      tag: "SCORER",
      technical: "Heuristic Behavioral Risk Weighting Engine"
    },
    { 
      id: "forensics",
      title: t("forensics_title"), 
      what: t("forensics_what"),
      how: t("forensics_how"),
      icon: Search, 
      color: "text-red-400", 
      bg: "bg-red-400/10",
      border: "border-red-400/20",
      tag: "FORENSICS",
      technical: "Session Reconstruction & Command Timeline Generation"
    },
    { 
      id: "cli",
      title: t("cli_title"), 
      what: t("cli_what"),
      how: t("cli_how"),
      icon: Binary, 
      color: "text-orange-400", 
      bg: "bg-orange-400/10",
      border: "border-orange-400/20",
      tag: "CLI ENGINE",
      technical: "Headless Scanning & Automated Shell Integration"
    },
    { 
      id: "config",
      title: t("config_title"), 
      what: t("config_what"),
      how: t("config_how"),
      icon: Settings, 
      color: "text-slate-200", 
      bg: "bg-white/10",
      border: "border-white/20",
      tag: "INFRASTRUCTURE",
      technical: "Dynamic Policy Enforcement & Hot-Reloading Config"
    },
    { 
      id: "reporting",
      title: t("reporting_title"), 
      what: t("reporting_what"),
      how: t("reporting_how"),
      icon: Database, 
      color: "text-pink-400", 
      bg: "bg-pink-400/10",
      border: "border-pink-400/20",
      tag: "ARTIFACTS",
      technical: "Cryptographic Evidence Signing & JSON Export"
    },
  ];

  const active = capabilities[activeTab];

  return (
    <div className="p-10 space-y-10 h-full flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-1000 overflow-hidden">
      <PageHeader 
        title={t("network")} 
        subtitle="Security Command Center - Advanced Module Orchestration" 
      />

      <div className="flex-1 flex gap-8 min-h-0 overflow-hidden">
        {/* Sidebar Navigation */}
        <div className="w-80 flex flex-col gap-3 overflow-y-auto pr-2 custom-scrollbar">
          {capabilities.map((cap, i) => (
            <button
              key={cap.id}
              onClick={() => setActiveTab(i)}
              className={`flex items-center gap-4 p-5 rounded-2xl border transition-all text-left group relative ${
                activeTab === i 
                ? `${cap.bg} ${cap.border} shadow-lg shadow-black/20` 
                : "bg-transparent border-transparent hover:bg-white/5 text-slate-500"
              }`}
            >
              <div className={`p-2 rounded-lg ${activeTab === i ? cap.bg : "bg-white/5"}`}>
                <cap.icon size={20} className={activeTab === i ? cap.color : "text-slate-500"} />
              </div>
              <div className="flex-1">
                <div className={`text-xs font-black uppercase tracking-tight ${activeTab === i ? "text-white" : "group-hover:text-slate-300"}`}>
                  {cap.title}
                </div>
                <div className="flex items-center gap-2">
                  <div className={`w-1.5 h-1.5 rounded-full ${systemStates[cap.id] ? "bg-cyan-500 shadow-[0_0_8px_rgba(6,182,212,0.6)]" : "bg-slate-700"}`}></div>
                  <div className="text-[9px] font-bold text-slate-600 tracking-widest uppercase">
                    {systemStates[cap.id] ? "Online" : "Offline"}
                  </div>
                </div>
              </div>
              <ChevronRight size={14} className={`transition-transform ${activeTab === i ? "rotate-90 opacity-100" : "opacity-0"}`} />
            </button>
          ))}
        </div>

        {/* Dynamic Content Area (Command Room) */}
        <div className="flex-1 glass-panel rounded-[3.5rem] p-12 relative overflow-hidden flex flex-col border-white/5 shadow-2xl bg-gradient-to-br from-slate-900/50 to-transparent">
          {/* Active System Aura */}
          <div className={`absolute -right-20 -top-20 w-96 h-96 ${active.bg.replace('/10', '/5')} blur-[120px] rounded-full`}></div>

          <div className="relative z-10 flex flex-col h-full">
            {/* Header Section */}
            <div className="flex items-start justify-between mb-12">
              <div className="space-y-4">
                <div className="flex items-center gap-4">
                  <div className={`p-4 ${active.bg} rounded-3xl border ${active.border}`}>
                    <active.icon className={active.color} size={32} />
                  </div>
                  <div>
                    <h2 className="text-4xl font-black text-white tracking-tighter uppercase italic leading-none mb-1">{active.title}</h2>
                    <div className="flex items-center gap-2">
                      <Activity size={12} className="text-cyan-500" />
                      <span className="text-[10px] font-black text-slate-500 uppercase tracking-[0.2em]">Module Terminal ID: {active.id.toUpperCase()}-092</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* System Control Switch */}
              <div className="flex flex-col items-end gap-4">
                <button 
                  onClick={() => toggleSystem(active.id)}
                  className={`flex items-center gap-3 px-6 py-3 rounded-2xl border transition-all ${
                    systemStates[active.id] 
                    ? "bg-cyan-500/10 border-cyan-500/30 text-cyan-400" 
                    : "bg-red-500/10 border-red-500/30 text-red-400"
                  }`}
                >
                  <Power size={18} />
                  <span className="text-xs font-black uppercase tracking-widest">
                    {systemStates[active.id] ? "System Armed" : "System Disarmed"}
                  </span>
                </button>
                <div className="text-[9px] font-bold text-slate-600 uppercase tracking-widest italic">Authorization: efe@research-lab</div>
              </div>
            </div>

            {/* Analysis Grid */}
            <div className="flex-1 grid grid-cols-1 lg:grid-cols-5 gap-10 min-h-0 overflow-y-auto pr-4 custom-scrollbar">
              <div className="lg:col-span-3 space-y-12 pb-10">
                <section className="space-y-4">
                  <div className="flex items-center gap-3 text-slate-500">
                    <Binary size={16} />
                    <span className="text-[10px] font-black uppercase tracking-widest">Conceptual Breakdown</span>
                  </div>
                  <div className="glass-panel p-8 rounded-[2rem] bg-white/[0.02] border-white/5 leading-relaxed">
                    <p className="text-lg text-slate-300 font-medium">{active.what}</p>
                  </div>
                </section>

                <section className="space-y-4">
                  <div className="flex items-center gap-3 text-slate-500">
                    <Eye size={16} />
                    <span className="text-[10px] font-black uppercase tracking-widest">Heuristic Detection Logic</span>
                  </div>
                  <div className="p-8 bg-slate-950/40 rounded-[2rem] border border-white/5 space-y-4">
                    <p className="text-sm text-slate-400 leading-relaxed font-medium italic">{active.how}</p>
                    <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                      <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                      <span className="text-[9px] font-black text-slate-600 uppercase tracking-widest underline decoration-dotted">Live Kernel Hook: {active.technical}</span>
                    </div>
                  </div>
                </section>
              </div>

              {/* Technical Sidebar */}
              <div className="lg:col-span-2 space-y-8">
                <div className="glass-panel p-8 rounded-[2.5rem] bg-slate-950/60 border-white/5 space-y-8">
                  <h4 className="text-xs font-black text-slate-400 uppercase tracking-[0.2em] mb-4">Command Execution</h4>
                  <div className="space-y-6">
                    {[
                      { l: "Init", v: "ad.trace_stream", s: "Success" },
                      { l: "Analytic", v: active.id + "_ruleset", s: "Ready" },
                      { l: "Heuristic", v: "Behavioral_Match", s: "Idle" }
                    ].map((step, k) => (
                      <div key={k} className="flex items-center justify-between group">
                        <div className="flex items-center gap-4">
                          <div className="w-8 h-8 rounded-xl bg-white/5 flex items-center justify-center text-[10px] font-black text-white group-hover:bg-cyan-500/20 transition-colors">0{k+1}</div>
                          <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">{step.l}</span>
                        </div>
                        <span className="text-[10px] font-black text-slate-300 font-mono">{step.v}</span>
                      </div>
                    ))}
                  </div>
                  
                  <div className="pt-8 border-t border-white/5">
                    <button className="w-full py-4 bg-white/5 hover:bg-white/10 rounded-2xl border border-white/5 text-[10px] font-black text-slate-400 uppercase tracking-widest transition-all">
                      Download Forensics Pack
                    </button>
                  </div>
                </div>

                <div className="p-8 rounded-[2.5rem] bg-gradient-to-br from-cyan-500/10 to-transparent border border-cyan-500/20 space-y-4">
                  <div className="flex items-center gap-2 text-cyan-400">
                    <Shield size={16} />
                    <span className="text-[10px] font-black uppercase tracking-widest">Active Hardening</span>
                  </div>
                  <p className="text-[11px] text-slate-400 font-medium leading-relaxed">
                    This module uses ASLR and stack canaries to prevent exploitation within the analyzer itself.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
