import { create } from "zustand";

type Language = "tr" | "en";

interface LanguageState {
  lang: Language;
  setLang: (lang: Language) => void;
  t: (key: string) => string;
}

const translations = {
  en: {
    dashboard: "Dashboard",
    alerts: "Alerts",
    network: "Network Map",
    logs: "Live Logs",
    settings: "Settings",
    system_status: "System Status",
    active: "Monitoring Active",
    risk_score: "Risk Score",
    total_alerts: "Total Alerts",
    anydesk_id: "Your AnyDesk ID",
    info_title: "How it works?",
    info_desc: "This system monitors AnyDesk trace files and system processes to detect 'Pivot' attacks where an attacker uses a compromised machine to move laterally within your network.",
  },
  tr: {
    dashboard: "Panel",
    alerts: "Uyarılar",
    network: "Ağ Haritası",
    logs: "Canlı Loglar",
    settings: "Ayarlar",
    system_status: "Sistem Durumu",
    active: "İzleme Aktif",
    risk_score: "Risk Puanı",
    total_alerts: "Toplam Uyarı",
    anydesk_id: "AnyDesk Numaranız",
    info_title: "Nasıl Çalışır?",
    info_desc: "Bu sistem, saldırganın ele geçirilmiş bir makineyi kullanarak ağınızda yatayda hareket ettiği 'Pivot' saldırılarını tespit etmek için AnyDesk loglarını ve sistem süreçlerini izler.",
  }
};

export const useLanguage = create<LanguageState>((set, get) => ({
  lang: "en",
  setLang: (lang) => set({ lang }),
  t: (key) => {
    const lang = get().lang;
    return translations[lang][key as keyof typeof translations["en"]] || key;
  }
}));
