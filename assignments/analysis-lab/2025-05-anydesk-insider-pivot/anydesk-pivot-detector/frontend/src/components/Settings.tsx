import { useState } from "react";
import { Shield, Save, Info, AlertCircle } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import PageHeader from "./PageHeader";

export default function Settings() {
  const [strictMode, setStrictMode] = useState(true);
  const [monitorFiles, setMonitorFiles] = useState(true);
  const [saveStatus, setSaveStatus] = useState<"idle" | "saving" | "success">("idle");

  const handleSave = async () => {
    setSaveStatus("saving");
    try {
      await invoke("update_config", { strictMode, monitorFiles });
      setSaveStatus("success");
      setTimeout(() => setSaveStatus("idle"), 2000);
    } catch (error) {
      console.error("Failed to save config:", error);
      setSaveStatus("idle");
    }
  };

  const rules = [
    {
      id: "strict",
      title: "Strict Pivot Detection",
      desc: "Flag all incoming connections outside ACL",
      info: "Bu ayar aktif olduğunda, sistem önceden tanımlanmış güvenli AnyDesk ID'leri dışındaki her türlü bağlantıyı anında 'Kritik' saldırı olarak işaretler. Maksimum güvenlik için önerilir.",
      state: strictMode,
      setter: setStrictMode,
    },
    {
      id: "files",
      title: "File Transfer Monitoring",
      desc: "Track all file system interactions via AnyDesk",
      info: "AnyDesk üzerinden yapılan her türlü dosya kopyalama, silme veya okuma işlemini takip eder. Veri sızıntılarını (Exfiltration) engellemek için kritik bir özelliktir.",
      state: monitorFiles,
      setter: setMonitorFiles,
    },
  ];

  return (
    <div className="p-10 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000 h-full overflow-y-auto">
      <PageHeader 
        title="Settings" 
        subtitle="Configure detection rules and system thresholds." 
      />

      <div className="max-w-3xl space-y-8">
        {/* Detection Rules Section */}
        <div className="glass-panel p-8 rounded-[2.5rem] bg-white/[0.02] border-white/5 space-y-8">
          <div className="flex items-center gap-3 text-cyan-400">
            <Shield size={20} />
            <h2 className="text-xl font-black uppercase tracking-tighter italic">Detection Rules</h2>
          </div>

          <div className="space-y-4">
            {rules.map((rule) => (
              <div key={rule.id} className="p-6 rounded-3xl bg-slate-950/40 border border-white/5 hover:border-white/10 transition-all group">
                <div className="flex items-center justify-between mb-4">
                  <div className="space-y-1">
                    <div className="flex items-center gap-2">
                      <h3 className="font-black text-white uppercase tracking-tight">{rule.title}</h3>
                      <div className="relative group/info">
                        <Info size={14} className="text-slate-500 cursor-help hover:text-cyan-400 transition-colors" />
                        <div className="absolute left-6 top-0 w-64 p-4 rounded-2xl bg-slate-900 border border-white/10 shadow-2xl opacity-0 invisible group-hover/info:opacity-100 group-hover/info:visible transition-all z-50 text-[11px] font-medium text-slate-300 leading-relaxed">
                          {rule.info}
                        </div>
                      </div>
                    </div>
                    <p className="text-xs text-slate-500 font-bold">{rule.desc}</p>
                  </div>
                  
                  <button
                    onClick={() => rule.setter(!rule.state)}
                    className={`relative w-14 h-7 rounded-full transition-all duration-300 ${
                      rule.state ? "bg-cyan-500 shadow-[0_0_15px_rgba(6,182,212,0.4)]" : "bg-slate-800"
                    }`}
                  >
                    <div className={`absolute top-1 w-5 h-5 bg-white rounded-full transition-all duration-300 ${
                      rule.state ? "left-8" : "left-1"
                    }`}></div>
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Action Button */}
        <button
          onClick={handleSave}
          disabled={saveStatus === "saving"}
          className={`group flex items-center justify-center gap-3 px-10 py-5 rounded-3xl font-black uppercase tracking-widest text-sm transition-all ${
            saveStatus === "success" 
            ? "bg-green-500 text-white shadow-[0_0_20px_rgba(34,197,94,0.4)]" 
            : "bg-gradient-to-r from-cyan-500 to-purple-600 text-white hover:scale-105 active:scale-95 shadow-xl shadow-cyan-500/20"
          }`}
        >
          {saveStatus === "success" ? (
            <>
              <Shield size={20} className="animate-bounce" />
              Configuration Saved!
            </>
          ) : (
            <>
              <Save size={20} className={saveStatus === "saving" ? "animate-spin" : "group-hover:rotate-12 transition-transform"} />
              {saveStatus === "saving" ? "Saving..." : "Save Configuration"}
            </>
          )}
        </button>

        {/* Security Note */}
        <div className="flex items-start gap-4 p-6 rounded-[2rem] bg-red-500/5 border border-red-500/10">
          <AlertCircle className="text-red-400 shrink-0" size={20} />
          <p className="text-[11px] text-slate-500 font-medium leading-relaxed uppercase">
            <span className="text-red-400 font-black">Warning:</span> Changes made here are applied immediately to the real-time analyzer engine. Ensure you have authorized these IDs before enabling strict mode.
          </p>
        </div>
      </div>
    </div>
  );
}
