import { useStore } from "../store/useStore";
import { Search, Filter, Download, ShieldAlert } from "lucide-react";
import PageHeader from "./PageHeader";

export default function AlertsTable() {
  const { alerts } = useStore();

  return (
    <div className="p-10 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <div className="flex justify-between items-start">
        <PageHeader 
          title="Threat Intelligence" 
          subtitle="Detailed list of all detected security events." 
        />
        <button className="flex items-center gap-3 px-6 py-3 bg-white/5 border border-white/10 rounded-2xl hover:bg-white/10 transition-all font-bold text-sm">
          <Download size={20} className="text-cyan-400" /> 
          <span>EXPORT LOGS</span>
        </button>
      </div>

      <div className="bg-slate-800/40 border border-slate-700/50 rounded-2xl overflow-hidden">
        <div className="p-4 border-b border-slate-700/50 flex gap-4">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-2.5 text-slate-500" size={18} />
            <input 
              placeholder="Search alerts..." 
              className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 focus:outline-none focus:border-blue-500"
            />
          </div>
          <button className="flex items-center gap-2 px-4 py-2 bg-slate-800 rounded-lg">
            <Filter size={18} /> Filter
          </button>
        </div>
        
        <table className="w-full text-left">
          <thead>
            <tr className="bg-slate-900/50 text-slate-400 text-sm">
              <th className="px-6 py-4 font-medium">Timestamp</th>
              <th className="px-6 py-4 font-medium">Severity</th>
              <th className="px-6 py-4 font-medium">Event Title</th>
              <th className="px-6 py-4 font-medium">Source</th>
              <th className="px-6 py-4 font-medium text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800">
            {alerts.map((alert) => (
              <tr key={alert.id} className="hover:bg-slate-800/30 transition-colors">
                <td className="px-6 py-4 text-sm text-slate-400">
                  {new Date(alert.timestamp).toLocaleString()}
                </td>
                <td className="px-6 py-4">
                  <span className={`px-2 py-1 rounded text-xs font-bold uppercase ${
                    alert.severity === "Critical" ? "bg-red-900/30 text-red-500" :
                    alert.severity === "High" ? "bg-orange-900/30 text-orange-500" :
                    "bg-blue-900/30 text-blue-500"
                  }`}>
                    {alert.severity}
                  </span>
                </td>
                <td className="px-6 py-4 font-medium">{alert.title}</td>
                <td className="px-6 py-4 text-sm text-slate-400">{alert.source_module}</td>
                <td className="px-6 py-4 text-right">
                  <button className="text-blue-400 hover:text-blue-300 text-sm font-medium">Details</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {alerts.length === 0 && (
          <div className="p-12 text-center text-slate-500">No alerts found.</div>
        )}
      </div>
    </div>
  );
}
