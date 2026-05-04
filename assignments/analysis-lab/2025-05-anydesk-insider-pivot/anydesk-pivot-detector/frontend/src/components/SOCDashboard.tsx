import { useState } from "react";
import { 
  LayoutDashboard, 
  ShieldAlert, 
  Activity, 
  FileText, 
  Settings, 
  Search, 
  Cpu, 
  Globe, 
  Terminal,
  Zap,
  Lock,
  Database,
  Eye,
  Binary
} from "lucide-react";
import { useLanguage } from "../store/useLanguage";
import Dashboard from "./Dashboard";
import AlertsTable from "./AlertsTable";
import LogViewer from "./LogViewer";
import ThreatModel from "./ThreatModel";
import ConfigChecker from "./ConfigChecker";
import PageHeader from "./PageHeader";

type TabType = "overview" | "modules" | "alerts" | "logs" | "forensics";

export default function SOCDashboard() {
  const [activeTab, setActiveTab] = useState<TabType>("overview");
  const { t } = useLanguage();

  const tabs = [
    { id: "overview", label: t("dashboard"), icon: LayoutDashboard },
    { id: "modules", label: "Detection Modules", icon: ShieldAlert },
    { id: "alerts", label: t("alerts"), icon: Activity },
    { id: "logs", label: t("logs"), icon: FileText },
    { id: "forensics", label: "Forensics", icon: Search },
  ];

  const renderContent = () => {
    switch (activeTab) {
      case "overview":
        return <Dashboard />;
      case "modules":
        return <DetectionModules />;
      case "alerts":
        return <AlertsTable />;
      case "logs":
        return <LogViewer />;
      case "forensics":
        return <ThreatModel />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="flex flex-col h-full animate-in fade-in duration-700">
      {/* Tab Navigation */}
      <div className="px-10 pt-10">
        <div className="flex items-center gap-2 p-1.5 bg-slate-900/50 backdrop-blur-xl border border-white/5 rounded-[2rem] w-fit">
          {tabs.map((tab) => {
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as TabType)}
                className={`flex items-center gap-3 px-6 py-3 rounded-[1.5rem] text-sm font-bold transition-all ${
                  isActive 
                  ? "bg-cyan-500 text-white shadow-lg shadow-cyan-500/20" 
                  : "text-slate-500 hover:text-slate-300 hover:bg-white/5"
                }`}
              >
                <tab.icon size={18} />
                {tab.label}
              </button>
            );
          })}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {renderContent()}
      </div>
    </div>
  );
}

function DetectionModules() {
  const { t } = useLanguage();
  const [selectedModule, setSelectedModule] = useState(0);

  const modules = [
    { 
      id: "process", 
      title: "Process Monitor", 
      icon: Cpu, 
      color: "text-purple-400",
      desc: "Monitors suspicious child processes spawned by AnyDesk (e.g. cmd.exe, powershell.exe).",
      educational: t("persistence_how"),
      technical: "Kernel-level Process Creation Hooking",
      status: "Active"
    },
    { 
      id: "network", 
      title: "Network Analyzer", 
      icon: Globe, 
      color: "text-blue-400",
      desc: "Detects unauthorized port forwarding and reverse tunneling attempts.",
      educational: t("bypass_how"),
      technical: "Protocol Fingerprinting & Traffic Analysis",
      status: "Active"
    },
    { 
      id: "config", 
      title: "Config Integrity", 
      icon: Settings, 
      color: "text-slate-400",
      desc: "Audits AnyDesk settings for dangerous configurations like Unattended Access.",
      educational: t("config_how"),
      technical: "Static File Analysis (system.conf)",
      status: "Active"
    },
    { 
      id: "log", 
      title: "Log Intelligence", 
      icon: FileText, 
      color: "text-cyan-400",
      desc: "Parses AnyDesk ad.trace files in real-time to find lateral movement patterns.",
      educational: t("realtime_how"),
      technical: "Heuristic Pattern Matching",
      status: "Active"
    },
    { 
      id: "anomaly", 
      title: "Anomaly Scorer", 
      icon: Zap, 
      color: "text-yellow-400",
      desc: "Calculates behavioral risk scores based on historical activity patterns.",
      educational: t("scorer_how"),
      technical: "Statistical Deviation Model",
      status: "Active"
    },
    { 
      id: "pivot", 
      title: "Pivot Detector", 
      icon: Binary, 
      color: "text-red-400",
      desc: "Main engine that correlates all events to detect active insider pivoting.",
      educational: t("forensics_how"),
      technical: "Cross-Module Event Correlation",
      status: "Active"
    }
  ];

  return (
    <div className="p-10 space-y-10 animate-in slide-in-from-bottom-4 duration-1000">
      <PageHeader 
        title="Detection Modules" 
        subtitle="Comprehensive security analysis engines for AnyDesk monitoring."
      />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {modules.map((mod, i) => (
          <div 
            key={mod.id}
            className={`glass-panel p-8 rounded-[2.5rem] border transition-all cursor-pointer group hover:scale-[1.02] ${
              selectedModule === i ? "border-cyan-500/50 bg-cyan-500/5" : "border-white/5 hover:border-white/10"
            }`}
            onClick={() => setSelectedModule(i)}
          >
            <div className="flex items-center justify-between mb-6">
              <div className={`p-4 rounded-2xl bg-white/5 ${mod.color}`}>
                <mod.icon size={28} />
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-[10px] font-black text-slate-500 uppercase tracking-widest">{mod.status}</span>
              </div>
            </div>
            
            <h3 className="text-xl font-bold text-white mb-2 group-hover:text-cyan-400 transition-colors">{mod.title}</h3>
            <p className="text-sm text-slate-400 font-medium mb-6 leading-relaxed">{mod.desc}</p>
            
            <div className="pt-6 border-t border-white/5">
              <div className="flex items-center gap-2 text-slate-500 mb-2">
                <Eye size={14} />
                <span className="text-[10px] font-black uppercase tracking-widest">SOC Knowledge</span>
              </div>
              <p className="text-[11px] text-slate-500 italic leading-relaxed">{mod.educational}</p>
            </div>
          </div>
        ))}
      </div>

      <div className="glass-panel p-10 rounded-[3rem] bg-gradient-to-r from-slate-900 to-slate-950 border-white/5">
        <div className="flex items-center gap-6">
          <div className={`p-6 rounded-[2rem] bg-white/5 ${modules[selectedModule].color}`}>
            <modules[selectedModule].icon size={48} />
          </div>
          <div>
            <h2 className="text-3xl font-black text-white italic uppercase tracking-tighter mb-1">
              {modules[selectedModule].title} Analysis
            </h2>
            <div className="flex items-center gap-3">
              <Terminal size={14} className="text-cyan-400" />
              <span className="text-xs font-bold text-slate-500 tracking-widest uppercase">
                Technical Implementation: {modules[selectedModule].technical}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
