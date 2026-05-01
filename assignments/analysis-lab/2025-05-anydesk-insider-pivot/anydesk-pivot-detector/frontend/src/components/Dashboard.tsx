import { PieChart, Pie, Cell, ResponsiveContainer } from "recharts";
import { useStore } from "../store/useStore";
import { ShieldAlert, Activity, User, Globe } from "lucide-react";

export default function Dashboard() {
  const { riskScore, alerts } = useStore();

  const data = [
    { name: "Risk", value: riskScore },
    { name: "Safe", value: 100 - riskScore },
  ];

  const COLORS = [riskScore > 70 ? "#ef4444" : riskScore > 40 ? "#f59e0b" : "#3b82f6", "#1e293b"];

  const stats = [
    { label: "Total Alerts", value: alerts.length, icon: ShieldAlert, color: "text-red-500" },
    { label: "Active Connections", value: "3", icon: Activity, color: "text-green-500" },
    { label: "AnyDesk ID", value: "987 654 321", icon: User, color: "text-blue-500" },
    { label: "Remote Location", value: "Istanbul, TR", icon: Globe, color: "text-cyan-500" },
  ];

  return (
    <div className="p-8 space-y-8 animate-in fade-in duration-700">
      <header>
        <h1 className="text-3xl font-bold">System Overview</h1>
        <p className="text-slate-400">Real-time threat monitoring and pivot detection.</p>
      </header>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {stats.map((stat, i) => (
          <div key={i} className="bg-slate-800/40 border border-slate-700/50 p-6 rounded-2xl backdrop-blur-sm">
            <div className="flex items-center justify-between mb-4">
              <stat.icon className={stat.color} size={24} />
            </div>
            <div className="text-2xl font-bold">{stat.value}</div>
            <div className="text-sm text-slate-400">{stat.label}</div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Risk Gauge */}
        <div className="lg:col-span-1 bg-slate-800/40 border border-slate-700/50 p-8 rounded-2xl flex flex-col items-center">
          <h2 className="text-xl font-bold mb-4 self-start">Risk Score</h2>
          <div className="relative w-full h-64">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={data}
                  innerRadius={60}
                  outerRadius={80}
                  startAngle={180}
                  endAngle={0}
                  paddingAngle={0}
                  dataKey="value"
                  stroke="none"
                >
                  {data.map((_, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
              </PieChart>
            </ResponsiveContainer>
            <div className="absolute inset-0 flex flex-col items-center justify-center pt-8">
              <span className="text-5xl font-bold">{riskScore}</span>
              <span className="text-xs uppercase tracking-widest text-slate-500">Hazard Level</span>
            </div>
          </div>
          <div className="text-center mt-[-2rem]">
            <p className={riskScore > 70 ? "text-red-500 font-bold" : "text-slate-400"}>
              {riskScore > 70 ? "CRITICAL THREAT DETECTED" : riskScore > 40 ? "MODERATE ANOMALY" : "SYSTEM STABLE"}
            </p>
          </div>
        </div>

        {/* Recent Alerts */}
        <div className="lg:col-span-2 bg-slate-800/40 border border-slate-700/50 p-8 rounded-2xl">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-xl font-bold">Recent Alerts</h2>
            <button className="text-sm text-blue-400 hover:underline">View All</button>
          </div>
          <div className="space-y-4">
            {alerts.slice(0, 5).map((alert) => (
              <div key={alert.id} className="flex items-center gap-4 p-4 bg-slate-900/50 rounded-xl border border-slate-800">
                <div className={`w-2 h-10 rounded-full ${
                  alert.severity === "Critical" ? "bg-red-600" : alert.severity === "High" ? "bg-orange-500" : "bg-blue-500"
                }`}></div>
                <div className="flex-1">
                  <div className="font-bold">{alert.title}</div>
                  <div className="text-sm text-slate-400 line-clamp-1">{alert.description}</div>
                </div>
                <div className="text-xs text-slate-500">{new Date(alert.timestamp).toLocaleTimeString()}</div>
              </div>
            ))}
            {alerts.length === 0 && (
              <div className="h-48 flex items-center justify-center text-slate-500 italic">
                No threats detected. System is clean.
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
