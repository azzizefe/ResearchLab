import { useState } from "react";
import { Shield, Zap, Database, Lock, Clock, Search, ChevronRight, Binary, Fingerprint, Eye, Settings } from "lucide-react";
import PageHeader from "./PageHeader";
import { useLanguage } from "../store/useLanguage";

export default function ThreatModel() {
  const { t } = useLanguage();
  const [activeTab, setActiveTab] = useState(0);

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
    <div className="p-10 space-y-10 h-full flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <PageHeader 
        title={t("network")} 
        subtitle="Cyber Security Operations Center - Multi-Module Analysis" 
      />

      <div className="flex-1 flex gap-8 min-h-0">
        {/* Sidebar Navigation */}
        <div className="w-80 flex flex-col gap-3">
          {capabilities.map((cap, i) => (
            <button
              key={cap.id}
              onClick={() => setActiveTab(i)}
              className={`flex items-center gap-4 p-5 rounded-2xl border transition-all text-left group ${
                activeTab === i 
                ? `${cap.bg} ${cap.border} shadow-lg shadow-black/20` 
                : "bg-transparent border-transparent hover:bg-white/5 text-slate-500"
              }`}
            >
              <div className={`p-2 rounded-lg ${activeTab === i ? cap.bg : "bg-white/5"}`}>
                <cap.icon size={20} className={activeTab === i ? cap.color : "text-slate-500"} />
              </div>
              <div className="flex-1">
                <div className={`text-sm font-black uppercase tracking-tight ${activeTab === i ? "text-white" : "group-hover:text-slate-300"}`}>
                  {cap.title}
                </div>
                <div className="text-[10px] font-bold text-slate-600 tracking-widest">{cap.tag}</div>
              </div>
              <ChevronRight size={16} className={`transition-transform ${activeTab === i ? "rotate-90 opacity-100" : "opacity-0"}`} />
            </button>
          ))}
        </div>

        {/* Dynamic Content Area */}
        <div className="flex-1 glass-panel rounded-[3rem] p-12 relative overflow-hidden flex flex-col border-white/5 shadow-2xl">
          {/* Decorative Background Icon */}
          <active.icon className={`absolute -right-20 -bottom-20 w-96 h-96 ${active.color} opacity-5 rotate-12 transition-all duration-700`} />

          <div className="relative z-10 space-y-12">
            <div className="flex items-center justify-between">
              <div className="space-y-2">
                <span className={`text-[10px] font-black px-3 py-1 rounded-full ${active.bg} border ${active.border} ${active.color} tracking-[0.3em]`}>
                  SECURITY MODULE 0{activeTab + 1}
                </span>
                <h2 className="text-5xl font-black text-white tracking-tighter italic uppercase">{active.title}</h2>
              </div>
              <div className="flex items-center gap-6">
                <div className="flex flex-col items-end">
                  <span className="text-[10px] font-black text-slate-500 uppercase tracking-widest">Protection Status</span>
                  <span className="text-sm font-black text-cyan-400 glow-text uppercase">ACTIVE & SHIELDED</span>
                </div>
                <div className="w-12 h-12 bg-cyan-500/10 border border-cyan-400/20 rounded-full flex items-center justify-center">
                  <Fingerprint className="text-cyan-400 animate-pulse" size={24} />
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-2 gap-12">
              <div className="space-y-10">
                <div className="space-y-4">
                  <div className="flex items-center gap-3">
                    <Binary className={active.color} size={20} />
                    <span className="text-xs font-black text-slate-400 uppercase tracking-widest">{t("tr") === "tr" ? "NEDİR?" : "WHAT IS IT?"}</span>
                  </div>
                  <p className="text-xl text-slate-200 font-medium leading-relaxed">
                    {active.what}
                  </p>
                </div>

                <div className="space-y-4">
                  <div className="flex items-center gap-3">
                    <Eye className="text-cyan-400" size={20} />
                    <span className="text-xs font-black text-slate-400 uppercase tracking-widest">{t("tr") === "tr" ? "NASIL TESPİT EDİLİR?" : "HOW TO DETECT?"}</span>
                  </div>
                  <div className="p-8 bg-white/5 rounded-3xl border border-white/5 italic text-slate-400 leading-relaxed font-medium">
                    {active.how}
                  </div>
                </div>
              </div>

              <div className="glass-panel p-8 rounded-3xl bg-slate-950/50 border-white/5 space-y-6 self-start">
                <div className="flex items-center gap-3 mb-2">
                  <div className="w-2 h-2 bg-red-500 rounded-full animate-ping"></div>
                  <span className="text-[10px] font-black text-slate-500 uppercase tracking-widest">Technical Logic Flow</span>
                </div>
                <div className="space-y-4">
                  <div className="flex items-center gap-4">
                    <div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-xs font-black text-white">01</div>
                    <div className="text-xs text-slate-400 font-bold uppercase tracking-wide">Data Source: <span className="text-white">anydesk.ad.trace</span></div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-xs font-black text-white">02</div>
                    <div className="text-xs text-slate-400 font-bold uppercase tracking-wide">Heuristic: <span className="text-white">{active.technical}</span></div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-xs font-black text-white">03</div>
                    <div className="text-xs text-slate-400 font-bold uppercase tracking-wide">Output: <span className="text-cyan-400">Security Alert & Forensics Log</span></div>
                  </div>
                </div>
                <div className="mt-8 pt-6 border-t border-white/5 text-[10px] text-slate-600 leading-relaxed">
                  This module is hardened against memory-corruption attacks and runs in a strictly isolated Rust sandbox.
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
