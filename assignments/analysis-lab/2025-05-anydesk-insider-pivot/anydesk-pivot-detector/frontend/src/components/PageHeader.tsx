import { ChevronLeft } from "lucide-react";
import { useNavigate, useLocation } from "react-router-dom";

interface PageHeaderProps {
  title: string;
  subtitle?: string;
}

export default function PageHeader({ title, subtitle }: PageHeaderProps) {
  const navigate = useNavigate();
  const location = useLocation();
  const isHome = location.pathname === "/";

  return (
    <div className="flex items-center justify-between mb-8">
      <div className="flex items-center gap-6">
        {!isHome && (
          <button 
            onClick={() => navigate(-1)}
            className="w-12 h-12 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center text-slate-400 hover:bg-white/10 hover:text-white transition-all group"
          >
            <ChevronLeft size={24} className="group-hover:-translate-x-0.5 transition-transform" />
          </button>
        )}
        <div>
          <h2 className="text-3xl font-black tracking-tight text-white glow-text">{title}</h2>
          {subtitle && <p className="text-slate-500 font-medium mt-1">{subtitle}</p>}
        </div>
      </div>
      
      <div className="flex items-center gap-4">
        <div className="px-4 py-2 rounded-xl bg-white/5 border border-white/5 text-xs font-mono text-slate-400">
          SESSION_ID: <span className="text-cyan-400">443-A29E-036B</span>
        </div>
      </div>
    </div>
  );
}
