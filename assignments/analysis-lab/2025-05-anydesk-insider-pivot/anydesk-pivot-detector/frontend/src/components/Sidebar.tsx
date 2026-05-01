import { LayoutDashboard, ShieldAlert, Settings, FileText, Activity } from "lucide-react";
import { Link, useLocation } from "react-router-dom";

export default function Sidebar() {
  const location = useLocation();
  
  const navItems = [
    { icon: LayoutDashboard, label: "Dashboard", path: "/" },
    { icon: ShieldAlert, label: "Alerts", path: "/alerts" },
    { icon: Activity, label: "Network", path: "/network" },
    { icon: FileText, label: "Logs", path: "/logs" },
    { icon: Settings, label: "Settings", path: "/settings" },
  ];

  return (
    <div className="w-64 bg-slate-900 border-r border-slate-800 flex flex-col p-4">
      <div className="flex items-center gap-3 mb-10 px-2">
        <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center font-bold text-xl">AD</div>
        <span className="font-bold text-lg">Pivot Detector</span>
      </div>
      
      <nav className="flex-1 space-y-2">
        {navItems.map((item) => {
          const active = location.pathname === item.path;
          return (
            <Link 
              key={item.path} 
              to={item.path}
              className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-all ${
                active ? "bg-blue-600 text-white" : "text-slate-400 hover:bg-slate-800 hover:text-white"
              }`}
            >
              <item.icon size={20} />
              <span className="font-medium">{item.label}</span>
            </Link>
          );
        })}
      </nav>
      
      <div className="mt-auto p-4 bg-slate-800/50 rounded-xl">
        <div className="text-xs text-slate-500 uppercase font-bold mb-2">System Status</div>
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
          <span className="text-sm">Monitoring Active</span>
        </div>
      </div>
    </div>
  );
}
