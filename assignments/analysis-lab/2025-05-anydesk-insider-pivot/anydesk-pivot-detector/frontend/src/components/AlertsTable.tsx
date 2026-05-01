import { useStore } from "../store/useStore";
import { Search, Filter, Download } from "lucide-react";

export default function AlertsTable() {
  const { alerts } = useStore();

  return (
    <div className="p-8 space-y-6">
      <header className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">Threat Intelligence</h1>
          <p className="text-slate-400">Detailed list of all detected security events.</p>
        </div>
        <div className="flex gap-4">
          <button className="flex items-center gap-2 px-4 py-2 bg-slate-800 rounded-lg hover:bg-slate-700 transition-colors">
            <Download size={18} /> Export
          </button>
        </div>
      </header>

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
