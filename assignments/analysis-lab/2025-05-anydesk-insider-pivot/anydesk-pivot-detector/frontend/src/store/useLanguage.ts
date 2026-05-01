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
    info_desc: "Monitoring AnyDesk activities to detect lateral movement.",
    capabilities_title: "Detection Academy",
    
    // Bypass
    bypass_title: "Firewall Bypass",
    bypass_what: "What is it? Attackers use AnyDesk's trusted port 443 to tunnel malicious traffic into your network, bypassing corporate firewalls.",
    bypass_how: "How we detect it: We analyze frame headers for 'Reverse-Tunneling' signatures and monitor port 443 for non-AnyDesk protocols wrapped inside.",

    // Persistence
    persistence_title: "Persistent Access",
    persistence_what: "What is it? Attackers set an 'Unattended Access' password to ensure they can return to the machine even after a reboot.",
    persistence_how: "How we detect it: Monitoring service.conf file integrity and registry keys for unauthorized password-based login configurations.",

    // Exfiltration
    exfiltration_title: "Data Exfiltration",
    exfiltration_what: "What is it? The illegal transfer of sensitive files from a corporate machine to an external attacker-controlled machine via AnyDesk.",
    exfiltration_how: "How we detect it: Correlating 'File Open' events with high network throughput and identifying sensitive file extensions being moved.",

    // Real-time
    realtime_title: "Real-time Monitor",
    realtime_what: "What is it? Constant oversight of all system events and AnyDesk trace files without any delay.",
    realtime_how: "How we detect it: Using Kernel-level File System Watchers to trigger the analyzer engine the millisecond a log entry is written.",

    // Scorer
    scorer_title: "Anomaly Scorer",
    scorer_what: "What is it? A brain for the system that calculates how 'dangerous' a specific sequence of events is.",
    scorer_how: "How we detect it: Using Heuristic algorithms that weigh multiple factors like time of day, IP reputation, and ACL status.",

    // Forensics
    forensics_title: "Forensics Engine",
    forensics_what: "What is it? Creating a digital crime scene report after an incident for legal and technical review.",
    forensics_how: "How we detect it: Aggregating all Session IDs and reconstructing the full command history of the remote operator.",

    // CLI
    cli_title: "CLI Engine",
    cli_what: "What is it? A powerful command-line interface for rapid head-less scanning and server integration.",
    cli_how: "How to use: Use 'cargo run -- scan' for instant forensic reports or 'monitor' for terminal-based live tracking.",

    // Config
    config_title: "Config Management",
    config_what: "What is it? The 'Brain Map' of the system that defines ACLs, working hours, and sensitive paths.",
    config_how: "How to use: Edit .env and default.toml files to dynamically change the system's defensive posture without recompiling.",

    // Reporting
    reporting_title: "Auto-Reporting",
    reporting_what: "What is it? Automatic generation of permanent forensic artifacts after a breach is detected.",
    reporting_how: "How to use: Check the 'reports/' directory for cryptographically signed JSON/HTML security reports.",
  },
  tr: {
    dashboard: "Panel",
    alerts: "Uyarılar",
    network: "Güvenlik Akademisi",
    logs: "Canlı Loglar",
    settings: "Ayarlar",
    system_status: "Sistem Durumu",
    active: "İzleme Aktif",
    risk_score: "Risk Puanı",
    total_alerts: "Toplam Uyarı",
    anydesk_id: "AnyDesk Numaranız",
    info_title: "Nasıl Çalışır?",
    info_desc: "AnyDesk aktivitelerini izleyerek yanal hareketleri tespit eder.",
    capabilities_title: "Tespit Akademisi",

    // Bypass
    bypass_title: "Güvenlik Atlatma",
    bypass_what: "Nedir? Saldırganlar, AnyDesk'in güvenilir 443 portunu kullanarak zararlı trafiği ağınıza sızdırır ve güvenlik duvarlarını devre dışı bırakır.",
    bypass_how: "Nasıl Tespit Edilir? 'Ters Tünelleme' imzaları için çerçeve başlıklarını analiz ederiz ve 443 portu üzerinden geçen AnyDesk dışı protokolleri izleriz.",

    // Persistence
    persistence_title: "Kalıcı Erişim",
    persistence_what: "Nedir? Saldırganlar, sistem yeniden başlatılsa bile geri gelebilmek için 'Gözetimsiz Erişim' şifresi koyarak sistemde kalıcılık sağlar.",
    persistence_how: "Nasıl Tespit Edilir? service.conf dosya bütünlüğünü ve kayıt defteri (registry) anahtarlarını yetkisiz şifre yapılandırmalarına karşı izleyerek.",

    // Exfiltration
    exfiltration_title: "Veri Sızdırma",
    exfiltration_what: "Nedir? Hassas dosyaların AnyDesk aracılığıyla kurumsal bir makineden saldırganın kontrolündeki harici bir makineye illegal transferidir.",
    exfiltration_how: "Nasıl Tespit Edilir? 'Dosya Açma' olaylarını yüksek ağ trafiğiyle ilişkilendirerek ve taşınan hassas dosya uzantılarını (örn: .env, .docx) belirleyerek.",

    // Real-time
    realtime_title: "Anlık İzleme",
    realtime_what: "Nedir? Tüm sistem olaylarının ve AnyDesk izleme dosyalarının gecikmesiz olarak sürekli denetlenmesidir.",
    realtime_how: "Nasıl Tespit Edilir? Kernel seviyesinde 'Dosya Sistemi İzleyicileri' kullanarak, bir log satırı yazıldığı anda analiz motorunu tetikleyerek.",

    // Scorer
    scorer_title: "Risk Puanlayıcı",
    scorer_what: "Nedir? Belirli bir olay dizisinin ne kadar 'tehlikeli' olduğunu hesaplayan sistemin beynidir.",
    scorer_how: "Nasıl Tespit Edilir? Günün saati, IP itibarı ve ACL durumu gibi çoklu faktörleri ağırlıklandıran 'Sezgisel' algoritmalar kullanarak.",

    // Forensics
    forensics_title: "Adli Bilişim",
    forensics_what: "Nedir? Bir olaydan sonra yasal ve teknik inceleme için dijital bir suç mahalli raporu oluşturulmasıdır.",
    forensics_how: "Nasıl Tespit Edilir? Tüm Oturum Kimliklerini (Session ID) birleştirerek ve uzak operatörün tüm komut geçmişini yeniden kurgulayarak.",

    // CLI
    cli_title: "CLI Motoru",
    cli_what: "Nedir? Hızlı taramalar ve sunucu entegrasyonları için kullanılan güçlü komut satırı arayüzüdür.",
    cli_how: "Nasıl Kullanılır? 'cargo run -- scan' komutuyla anında adli rapor alabilir veya 'monitor' ile terminalden canlı izleme yapabilirsiniz.",

    // Config
    config_title: "Yapılandırma Yönetimi",
    config_what: "Nedir? ACL'leri, çalışma saatlerini ve hassas yolları tanımlayan sistemin 'Beyin Haritası'dır.",
    config_how: "Nasıl Kullanılır? .env ve default.toml dosyalarını düzenleyerek sistemin savunma hattını yeniden derlemeden değiştirebilirsiniz.",

    // Reporting
    reporting_title: "Otomatik Raporlama",
    reporting_what: "Nedir? Bir sızıntı tespit edildikten sonra kalıcı adli kanıtların otomatik olarak oluşturulmasıdır.",
    reporting_how: "Nasıl Kullanılır? Kriptografik olarak imzalanmış güvenlik raporları için 'reports/' dizinini kontrol edin.",
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
