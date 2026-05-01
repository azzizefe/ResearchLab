import { PieChart, Pie, Cell, ResponsiveContainer } from "recharts";
import { useStore } from "../store/useStore";
import { useLanguage } from "../store/useLanguage";
import { ShieldAlert, Activity, User, Globe } from "lucide-react";
import PageHeader from "./PageHeader";

export default function Dashboard() {
  const { riskScore, alerts } = useStore();
  const { t } = useLanguage();

  const data = [
    { name: "Risk", value: riskScore },
    { name: "Safe", value: 100 - riskScore },
  ];

  const COLORS = [riskScore > 70 ? "#00f2ff" : riskScore > 40 ? "#f59e0b" : "#00f2ff", "rgba(255,255,255,0.05)"];

  const stats = [
    { label: t("total_alerts"), value: alerts.length, icon: ShieldAlert, color: "text-red-400" },
    { label: t("active"), value: "3", icon: Activity, color: "text-cyan-400" },
    { label: t("anydesk_id"), value: "987 654 321", icon: User, color: "text-purple-400" },
    { label: "Remote Location", value: "Istanbul, TR", icon: Globe, color: "text-blue-400" },
  ];

  return (
    <div className="p-10 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <PageHeader 
        title={t("dashboard")} 
        subtitle="Real-time pivot detection and behavior analysis." 
      />

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        {stats.map((stat, i) => (
          <div key={i} className="glass-panel p-8 rounded-3xl group hover:scale-[1.02] transition-transform">
            <div className="flex items-center justify-between mb-6">
              <div className={`p-3 rounded-2xl bg-white/5 border border-white/5 ${stat.color}`}>
                <stat.icon size={28} />
              </div>
            </div>
            <div className="text-4xl font-black text-white glow-text mb-1">{stat.value}</div>
            <div className="text-sm font-bold text-slate-500 uppercase tracking-widest">{stat.label}</div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Risk Gauge */}
        <div className="lg:col-span-1 glass-panel p-10 rounded-[2.5rem] flex flex-col items-center">
          <h2 className="text-xl font-bold mb-8 self-start text-white">Risk Analysis</h2>
          <div className="relative w-full h-64">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={data}
                  innerRadius={70}
                  outerRadius={90}
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
              <span className="text-6xl font-black text-white glow-text">{riskScore}</span>
              <span className="text-[10px] uppercase font-black tracking-[0.2em] text-slate-500 mt-2">Hazard Level</span>
            </div>
          </div>
          <div className="text-center mt-[-2rem]">
            <p className={`text-sm font-black tracking-widest uppercase ${riskScore > 70 ? "text-red-500" : "text-cyan-400"}`}>
              {riskScore > 70 ? "CRITICAL THREAT" : riskScore > 40 ? "MODERATE ANOMALY" : "SYSTEM SECURE"}
            </p>
          </div>
        </div>

        {/* Recent Alerts */}
        <div className="lg:col-span-2 glass-panel p-10 rounded-[2.5rem]">
          <div className="flex justify-between items-center mb-8">
            <h2 className="text-xl font-bold text-white">Recent Intelligence</h2>
            <button className="text-xs font-black text-cyan-400 uppercase tracking-widest hover:text-white transition-colors">View All Analysis</button>
          </div>
          <div className="space-y-4">
            {alerts.slice(0, 5).map((alert: any) => (
              <div key={alert.id} className="flex items-center gap-6 p-6 bg-white/5 rounded-3xl border border-white/5 hover:bg-white/10 transition-all group">
                <div className={`w-1.5 h-12 rounded-full shadow-lg ${
                  alert.severity === "Critical" ? "bg-red-500 shadow-red-500/20" : alert.severity === "High" ? "bg-orange-500 shadow-orange-500/20" : "bg-cyan-400 shadow-cyan-400/20"
                }`}></div>
                <div className="flex-1">
                  <div className="font-bold text-white text-lg group-hover:text-cyan-400 transition-colors">{alert.title}</div>
                  <div className="text-sm text-slate-500 font-medium line-clamp-1">{alert.description}</div>
                </div>
                <div className="text-[10px] font-black text-slate-600 uppercase tracking-tighter bg-white/5 px-3 py-1 rounded-full">{new Date(alert.timestamp).toLocaleTimeString()}</div>
              </div>
            ))}
            {alerts.length === 0 && (
              <div className="h-64 flex flex-col items-center justify-center text-slate-500 gap-4">
                <ShieldAlert size={48} className="opacity-20" />
                <p className="italic font-medium">No threats detected. System is clean.</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
