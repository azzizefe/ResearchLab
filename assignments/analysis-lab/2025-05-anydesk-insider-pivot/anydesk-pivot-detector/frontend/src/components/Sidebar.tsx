import { LayoutDashboard, ShieldAlert, Settings, FileText, Activity, Info, Globe2 } from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import { useLanguage } from "../store/useLanguage";

export default function Sidebar() {
  const location = useLocation();
  const { lang, setLang, t } = useLanguage();
  
  const navItems = [
    { icon: LayoutDashboard, label: t("dashboard"), path: "/" },
    { icon: ShieldAlert, label: t("alerts"), path: "/alerts" },
    { icon: Activity, label: t("network"), path: "/network" },
    { icon: FileText, label: t("logs"), path: "/logs" },
    { icon: Settings, label: t("settings"), path: "/settings" },
  ];

  return (
    <div className="w-80 glass-panel flex flex-col p-6 m-4 rounded-[2.5rem] overflow-hidden border-white/10 shadow-2xl">
      <div className="flex items-center gap-4 mb-10 px-2">
        <div className="w-14 h-14 bg-gradient-to-br from-cyan-400 via-blue-500 to-purple-600 rounded-2xl flex items-center justify-center font-bold text-2xl shadow-xl shadow-cyan-500/20 ring-1 ring-white/20">
          <ShieldAlert size={32} className="text-white" />
        </div>
        <div>
          <h1 className="font-black text-2xl tracking-tighter leading-none bg-gradient-to-r from-white to-slate-400 bg-clip-text text-transparent">ANYDESK</h1>
          <p className="text-[10px] text-cyan-400 font-black tracking-[0.3em] uppercase mt-1">PIVOT DETECTOR</p>
        </div>
      </div>

      <div className="flex bg-white/5 p-1 rounded-xl mb-8 border border-white/5">
        <button 
          onClick={() => setLang("en")}
          className={`flex-1 flex items-center justify-center gap-2 py-2 rounded-lg text-xs font-bold transition-all ${lang === "en" ? "bg-white/10 text-white shadow-lg" : "text-slate-500 hover:text-slate-300"}`}
        >
          <Globe2 size={14} /> EN
        </button>
        <button 
          onClick={() => setLang("tr")}
          className={`flex-1 flex items-center justify-center gap-2 py-2 rounded-lg text-xs font-bold transition-all ${lang === "tr" ? "bg-white/10 text-white shadow-lg" : "text-slate-500 hover:text-slate-300"}`}
        >
          <Globe2 size={14} /> TR
        </button>
      </div>
      
      <nav className="flex-1 space-y-2">
        {navItems.map((item) => {
          const active = location.pathname === item.path;
          return (
            <Link 
              key={item.path} 
              to={item.path}
              className={`sidebar-item flex items-center gap-4 px-6 py-4 rounded-2xl group ${
                active ? "active" : "text-slate-400 hover:text-white"
              }`}
            >
              <item.icon size={22} className={active ? "text-white" : "group-hover:text-cyan-400 transition-colors"} />
              <span className="font-bold tracking-wide">{item.label}</span>
            </Link>
          );
        })}
      </nav>

      <div className="space-y-4">
        <div className="p-5 bg-gradient-to-br from-white/5 to-transparent rounded-3xl border border-white/5 relative overflow-hidden group">
          <div className="flex items-center gap-3 mb-3">
            <Info size={16} className="text-cyan-400" />
            <span className="text-[10px] font-black uppercase tracking-widest text-slate-300">{t("info_title")}</span>
          </div>
          <p className="text-[11px] leading-relaxed text-slate-400 font-medium">
            {t("info_desc")}
          </p>
          <div className="absolute -right-4 -bottom-4 w-16 h-16 bg-cyan-500/10 rounded-full blur-2xl group-hover:bg-cyan-500/20 transition-all"></div>
        </div>

        <div className="p-5 bg-white/5 rounded-3xl border border-white/5">
          <div className="text-[9px] text-slate-500 uppercase font-black mb-3 tracking-[0.2em]">{t("system_status")}</div>
          <div className="flex items-center gap-3">
            <div className="relative">
              <div className="w-3 h-3 bg-cyan-400 rounded-full animate-ping absolute"></div>
              <div className="w-3 h-3 bg-cyan-400 rounded-full relative"></div>
            </div>
            <span className="text-xs font-black text-cyan-400 glow-text uppercase tracking-widest">{t("active")}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
