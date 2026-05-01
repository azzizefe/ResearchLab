import { useStore } from "../store/useStore";
import { CheckCircle, AlertTriangle, ShieldCheck } from "lucide-react";

export default function ConfigChecker() {
  const configs = [
    { name: "Unattended Access", status: "Secure", detail: "Disabled", icon: ShieldCheck, color: "text-green-500" },
    { name: "Access Control List", status: "Risk", detail: "Allowing all IDs", icon: AlertTriangle, color: "text-yellow-500" },
    { name: "Password Security", status: "Optimal", detail: "Strong token used", icon: CheckCircle, color: "text-green-500" },
    { name: "Direct Connection", status: "Monitor", detail: "Port 6568 listening", icon: Activity, color: "text-blue-500" },
  ];

  return (
    <div className="p-8 space-y-6">
      <header>
        <h1 className="text-3xl font-bold">Security Posture</h1>
        <p className="text-slate-400">Analysis of AnyDesk configuration and system hardening.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {configs.map((cfg, i) => (
          <div key={i} className="bg-slate-800/40 border border-slate-700/50 p-6 rounded-2xl flex items-center gap-6">
            <div className={`p-4 rounded-xl bg-slate-900/50 ${cfg.color}`}>
              <cfg.icon size={32} />
            </div>
            <div className="flex-1">
              <div className="text-lg font-bold">{cfg.name}</div>
              <div className="text-sm text-slate-400">{cfg.detail}</div>
            </div>
            <div className={`text-sm font-bold uppercase tracking-widest ${cfg.color}`}>
              {cfg.status}
            </div>
          </div>
        ))}
      </div>

      <div className="bg-blue-900/20 border border-blue-800/50 p-6 rounded-2xl">
        <h3 className="text-blue-400 font-bold mb-2">Hardening Recommendation</h3>
        <p className="text-sm text-blue-200/70">
          We detected that your Access Control List is not configured. 
          It is highly recommended to restrict AnyDesk access to specific IDs to prevent unauthorized pivot attempts.
        </p>
        <button className="mt-4 px-4 py-2 bg-blue-600 rounded-lg text-sm font-bold">Apply Hardening</button>
      </div>
    </div>
  );
}

function Activity(props: any) {
  return <ShieldCheck {...props} />
}
