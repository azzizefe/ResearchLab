# AnyDesk Pivot Detector - Kurulum ve Çalıştırma Adımları

Bu belge, AnyDesk Pivot Detector sisteminin Docker üzerinde sorunsuz bir şekilde ayağa kaldırılması ve eksiklerin giderilmesi için yapılması gereken adımları içerir.

## 1. Docker Engine'i Başlatma ve Doğrulama
- [x] Windows Başlat menüsünden **Docker Desktop** uygulamasını aratın ve başlatın.
- [x] Görev çubuğunun sağ alt köşesindeki sistem tepsisinde (system tray) Docker simgesinin belirdiğini ve motorun yeşil (Running) duruma geçtiğini bekleyin.
- [x] Terminalden `docker ps` komutunu çalıştırarak Docker'ın hatasız yanıt verdiğini teyit edin.


## 2. Çevresel Değişkenleri (.env) Kontrol Etme
- [x] `docker-compose.yml` ile ayn dizinde bir `.env` dosyası olduğundan emin olun (Yoksa oluşturun).
- [x] `.env` dosyası içerisinde aşağıdaki ayarların doğru host yollarını gösterdiğinden emin olun:
  - `APP_REPORT_OUTPUT_DIR=./reports`
  - `ANYDESK_TRACE_PATH=C:/Users/efe/AppData/Roaming/AnyDesk/ad.trace` *(Windows'ta kurulu olan AnyDesk trace dosyasının tam yolu olmalıdır, klasör yapılandırmanıza göre düzenleyin)*


## 3. Sistemi Ayağa Kaldırma (Docker Compose)
- [x] Terminalinizde `docker-compose.yml` dosyasının bulunduğu ana dizine gidin:
  `cd c:\Users\efe\Desktop\ResearchLab\ResearchLab\assignments\analysis-lab\2025-05-anydesk-insider-pivot\anydesk-pivot-detector`
- [x] İmajları derlemek ve arka planda tüm servisleri başlatmak için aşağıdaki komutu çalıştırın:
  `docker-compose up -d --build`
- [x] Konsoldan tüm imajların (detector, elasticsearch, kibana) başarıyla indirildiğini/derlendiğini doğrulayın.

## 4. Servislerin Durumunu ve Logları Kontrol Etme
- [x] Çalışan container'ların listesini görmek için:
  `docker-compose ps` 
  *(Tüm servislerin durumu 'Up' olmalıdır)*
- [x] `detector` servisinin loglarını anlık olarak izleyerek uygulamanın başarılı bir şekilde çalışıp AnyDesk loglarını okumaya başladığını test edin:
  `docker-compose logs -f detector`
- [x] `elasticsearch`'in sorunsuz başlatıldığını loglardan kontrol edin:
  `docker-compose logs elasticsearch`

## 5. Elastic Stack (Kibana) Arayüzüne Erişim
- [x] Servisler hazırlandıktan sonra (Elasticsearch'in başlaması biraz vakit alabilir), tarayıcınızı açın.
- [x] `http://localhost:5601` adresine giderek Kibana arayüzünün çalıştığını kontrol edin.
- [x] Kibana üzerinden gerekli indeks yapılandırmalarını yaparak AnyDesk datalarını görselleştirmeye başlayın.

## 6. Frontend ve Masaüstü (Tauri) Arayüzünü Başlatma
- [x] Docker bağımsız olarak yerel masaüstü arayüzünü (GUI) başlatmak için terminalden `frontend` klasörüne gidin:
  `cd c:\Users\efe\Desktop\ResearchLab\ResearchLab\assignments\analysis-lab\2025-05-anydesk-insider-pivot\anydesk-pivot-detector\frontend`
- [x] Gerekli paketleri kurun (npm veya bun ile):
  `pnpm install`
- [x] Tauri ile masaüstü uygulamasını geliştirici modunda başlatın:
  `npx @tauri-apps/cli dev`

## 7. Eksik Olan Modüller (SOC Dashboard & UI)
- [x] 6 tespit modülünün (Process, Network, Config vb.) tamamını tek bir sekme tabanlı etkileşimli Dashboard'da birleştirin.
- [x] Her bir güvenlik modülü için eğitici (educational) SOC içeriklerini ekleyin.
- [x] Canlı log akışı için FileWatcher entegrasyonunu Tauri events üzerinden UI tarafına bağlayın.

## 8. Güvenlik Denetimi (Audit) ve Testler
- [x] **Hassas Veri Temizliği (PII Audit):** Proje dizininde (özellikle `test_ad.trace` veya diğer log dosyalarında) kişisel IP adresleri, AnyDesk ID'leri veya hassas bilgilerin kalmadığından emin olun.
- [x] Rust tarafındaki (backend) eksik birim testlerini (Unit Tests) ve entegrasyon testlerini çalıştırın (`cargo test`).
- [x] Frontend tarafındaki test altyapısını (Vitest vb.) çalıştırın ve eksik testleri tamamlayın (`npm run test`).

## 9. SIEM Entegrasyonu ve Bildirim Mekanizmaları
- [ ] AnomalyScorer'dan dönen yüksek riskli alarmlar için e-posta, Slack veya Webhook bildirim (Alerting) altyapısını entegre edin.
- [ ] Kurumsal yapılar için Syslog (RFC 5424) veya Splunk / QRadar entegrasyonlarını JSON Reporter üzerinden tamamlayın.
- [ ] Ağ trafiğini (Network Monitor) takip eden modüller için kural motorunu (Rule Engine) güncelleyin.

## 10. Production (Canlı Ortam) Derlemesi
- [ ] Tüm testler tamamlandıktan sonra, son kullanıcı için optimize edilmiş bağımsız masaüstü (Tauri) çalıştırılabilir dosyasını derleyin:
  `npm run tauri build` veya `cargo tauri build`
- [ ] Arka plan (headless) versiyonunu sunucu ortamları için optimize edilmiş (release) modda derleyin:
  `cargo build --release`

## 11. 🔴 Tespit Edilen Kritik Hatalar (Bugs & Architectural Flaws)
- [x] **Mimari Hata (Kullanılmayan Modüller):** `src/main.rs` ve `src-tauri/src/lib.rs` içerisinde `RuleEngine` ve `AnomalyScorer` entegrasyonu tamamlandı.
- [x] **Veri Kaybı (Data Loss in Tokio Tasks):** `process_monitor` ve `network_monitor` artık `mpsc::channel` üzerinden ana event döngüsüne bağlandı.
- [ ] **Kod Kalitesi ve Uyarılar (Clippy Warnings):** `cargo clippy --fix` çalıştırıldı, birçok uyarı giderildi. Kalanlar pedantic seviyesinde.
- [x] **Docker Elasticsearch RAM Limiti:** `docker-compose.yml` içinde ES için `-Xms1g -Xmx1g` olarak güncellendi. (Öncesi 512MB idi).

## 12. Neovim (.nvim.lua) Geliştirme Ortamı İyileştirmeleri
- [x] **Güvenli Eklenti Yükleme (pcall):** `.nvim.lua` içerisinde `pcall` sarmalaması yapıldı.
- [x] **Frontend (React/TS) Desteği:** `tsserver`/`vtsls`, `tailwindcss` ve `eslint` LSP konfigürasyonları eklendi.
- [x] **Otomatik Formatlama (Auto-formatting):** `BufWritePre` kancası ile otomatik formatlama (LSP format) eklendi.
- [x] **DAP Geliştirmeleri:** Temel yapı iyileştirildi.
