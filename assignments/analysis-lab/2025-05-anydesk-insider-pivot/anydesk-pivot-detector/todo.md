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
- [x] AnomalyScorer'dan dönen yüksek riskli alarmlar için e-posta, Slack veya Webhook bildirim (Alerting) altyapısını entegre edin.
- [x] Kurumsal yapılar için Syslog (RFC 5424) veya Splunk / QRadar entegrasyonlarını JSON Reporter üzerinden tamamlayın.
- [x] Ağ trafiğini (Network Monitor) takip eden modüller için kural motorunu (Rule Engine) güncelleyin.

## 10. Production (Canlı Ortam) Derlemesi
- [ ] Tüm testler tamamlandıktan sonra, son kullanıcı için optimize edilmiş bağımsız masaüstü (Tauri) çalıştırılabilir dosyasını derleyin:
  `npm run tauri build` veya `cargo tauri build`
- [ ] Arka plan (headless) versiyonunu sunucu ortamları için optimize edilmiş (release) modda derleyin:
  `cargo build --release`

## 11. 🔴 Tespit Edilen Kritik Hatalar (Bugs & Architectural Flaws)
- [x] **Mimari Hata (Kullanılmayan Modüller):** `src/main.rs` ve `src-tauri/src/lib.rs` içerisinde `RuleEngine` ve `AnomalyScorer` entegrasyonu tamamlandı.
- [x] **Veri Kaybı (Data Loss in Tokio Tasks):** `process_monitor` ve `network_monitor` artık `mpsc::channel` üzerinden ana event döngüsüne bağlandı.
- [x] **Kod Kalitesi ve Uyarılar (Clippy Warnings):** `cargo clippy --fix` çalıştırıldı, birçok uyarı giderildi. Kalanlar pedantic seviyesinde ancak `lib.rs` üzerinden susturuldu.
- [x] **Docker Elasticsearch RAM Limiti:** `docker-compose.yml` içinde ES için `-Xms1g -Xmx1g` olarak güncellendi. (Öncesi 512MB idi).

## 12. Neovim (.nvim.lua) Geliştirme Ortamı İyileştirmeleri
- [x] **Güvenli Eklenti Yükleme (pcall):** `.nvim.lua` içerisinde `pcall` sarmalaması yapıldı.
- [x] **Frontend (React/TS) Desteği:** `tsserver`/`vtsls`, `tailwindcss` ve `eslint` LSP konfigürasyonları eklendi.
- [x] **Otomatik Formatlama (Auto-formatting):** `BufWritePre` kancası ile otomatik formatlama (LSP format) eklendi.
- [x] **DAP Geliştirmeleri:** Temel yapı iyileştirildi.

## 13. 🐧 Cross-Platform Desteği (Linux/macOS Ağ İzleme)
- [x] **Linux Network Monitor:** `network_monitor.rs` içindeki PowerShell tabanlı bağlantı taramasını `/proc/net/tcp` veya `ss` komutuyla Linux'ta çalışacak şekilde implement edin.
- [x] **Linux DNS Cache:** `systemd-resolve --statistics` veya `/etc/resolv.conf` tabanlı DNS önbellek kontrolünü ekleyin.
- [x] **macOS Network Monitor:** `lsof -i` veya `nettop` komutlarıyla macOS ağ bağlantı izlemesini implement edin.
- [ ] **Platform Testleri:** CI/CD pipeline'ına Linux ve macOS build matrislerini ekleyerek cross-platform derleme doğrulaması yapın.

## 14. 🔐 SMTP Kimlik Doğrulama ve Güvenli Bildirimler
- [x] **Credentials Desteği:** `NotificationManager::send_email` fonksiyonuna `STARTTLS` ve kullanıcı adı/şifre (Credentials) desteği ekleyin.
- [x] **Ortam Değişkenleri:** SMTP kullanıcı adı ve şifresini `config/default.toml` veya `.env` dosyasından güvenli şekilde okuyun (`smtp_username`, `smtp_password`).
- [x] **Email Doğrulama:** `email_to` ve `from` alanlarının geçerli adresler olduğunu `parse()` öncesinde kontrol edin, `unwrap()` yerine hata yönetimi ekleyin.
- [x] **TLS Desteği:** Gmail, Outlook gibi servisler için `STARTTLS` veya doğrudan TLS bağlantı seçeneği ekleyin.

## 15. ⚡ Aktif Müdahale (Active Response / Incident Response)
- [x] **Process Kill:** Kritik seviyedeki alarmlar için AnyDesk sürecini otomatik sonlandırma (`taskkill` / `kill`) mekanizması ekleyin (konfigürasyondan açılıp kapatılabilir olmalı).
- [ ] **IP Engelleme:** Bilinen zararlı IP'lere bağlantı yapıldığında Windows Firewall veya `iptables` kuralı ekleyerek otomatik engelleme yapın.
- [ ] **Oturum Kapatma:** AnyDesk'in aktif oturumunu uzaktan sonlandırmak için AnyDesk CLI (`anydesk --remove-password`) entegrasyonu ekleyin.
- [ ] **Onay Mekanizması:** Aktif müdahale aksiyonlarının yanlışlıkla tetiklenmemesi için GUI/CLI üzerinden kullanıcı onayı (confirmation prompt) isteyin.

## 16. 🌍 GeoIP Lokalizasyon İyileştirmeleri
- [x] **MaxMind GeoLite2 Entegrasyonu:** Ücretsiz `ip-api.com` yerine lokal `MaxMind GeoLite2` veritabanını (`.mmdb`) kullanarak rate-limit sorunlarını ortadan kaldırın.
- [x] **Offline Mod:** İnternet bağlantısı olmayan ortamlarda bile çalışabilmesi için GeoIP veritabanını lokal olarak saklayın.
- [ ] **Otomatik Güncelleme:** GeoIP veritabanını belirli aralıklarla (haftalık) otomatik güncelleyen bir mekanizma ekleyin.

## 17. 📜 Log Rotasyonu ve Dayanıklılık (Log Rotation & Resilience)
- [x] **Log Rotasyonu Algılama:** `FileWatcher`'a AnyDesk'in `ad.trace` dosyasını arşivleyip yeni dosya oluşturduğu anı yakalayan bir mekanizma ekleyin (dosya adı değişikliği veya inode takibi).
- [ ] **Tampon Bellek (Buffer):** Dosya rotasyonu sırasında oluşabilecek veri kaybını önlemek için bir ring buffer veya write-ahead log (WAL) mekanizması implement edin.
- [x] **Checkpoint Sistemi:** Son okunan satır pozisyonunu diske yazarak, servis yeniden başladığında kaldığı yerden devam etmesini sağlayın.

## 18. 📈 Ölçeklenebilirlik ve Performans (Scalability)
- [ ] **Elasticsearch Cluster:** Tek node yerine en az 3 node'lu bir ES cluster yapılandırması için `docker-compose.prod.yml` dosyası oluşturun.
- [ ] **Index Lifecycle Management (ILM):** Eski alarmları otomatik arşivleyen veya silen bir Elasticsearch ILM politikası tanımlayın.
- [x] **Metrik Toplama:** Dedektörün CPU, bellek ve işlenen olay/saniye gibi performans metriklerini Prometheus formatında dışarı verin.
- [ ] **Benchmark Testleri:** Saniyede 1000+ olay yükü altında sistemin performansını ölçen benchmark testleri yazın (`criterion` crate).
