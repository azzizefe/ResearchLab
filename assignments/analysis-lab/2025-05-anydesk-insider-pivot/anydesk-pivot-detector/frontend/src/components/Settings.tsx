import { Save, Shield, Bell, HardDrive } from "lucide-react";
import PageHeader from "./PageHeader";

export default function Settings() {
  return (
    <div className="p-10 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <div className="flex justify-between items-start">
        <PageHeader 
          title="System Settings" 
          subtitle="Configure analysis thresholds and alert notifications." 
        />
        <button className="flex items-center gap-3 px-8 py-4 bg-gradient-to-r from-cyan-500 to-purple-600 rounded-2xl hover:scale-105 transition-all font-black text-sm text-white shadow-lg shadow-cyan-500/20">
          <Save size={20} /> 
          <span>SAVE CONFIGURATION</span>
        </button>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-8">
        <div className="glass-panel p-10 rounded-[2.5rem] space-y-8">
          <div className="flex items-center gap-4 mb-2">
            <Shield className="text-cyan-400" size={24} />
            <h3 className="text-xl font-bold text-white">Detection Rules</h3>
          </div>
          
          <div className="space-y-6">
            <div className="flex items-center justify-between p-6 bg-white/5 rounded-2xl border border-white/5">
              <div>
                <div className="font-bold text-white">Strict Pivot Detection</div>
                <div className="text-sm text-slate-500">Flag all incoming connections outside ACL</div>
              </div>
              <div className="w-12 h-6 bg-cyan-400 rounded-full relative">
                <div className="w-4 h-4 bg-white rounded-full absolute right-1 top-1"></div>
              </div>
            </div>

            <div className="flex items-center justify-between p-6 bg-white/5 rounded-2xl border border-white/5">
              <div>
                <div className="font-bold text-white">File Transfer Monitoring</div>
                <div className="text-sm text-slate-500">Track all file system interactions via AnyDesk</div>
              </div>
              <div className="w-12 h-6 bg-cyan-400 rounded-full relative">
                <div className="w-4 h-4 bg-white rounded-full absolute right-1 top-1"></div>
              </div>
            </div>
          </div>
        </div>

        <div className="glass-panel p-10 rounded-[2.5rem] space-y-8">
          <div className="flex items-center gap-4 mb-2">
            <Bell className="text-purple-400" size={24} />
            <h3 className="text-xl font-bold text-white">Notifications</h3>
          </div>

          <div className="space-y-6">
            <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
              <label className="block text-xs font-black text-slate-500 uppercase tracking-widest mb-3">Alert Email</label>
              <input 
                defaultValue="security-ops@internal.net"
                className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-cyan-400"
              />
            </div>

            <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
              <label className="block text-xs font-black text-slate-500 uppercase tracking-widest mb-3">Minimum Severity for Alerts</label>
              <select className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-cyan-400">
                <option>Medium</option>
                <option>High</option>
                <option>Critical Only</option>
              </select>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
