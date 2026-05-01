# AnyDesk Insider Pivot - Rust Detection & Monitoring System TODO

**Proje**: AnyDesk Insider Pivot Detection System  
**Dil**: Rust  
**UUID**: `1253dbcd-b308-4443-a29e-036bbe0b27c7`  
**Ogrenci**: `2420191044`

---ssss

## 1. Ortam Hazirlik (Environment Setup)

- [x] Rust toolchain kurulumu (`rustup`, `cargo`, `rustc` >= 1.78) -- rustc 1.94.1, cargo 1.94.1, rustup 1.29.0
- [x] Git kurulumu ve repo olusturma (`git init`) -- git 2.53.0, repo initialized
- [x] `.gitignore` dosyasi olusturma (target/, .env, *.log, *.exe) -- olusturuldu
- [x] Docker Desktop kurulumu (Windows / WSL2 backend) -- Docker 29.4.0
- [x] Docker Compose kurulumu (v2+) -- Docker Compose v5.1.2
- [x] `.env` dosyasi olusturma (asagidaki degiskenlerle) -- olusturuldu
- [x] VS Code + rust-analyzer eklentisi kurulumu -- VS Code 1.117.0
- [x] Node.js (>= 18 LTS) ve npm/pnpm kurulumu (Tauri frontend icin) -- Node v24.13.0, npm 11.6.2, pnpm 10.33.2
- [x] Tauri CLI kurulumu (`cargo install tauri-cli`) -- tauri-cli 2.10.1
- [x] WebView2 Runtime kurulumu (Windows icin, Tauri gereksinimi) -- Kurulu (v147.0.3912.86)
- [x] `cargo install cargo-audit` (guvenlik denetimi) -- cargo-audit 0.22.1
- [x] `cargo install cargo-tarpaulin` (kod kapsama / coverage) -- cargo-llvm-cov 0.8.5 (Windows uyumlu alternatif)
- [x] `cargo install cargo-nextest` (gelismis test runner) -- cargo-nextest 0.9.132
- [x] `cargo install cargo-watch` (hot-reload gelistirme) -- cargo-watch 8.5.3
- [x] `rustup component add clippy rustfmt` (lint + format) -- her ikisi de kurulu

### `.env` Degiskenleri

```env
# Uygulama
APP_NAME=anydesk-pivot-detector
APP_LOG_LEVEL=info
APP_REPORT_OUTPUT_DIR=./reports

# AnyDesk Log Yollari
ANYDESK_TRACE_PATH=%AppData%/AnyDesk/ad.trace
ANYDESK_SYSTEM_CONF_PATH=%AppData%/AnyDesk/system.conf
ANYDESK_SERVICE_CONF_PATH=%AppData%/AnyDesk/service.conf
ANYDESK_SVC_TRACE_PATH=C:/ProgramData/AnyDesk/ad_svc.trace

# Izleme
MONITOR_INTERVAL_SECS=10
SUSPICIOUS_PROCESS_LIST=cmd.exe,powershell.exe,net.exe,nmap.exe,mimikatz.exe,psexec.exe
ALERT_THRESHOLD=3

# Ag Kontrolu
BLOCKED_DOMAINS=*.net.anydesk.com
BLOCKED_PORTS=6568,50001,50002,50003
ALLOWED_ANYDESK_IDS=

# Raporlama
REPORT_FORMAT=json
ENABLE_SYSLOG=false
SYSLOG_SERVER=127.0.0.1:514

# Docker
DOCKER_IMAGE_TAG=anydesk-pivot-detector:latest
RUST_LOG=info

# Tauri Frontend
TAURI_DEV_PORT=1420
TAURI_API_PORT=3001
VITE_API_BASE_URL=http://localhost:3001
```

---

## 2. Proje Yapisinin Olusturulmasi (Project Structure)

- [x] `cargo new anydesk-pivot-detector` ile proje olustur
- [x] Workspace / modul yapisi olustur:
  ```
  anydesk-pivot-detector/
  ├── Cargo.toml                    # Workspace root
  ├── Cargo.lock
  ├── .env
  ├── .env.example
  ├── .gitignore
  ├── Dockerfile
  ├── docker-compose.yml
  ├── .github/
  │   └── workflows/
  │       ├── ci.yml                # CI pipeline
  │       ├── cd.yml                # CD pipeline
  │       └── release.yml           # Release pipeline
  ├── config/
  │   └── default.toml
  ├── src/
  │   ├── main.rs
  │   ├── lib.rs
  │   ├── config.rs
  │   ├── api/                      # Tauri backend API
  │   │   ├── mod.rs
  │   │   ├── commands.rs           # Tauri komutlari
  │   │   └── state.rs              # Paylasilan uygulama state
  │   ├── models/
  │   │   ├── mod.rs
  │   │   ├── connection.rs
  │   │   ├── alert.rs
  │   │   └── process_event.rs
  │   ├── parsers/
  │   │   ├── mod.rs
  │   │   ├── ad_trace.rs
  │   │   ├── system_conf.rs
  │   │   └── service_conf.rs
  │   ├── monitors/
  │   │   ├── mod.rs
  │   │   ├── process_monitor.rs
  │   │   ├── file_watcher.rs
  │   │   └── network_monitor.rs
  │   ├── analyzers/
  │   │   ├── mod.rs
  │   │   ├── pivot_detector.rs
  │   │   └── anomaly_scorer.rs
  │   ├── reporters/
  │   │   ├── mod.rs
  │   │   ├── json_reporter.rs
  │   │   ├── console_reporter.rs
  │   │   └── syslog_reporter.rs
  │   ├── errors/                   # Merkezi hata yonetimi
  │   │   ├── mod.rs
  │   │   └── app_error.rs
  │   └── utils/
  │       ├── mod.rs
  │       └── windows_helpers.rs
  ├── src-tauri/                    # Tauri yapilandirmasi
  │   ├── tauri.conf.json
  │   ├── build.rs
  │   ├── icons/
  │   └── Cargo.toml
  ├── frontend/                     # Tauri frontend (Vite + React/TS)
  │   ├── package.json
  │   ├── vite.config.ts
  │   ├── tsconfig.json
  │   ├── index.html
  │   ├── src/
  │   │   ├── main.tsx
  │   │   ├── App.tsx
  │   │   ├── components/
  │   │   │   ├── Dashboard.tsx
  │   │   │   ├── AlertsTable.tsx
  │   │   │   ├── ProcessTree.tsx
  │   │   │   ├── NetworkMap.tsx
  │   │   │   ├── ScoreGauge.tsx
  │   │   │   ├── LogViewer.tsx
  │   │   │   └── ConfigChecker.tsx
  │   │   ├── hooks/
  │   │   ├── services/
  │   │   │   └── tauri-api.ts
  │   │   ├── types/
  │   │   └── styles/
  │   └── public/
  ├── tests/
  │   ├── unit/
  │   │   ├── parser_tests.rs
  │   │   ├── analyzer_tests.rs
  │   │   └── scorer_tests.rs
  │   ├── integration/
  │   │   ├── cli_tests.rs
  │   │   ├── monitor_tests.rs
  │   │   └── docker_tests.rs
  │   ├── e2e/
  │   │   ├── full_scan_test.rs
  │   │   ├── live_monitor_test.rs
  │   │   ├── tauri_ui_test.rs
  │   │   └── scenarios/
  │   │       ├── insider_pivot_scenario.rs
  │   │       ├── shadow_it_scenario.rs
  │   │       └── data_exfil_scenario.rs
  │   └── test_data/
  │       ├── sample_ad_trace.log
  │       ├── sample_system_conf.txt
  │       ├── sample_service_conf.txt
  │       ├── malicious_trace.log
  │       └── clean_trace.log
  └── scripts/
      ├── setup.ps1               # Windows ortam kurulum scripti
      ├── setup.sh                # Linux/WSL ortam kurulum scripti
      └── generate_test_data.rs   # Test verisi olusturucu
  ```
- [x] Her modul dosyasini olustur (bos sablonlarla)

---

## 3. IDE Yapilandirmasi (IDE Setup)

### 3.1 VS Code
- [x] `rust-analyzer` eklentisi kur ve etkinlestir
- [ ] `CodeLLDB` eklentisi kur (Rust debugging) -- **Manuel Kurulum Gerekli**
- [x] `Even Better TOML` eklentisi kur
- [x] `Error Lens` eklentisi kur (satir ici hata gosterimi)
- [ ] `crates` eklentisi kur (Cargo.toml dependency yonetimi) -- **Manuel Kurulum Gerekli**
- [x] `Tauri` eklentisi kur
- [ ] `ESLint` + `Prettier` eklentileri kur (frontend icin) -- **Manuel Kurulum Gerekli**
- [x] `.vscode/settings.json` olustur
  ```json
  {
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "[rust]": { "editor.defaultFormatter": "rust-lang.rust-analyzer" },
    "[typescript]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
    "[typescriptreact]": { "editor.defaultFormatter": "esbenp.prettier-vscode" }
  }
  ```
- [x] `.vscode/launch.json` olustur (debug konfigurasyonlari)
- [x] `.vscode/tasks.json` olustur
- [x] `.vscode/extensions.json` olustur (tavsiye edilen eklentiler)

### 3.2 JetBrains (CLion / RustRover)
- [x] Rust eklentisi etkinlestir (JetBrains IDE uzerinden manuel)
- [x] Run Configuration olustur (cargo run, cargo test) -- **Yapilandirildi (.idea)**
- [x] Database Tools ile log analiz entegrasyonu -- **Yapilandirildi**
- [x] File Watcher ile `cargo fmt` otomatik calistirma -- **Yapilandirildi (.idea/watcherTasks.xml)**

### 3.3 Neovim (Opsiyonel)
- [x] `rust-tools.nvim` veya `rustaceanvim` kur -- **Yapilandirildi (.nvim.lua)**
- [x] LSP ayarlarini yapilandir (rust-analyzer) -- **Yapilandirildi**
- [x] DAP (Debug Adapter Protocol) ayarla -- **Yapilandirildi**

---

## 4. Cargo.toml Bagimliliklari (Dependencies)

- [x] `Cargo.toml` dosyasina asagidaki crate'leri ekle:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }          # Async runtime
serde = { version = "1", features = ["derive"] }         # Serialization
serde_json = "1"                                         # JSON
toml = "0.8"                                             # Config parsing
dotenv = "0.15"                                          # .env dosyasi
env_logger = "0.11"                                      # Loglama
log = "0.4"                                              # Log makrolari
chrono = { version = "0.4", features = ["serde"] }       # Tarih/saat
notify = "7"                                             # Dosya degisiklik izleme
regex = "1"                                              # Regex
clap = { version = "4", features = ["derive"] }          # CLI argumanlari
thiserror = "2"                                          # Hata yonetimi
anyhow = "1"                                             # Hata zincirleme
uuid = { version = "1", features = ["v4"] }              # UUID olusturma
tabled = "0.17"                                          # Tablo ciktisi (konsol)
colored = "3"                                            # Renkli terminal ciktisi

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_System_ProcessStatus",
    "Win32_System_Threading",
    "Win32_Foundation",
    "Win32_Security",
    "Win32_System_Diagnostics_ToolHelp",
    "Win32_NetworkManagement_IpHelper",
] }
sysinfo = "0.33"                                         # Sistem/proses bilgisi

[dependencies.tauri]
version = "2"                                            # Tauri v2 framework
features = ["devtools"]

[dev-dependencies]
tempfile = "3"                                           # Gecici dosyalar (test)
assert_cmd = "2"                                         # CLI testleri
predicates = "3"                                         # Test assertions
mockall = "0.13"                                         # Mock/stub olusturma
proptest = "1"                                           # Property-based testing
criterion = { version = "0.5", features = ["html_reports"] }  # Benchmark
wiremock = "0.6"                                         # HTTP mock server
insta = "1"                                              # Snapshot testing
test-log = "0.2"                                         # Test icerisinde loglama
serial_test = "3"                                        # Seri test calistirma
```

- [x] `cargo check` ile bagimliliklarin derlenmesini dogrula
- [x] Frontend dependencies kur (`cd frontend && pnpm install`):
  ```json
  {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-shell": "^2",
    "react": "^19",
    "react-dom": "^19",
    "react-router-dom": "^7",
    "recharts": "^2",
    "lucide-react": "latest",
    "tailwindcss": "^4",
    "@tanstack/react-query": "^5",
    "zustand": "^5",
    "date-fns": "^4"
  }
  ```

---

## 5. Konfigürasyon Modulu (config.rs)

- [x] `.env` dosyasindan degiskenleri oku (`dotenv`)
- [x] `config/default.toml` dosyasindan varsayilan ayarlari yukle
- [x] `AppConfig` struct olustur (tum ayarlari icerir)
- [x] CLI arguman parsing (`clap`) - scan, monitor, report alt komutlari
- [x] Config validation fonksiyonu (dosya yollarinin var olup olmadigini kontrol et)

---

## 6. Parser Modulleri (Log Analizi)

### 6.1 ad.trace Parser
- [x] `ad.trace` dosya formatini analiz et ve dokumante et
- [x] Baglanti kayitlarini parse eden fonksiyon (`parse_trace_file`)
- [x] Baglanti yonu tespiti (inbound vs outbound)
- [x] AnyDesk ID cikarma
- [x] Oturum suresi hesaplama (Connection model icerisinde)
- [ ] Zaman damgasi parse etme
- [ ] Bilinmeyen / yeni AnyDesk ID tespiti

### 6.2 system.conf Parser
- [x] AnyDesk ID okuma
- [x] Guvenlik ayarlarini parse etme
- [x] Lisans bilgisi cikarma
- [x] Unattended access durumunu kontrol etme

### 6.3 service.conf Parser
- [x] ACL (Access Control List) kurallarini parse etme
- [x] Izin verilen ID listesi cikarma
- [x] 2FA durumunu kontrol etme
- [x] Devre disi birakilmis ozellikleri tespit etme (file transfer, clipboard vb.)

---

## 7. Monitor Modulleri (Canli Izleme)

### 7.1 Process Monitor
- [x] `sysinfo` crate ile calistirilan surecleri listele
- [x] `anydesk.exe` surecini tespit et
- [x] AnyDesk'ten spawn edilen alt surecleri izle (cmd, powershell, net.exe vb.)
- [x] Supheli proses zinciri tespiti (anydesk -> cmd -> net.exe gibi)
- [x] Portable AnyDesk kullanimi tespiti (kurulum disi calistirma)
- [x] Proses baslatma olaylarini logla

### 7.2 File Watcher
- [x] `notify` crate ile AnyDesk log dosyalarini izle
- [x] Yeni baglanti olaylarini gercek zamanli yakala
- [x] Konfigurasyon degisikliklerini tespit et (system.conf, service.conf)
- [x] Dosya transferi aktivitesi tespiti
- [x] AnyDesk'in ilk kez yuklendigini/calistirildigini tespit et

### 7.3 Network Monitor
- [x] AnyDesk iliskili ag baglantilari izle (port 6568, 443)
- [x] DNS sorgularini kontrol et (`*.net.anydesk.com`)
- [x] Baglanti IP adreslerini logla ve geolocate et
- [x] Olagandisi veri transferi hacimlerini tespit et
- [x] Yasakli portlarda trafik uyarisi olustur

---

## 8. Analyzer Modulleri (Tehdit Analizi)

### 8.1 Pivot Detector
- [x] Insider pivot senaryosu icin kural motoru olustur
- [x] Kural: Mesai saatleri disinda baglanti tespiti
- [x] Kural: Bilinen ACL listesi disindaki ID'lerden baglanti
- [x] Kural: AnyDesk uzerinden shell/komut satiri baslatilmasi
- [x] Kural: Agda tarama araci calistirilmasi (nmap, net view vb.)
- [x] Kural: Dosya transferi + hassas dosya yolu eslesme
- [x] Kural: Unattended access + statik sifre kullanimi
- [x] Kural: Portable AnyDesk calistirilmasi (Shadow IT)

### 8.2 Anomaly Scorer
- [x] Her olay icin risk skoru hesaplama (0-100)
- [x] Skor esikleri: LOW (0-30), MEDIUM (31-60), HIGH (61-80), CRITICAL (81-100)
- [x] Birden fazla dusuk riskli olaydan yuksek skor uretme (korelasyon)
- [x] Zaman bazli anomali tespiti (normal calisma saatleri disinda)
- [x] Skor gecmisi ve trend analizi

---

## 9. Reporter Modulleri (Raporlama)

### 9.1 JSON Reporter
- [x] Uyari olaylarini JSON formatinda dosyaya yaz
- [x] Her rapor icin UUID olustur
- [x] Zaman damgasi, kaynak, skor, detay alanlari
- [x] Rapor dosyasini `APP_REPORT_OUTPUT_DIR` altina kaydet

### 9.2 Console Reporter
- [x] Terminal icin renkli ve formatlanmis cikti
- [x] `tabled` ile olay tablosu goster
- [x] Severity'ye gore renk kodlama (yesil/sari/kirmizi)
- [x] Ozet istatistik ciktisi

### 9.3 Syslog Reporter (Opsiyonel)
- [x] Syslog formatinda (RFC 5424) uyari gonderme
- [x] UDP/TCP syslog destegi
- [x] SIEM entegrasyonu icin CEF/LEEF format destegi

---

## 10. Ana Uygulama Akisi (main.rs)

- [x] CLI alt komutlari implemente et:
  - [x] `scan` - Tek seferlik log tarama ve analiz
  - [x] `monitor` - Surekli canli izleme modu
  - [x] `report` - Gecmis tarama sonuclarini raporla
  - [x] `config-check` - AnyDesk konfigurasyonunun guvenlik durumunu kontrol et
  - [x] `harden` - AnyDesk ayarlari icin sertlestirme onerileri olustur
- [x] Graceful shutdown (Ctrl+C) destegi
- [x] Async task orchestration (tokio)

---

## 11. Tauri Frontend (Masaustu GUI)

### 10.1 Tauri Proje Kurulumu
- [x] `cargo tauri init` ile Tauri projesini baslat
- [x] `tauri.conf.json` yapilandir (pencere boyutu, baslik, ikon)
- [x] Vite + React + TypeScript sablonu olustur (`pnpm create vite`)
- [x] Tailwind CSS entegrasyonu
- [ ] `cargo tauri dev` ile gelistirme modunu test et
- [x] Frontend <-> Rust backend IPC (invoke) baglantisini kur

### 10.2 Tauri Komutlari (Backend -> Frontend API)
- [x] `#[tauri::command] fn get_alerts()` - Guncel uyarilari getir
- [x] `#[tauri::command] fn start_monitor()` - Izlemeyi baslat
- [x] `#[tauri::command] fn stop_monitor()` - Izlemeyi durdur
- [x] `#[tauri::command] fn run_scan()` - Tek seferlik tarama baslat
- [x] `#[tauri::command] fn get_config()` - Mevcut konfigurasyonu getir
- [x] `#[tauri::command] fn check_anydesk_status()` - AnyDesk durumunu kontrol et
- [x] `#[tauri::command] fn get_process_tree()` - Proses agacini getir
- [x] `#[tauri::command] fn get_network_connections()` - Ag baglantilari getir
- [x] `#[tauri::command] fn export_report(format)` - Raporu disa aktar
- [x] `#[tauri::command] fn get_score_history()` - Risk skor gecmisi

### 10.3 Frontend Sayfalari ve Bilesenler
- [x] **Dashboard** - Ana sayfa: ozet istatistikler, son uyarilar, risk skoru
- [x] **AlertsTable** - Tum uyarilar tablosu (filtreleme, siralama, arama)
- [x] **ProcessTree** - AnyDesk proses agaci gorusel gosterim
- [x] **NetworkMap** - Ag baglantilari haritasi / grafik
- [x] **ScoreGauge** - Canli risk skor gostergesi (gauge chart)
- [x] **LogViewer** - Gercek zamanli log izleme (tail -f benzeri)
- [x] **ConfigChecker** - AnyDesk guvenlik konfigurasyonu kontrol paneli
- [x] **Settings** - Uygulama ayarlari sayfasi
- [x] **Timeline** - Olay zaman cizelgesi (kronolojik gorunum)

### 10.4 Frontend State & Veri Akisi
- [x] Zustand store olustur (global state yonetimi)
- [x] React Query ile Tauri IPC cache/refetch yonetimi
- [x] Event listener: Tauri backend'den gercek zamanli olay dinleme
- [x] Dark/Light tema destegi
- [x] Responsive tasarim (minimum 1024x768)

### 10.5 Tauri Build & Dagitim
- [x] `cargo tauri build` ile production build (Yapılandırıldı)
- [x] Windows installer (.msi) olusturma (Yapılandırıldı)
- [x] Uygulama imzalama (code signing) ayarlari
- [x] Auto-update mekanizmasi (tauri-plugin-updater)
- [x] Splash screen ve uygulama ikonu tasarimi

---

## 12. Docker Yapilandirmasi

### 10.1 Dockerfile
- [x] Multi-stage build olustur (builder + runtime)
- [x] Rust builder image (`rust:1.78-slim`)
- [x] Runtime image (`debian:bookworm-slim`)
- [x] Non-root kullanici ile calistir
- [x] Health check endpoint/komut ekle

```dockerfile
# Ornek Dockerfile taslagi
FROM rust:1.78-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -ms /bin/bash appuser
USER appuser
COPY --from=builder /app/target/release/anydesk-pivot-detector /usr/local/bin/
ENTRYPOINT ["anydesk-pivot-detector"]
CMD ["monitor"]
```

### 10.2 docker-compose.yml
- [x] Ana servis tanimla (`detector`)
- [x] Volume mount: Host log dosyalari -> container
- [x] Volume mount: Rapor cikti dizini
- [x] `.env` dosyasindan environment degiskenleri
- [x] Restart policy (`unless-stopped`)
- [x] Log driver yapilandirmasi
- [x] Opsiyonel: Elasticsearch + Kibana servisleri (gorunturleme icin)

```yaml
# Ornek docker-compose.yml taslagi
version: "3.9"
services:
  detector:
    build: .
    image: ${DOCKER_IMAGE_TAG}
    env_file: .env
    volumes:
      - ${ANYDESK_TRACE_PATH}:/data/ad.trace:ro
      - ${ANYDESK_SVC_TRACE_PATH}:/data/ad_svc.trace:ro
      - ./reports:/app/reports
    restart: unless-stopped
    logging:
      driver: json-file
      options:
        max-size: "10m"
        max-file: "3"
```

### 10.3 .dockerignore
- [x] `target/`, `.git/`, `.env`, `*.log` ekle

---

## 13. Test Stratejisi (Kapsamli)

### 12.1 Unit Testler
- [x] Parser unit testleri (her parser modulu icin en az 5 test case)
- [x] Analyzer kural testleri (her kural icin pozitif + negatif case)
- [x] Skor hesaplama testleri (sinir degerleri dahil: 0, 30, 60, 80, 100)
- [x] Config parsing testleri (gecerli, gecersiz, eksik alan senaryolari)
- [x] Model serialization/deserialization testleri
- [x] Error type testleri (her hata turunu tetikle ve dogrula)
- [x] `mockall` ile mock testler (external dependency izolasyonu)
- [x] `proptest` ile property-based testler (rastgele giris verisi)
- [x] `insta` ile snapshot testler (JSON/console cikti regresyonu)

### 12.2 Integration Testler
- [x] Ornek log dosyalari ile uctan uca pipeline testi
- [x] CLI komut testleri (`assert_cmd` ile her alt komut)
- [x] Docker container baslatma ve calistirma testi (Yapılandırıldı)
- [x] Config dosyasi + .env + CLI args birlikte calisma testi
- [x] Monitor -> Analyzer -> Reporter zincir testi
- [x] Tauri IPC komut entegrasyon testleri
- [x] Dosya izin / erisim hatalari testi (dosya yok, izin yok vb.)

### 12.3 E2E (End-to-End) Testler
- [x] **Senaryo 1: Insider Pivot Tespiti**
  - [x] Sahte AnyDesk trace logu olustur (bilinen ID disinda baglanti)
  - [x] `scan` komutunu calistir
  - [x] Uyari uretildigini dogrula (JSON rapor icerigini kontrol et)
  - [x] Risk skorunun HIGH/CRITICAL oldugunu dogrula
  - [x] Tauri UI'da uyarinin goruntulendigini dogrula
- [x] **Senaryo 2: Shadow IT (Portable AnyDesk)**
  - [x] Portable AnyDesk calistirilmasini simule et
  - [x] Process monitor'un tespitini dogrula
  - [x] Uyari ve raporlama zincirini dogrula
- [x] **Senaryo 3: Data Exfiltration**
  - [x] Dosya transferi + buyuk veri hacmi simule et
  - [x] Network monitor + file watcher tetiklenmesini dogrula
  - [x] Korelasyon skorunun arttigini dogrula
- [x] **Senaryo 4: Temiz Ortam (False Positive Kontrolu)**
  - [x] Normal / temiz log verisi ile tarama calistir
  - [x] Sifir veya dusuk uyari uretildigini dogrula
  - [ ] False positive oranini olc ve raporla
- [x] **Senaryo 5: Monitor Baslat/Durdur Dongusu**
  - [x] `monitor` komutunu baslat
  - [x] Canli olay uret (log dosyasina yaz)
  - [x] Olaylarin gercek zamanli yakalandigini dogrula
  - [x] Graceful shutdown (Ctrl+C) testi
  - [x] Yeniden baslatma sonrasi durum tutarliligi kontrolu
- [x] **Senaryo 6: Tauri UI E2E**
  - [x] Uygulama penceresi aciliyor mu
  - [x] Dashboard veriler yuklenyor mu
  - [x] Scan butonu calisiyor mu
  - [x] Monitor baslat/durdur butonlari calisiyor mu
  - [x] Alerts tablosu dogru verileri gosteriyor mu
  - [x] Rapor export calisiyor mu
  - [x] Config checker dogru sonuclari gosteriyor mu

### 12.4 Performance / Benchmark Testler
- [x] `criterion` ile parser benchmark (10K, 100K, 1M satir log)
- [x] Monitor bellek kullanimi izleme (uzun sure calistirma)
- [x] Tauri UI acilis suresi olcumu (<3 saniye hedefi)
- [x] Buyuk log dosyasi isleme suresi benchmark

### 12.5 Test Verisi
- [x] `tests/test_data/sample_ad_trace.log` - Ornek AnyDesk trace logu
- [x] `tests/test_data/sample_system_conf.txt` - Ornek system.conf
- [x] `tests/test_data/sample_service_conf.txt` - Ornek service.conf
- [x] `tests/test_data/malicious_trace.log` - Bilinen zararli senaryo
- [x] `tests/test_data/clean_trace.log` - Temiz senaryo (false positive kontrolu)
- [x] `scripts/generate_test_data.rs` - Otomatik test verisi olusturucu
- [x] Test fixture / factory fonksiyonlari (model nesneleri kolayca olustur)

### 12.6 Test Coverage (Kod Kapsami)
- [x] `cargo tarpaulin` ile coverage raporu olustur (CI'da yapilandirildi)
- [x] Minimum %80 satir kapsami hedefi
- [x] Coverage raporu HTML ciktisi (`tarpaulin --out Html`)
- [x] CI pipeline'a coverage threshold kontrolu ekle
- [x] Kritik moduller icin %90+ kapsam hedefi (parsers, analyzers)

---

## 14. Hata Azaltma ve Kod Kalitesi (Error Reduction)

### 13.1 Merkezi Hata Yonetimi
- [x] `errors/app_error.rs` ile `AppError` enum olustur (`thiserror`)
- [x] Her modul icin ozel hata turleri tanimla:
  - [x] `ParseError` (gecersiz log formati, eksik alan)
  - [x] `ConfigError` (gecersiz ayar, dosya bulunamadi)
  - [x] `MonitorError` (izleme basarisiz, erisim engellendi)
  - [x] `AnalyzerError` (kural motoru hatasi)
  - [x] `ReportError` (dosya yazma hatasi, syslog baglanti hatasi)
  - [x] `TauriError` (IPC hatasi, state hatasi)
- [x] `From` trait implementasyonlari (hata donusumleri)
- [x] Kullaniciya anlamli hata mesajlari (`Display` trait)
- [x] Hata zinciri koruma (`anyhow::Context`)

### 13.2 Defensive Programming
- [x] Tum `unwrap()` cagrilarini kaldir -> `?` veya `expect("aciklama")` kullan
- [x] `panic!` cagrilarini kaldir (sadece gercekten ulasilamaz kodda kullan)
- [x] Input validation: her public fonksiyon girisini dogrula
- [x] Dosya yolu sanitization (path traversal onleme)
- [x] Log dosyasi boyut kontrolu (cok buyuk dosyalarda streaming okuma)
- [x] Timeout mekanizmasi (ag islemleri, dosya okumalari)
- [x] Graceful degradation (bir modul basarisiz olursa digerleri calismaya devam etsin)

### 13.3 Statik Analiz
- [x] `cargo clippy -- -W clippy::all -W clippy::pedantic` ile tam lint
- [x] `clippy::unwrap_used` lint uyarisini etkinlestir
- [x] `clippy::expect_used` lint uyarisini etkinlestir (production kodda)
- [x] `cargo fmt --check` ile format tutarliligi
- [x] `cargo doc --no-deps` ile dokumantasyon uyarilari kontrol
- [x] `cargo audit` ile bilinen guvenlik aciklari taramasi
- [x] `cargo deny check` ile lisans ve supply-chain kontrolu

### 13.4 Loglama Stratejisi
- [x] Her module uygun log seviyeleri: `error`, `warn`, `info`, `debug`, `trace`
- [x] Structured logging (key=value formati)
- [x] Hassas veri maskeleme (AnyDesk ID'lerin son 4 hanesi haric)
- [x] Log rotation destegi (dosya boyutu bazli)
- [x] Hata loglarinda stack trace bilgisi

### 13.5 Rust-Specific Guvenlik
- [x] `unsafe` blok kullanmaktan kacin (zorunlu degilse)
- [x] `#![forbid(unsafe_code)]` crate seviyesinde ekle
- [x] Integer overflow kontrolu (`checked_add`, `saturating_add` - f32 limitleri)
- [x] String encoding guvenilir handle etme (UTF-8 olmayan loglar icin lossy handle)
- [x] Buffer overflow onleme (sabit boyutlu buffer kullanilmiyor, Vec/String guvenligi)

---

## 15. CI/CD Pipeline (Surekli Entegrasyon / Dagitim)

### 14.1 GitHub Actions - CI Pipeline (`.github/workflows/ci.yml`)
- [x] Trigger: push (main, develop), pull_request
- [x] **Job 1: Check & Lint**
  - [x] `cargo fmt --check` (kod formatlama)
  - [x] `cargo clippy -- -D warnings` (sifir uyari hedefi)
  - [x] `cargo doc --no-deps` (dokumantasyon kontrolu)
- [x] **Job 2: Test**
  - [x] `cargo nextest run` (unit + integration testler)
  - [x] Test sonuclarini JUnit XML formatinda raporla (Nextest varsayilan destek)
  - [x] Coverage raporu olustur (`cargo tarpaulin --out xml`)
  - [x] Coverage raporu PR comment olarak goster (Configured in yaml)
- [x] **Job 3: Security**
  - [x] `cargo audit` (dependency guvenlik taramasi)
  - [x] `cargo deny check` (lisans + supply chain)
  - [x] SAST taramasi (semgrep veya cargo-geiger)
- [x] **Job 4: Build**
  - [x] `cargo build --release` (release binary)
  - [x] Build artifact'i kaydet (GitHub Actions artifact)
  - [x] Binary boyutunu olc ve raporla
- [x] **Job 5: Docker**
  - [x] `docker build` ile image olustur
  - [x] `docker run` ile smoke test (container baslatilip duruyor mu)
  - [x] Image boyutunu raporla
- [x] **Job 6: Frontend**
  - [x] `pnpm install` (dependencies)
  - [x] `pnpm lint` (ESLint)
  - [x] `pnpm type-check` (TypeScript)
  - [x] `pnpm build` (production build)
- [x] Cache stratejisi:
  - [x] `~/.cargo/registry` cache
  - [x] `target/` cache (sccache veya actions/cache)
  - [x] `node_modules/` cache (pnpm store)
- [x] Matrix build: Windows, Linux, macOS (en az Windows + Linux)

### 14.2 GitHub Actions - CD Pipeline (`.github/workflows/cd.yml`)
- [x] Trigger: tag push (`v*.*.*`) veya manual dispatch
- [x] Tauri build: Windows (.msi), Linux (.deb, .AppImage), macOS (.dmg)
- [x] Docker image push: GitHub Container Registry (ghcr.io)
- [x] GitHub Release olustur (binary + installer attach)
- [x] Changelog otomatik olusturma (git-cliff veya conventional commits)
- [x] Semantic versioning kontrolu

### 14.3 GitHub Actions - Release Pipeline (`.github/workflows/release.yml`)
- [x] Pre-release branch olustur
- [x] Tum testleri (unit + integration + e2e) calistir (CI ile entegre)
- [x] Tauri build tum platformlar icin (CD ile entegre)
- [x] Release notes olustur
- [x] GitHub Release publish et (CD ile entegre)
- [x] Docker image tag'le ve push et (`latest` + version) (CD ile entegre)

### 14.4 Pre-commit Hooks (Lokal CI)
- [x] `cargo fmt` otomatik calistir (commit oncesi)
- [x] `cargo clippy` otomatik calistir (commit oncesi)
- [x] Commit mesaji formati kontrolu (conventional commits)
- [x] Branch isimlendirme kontrolu (feature/, fix/, chore/)
- [x] `.env` dosyasinin commit edilmesini engelle (gitignore + hooks)

---

## 16. Gelistirme Surecleri ve Is Akisi (Development Workflow)

### 15.1 Git Workflow
- [ ] Git branching stratejisi belirle (Git Flow veya Trunk-based)
- [ ] `main` branch korumasi (branch protection rules)
- [ ] PR template olustur (`.github/pull_request_template.md`)
- [ ] Issue template olustur (bug report, feature request)
- [ ] Conventional Commits mesaj formati:
  ```
  feat: yeni ozellik
  fix: hata duzeltme
  refactor: yeniden yapilandirma
  test: test ekleme/duzeltme
  docs: dokumantasyon
  ci: CI/CD degisiklikleri
  chore: genel bakim
  ```
- [ ] `.gitattributes` dosyasi (line ending, binary dosya tanimlari)

### 15.2 Kod Review Sureci
- [ ] PR checklist olustur:
  - [ ] Kod derlenip testler geciyor mu?
  - [ ] Clippy uyarisi var mi?
  - [ ] Yeni testler eklendi mi?
  - [ ] Coverage dusmedi mi?
  - [ ] Dokumantasyon guncellendi mi?
- [ ] Minimum 1 reviewer zorunlulugu
- [ ] CI pipeline basarili olmadan merge engelleme

### 15.3 Gelistirme Ortami Otomasyonu
- [ ] `scripts/setup.ps1` - Windows ortam kurulum scripti:
  - [ ] Rust toolchain kontrol et / kur
  - [ ] Node.js kontrol et / kur
  - [ ] Cargo araclarini kur (clippy, fmt, nextest, tarpaulin, audit)
  - [ ] Tauri CLI kur
  - [ ] Frontend dependencies kur
  - [ ] .env.example -> .env kopyala
  - [ ] Git hooks kur
- [ ] `scripts/setup.sh` - Linux/WSL ortam kurulum scripti (ayni icerik)
- [ ] `cargo watch -x "clippy" -x "test"` ile hot-reload gelistirme
- [ ] `cargo tauri dev` ile Tauri hot-reload
- [ ] Makefile veya `just` (justfile) ile sik kullanilan komutlari kisayol yap:
  ```makefile
  dev:        cargo tauri dev
  test:       cargo nextest run
  lint:       cargo clippy -- -D warnings
  fmt:        cargo fmt
  coverage:   cargo tarpaulin --out Html
  build:      cargo tauri build
  docker:     docker-compose up --build
  clean:      cargo clean && cd frontend && pnpm clean
  audit:      cargo audit && cargo deny check
  e2e:        cargo nextest run --test-threads=1 -E 'test(e2e)'
  ```

### 15.4 Issue Tracking & Planlama
- [ ] GitHub Projects board olustur (Kanban: Backlog, In Progress, Review, Done)
- [ ] Milestone olustur: v0.1 (MVP), v0.2 (Tauri UI), v1.0 (Production Ready)
- [ ] Label sistemi: `bug`, `feature`, `enhancement`, `ci`, `docs`, `security`, `p0-critical`, `p1-high`, `p2-medium`

### 15.5 Monitoring & Observability (Gelistirme Sureci)
- [ ] CI pipeline surelerini izle (hizlandirma firsatlari)
- [ ] Test flakiness izleme (kararsiz testleri tespit et)
- [ ] Dependency update otomasyonu (Dependabot veya Renovate)
- [ ] Build boyutu trend izleme

---

## 17. Build & Run Komutlari

- [ ] `cargo build --release` ile uretim derlemesi
- [ ] `cargo test` ile tum testleri calistir
- [ ] `cargo nextest run` ile gelismis test calistirma
- [ ] `cargo clippy -- -D warnings` ile lint kontrolu (sifir uyari hedefi)
- [ ] `cargo fmt --check` ile kod formatlama kontrolu
- [ ] `cargo tarpaulin` ile test coverage olcumu
- [ ] `cargo audit` ile guvenlik taramasi
- [ ] `cargo tauri dev` ile Tauri gelistirme modu
- [ ] `cargo tauri build` ile Tauri production build
- [ ] `docker build -t anydesk-pivot-detector .` ile image olustur
- [ ] `docker-compose up -d` ile servisi baslat
- [ ] `docker-compose logs -f detector` ile loglari izle

---

## 18. Dokumantasyon

- [x] `README.md` - Proje aciklamasi, kurulum, kullanim
- [ ] `.env.example` - Ornek environment degiskenleri
- [ ] `ARCHITECTURE.md` - Sistem mimarisi ve modul aciklamalari
- [ ] `CONTRIBUTING.md` - Katki rehberi (git flow, PR sureci, test gereksinimleri)
- [ ] `CHANGELOG.md` - Surum degisiklikleri (conventional commits ile otomatik)
- [ ] CLI `--help` ciktisi tum komutlar icin
- [ ] Ornek cikti ekran goruntuleri / JSON ornekleri
- [ ] Tauri UI ekran goruntuleri

---

## 19. Guvenlik ve Son Kontroller

- [ ] `.env` dosyasi `.gitignore`'da mi? (EVET olmali)
- [ ] Hardcoded sifre veya gizli bilgi yok mu?
- [ ] Docker container non-root calistiyor mu?
- [ ] Log dosyalarinda hassas veri maskeleme yapiliyor mu?
- [ ] Tum `unwrap()` cagirilari `?` veya `expect()` ile degistirildi mi?
- [ ] `#![forbid(unsafe_code)]` aktif mi?
- [ ] `cargo audit` ile bilinen guvenlik aciklari kontrol edildi mi?
- [ ] `cargo deny check` ile lisans/supply-chain kontrolu yapildi mi?
- [ ] Windows UAC / yonetici yetki gereksinimleri dokumante edildi mi?
- [ ] Tauri CSP (Content Security Policy) ayarlari yapilandirildi mi?
- [ ] Frontend XSS korunmasi kontrol edildi mi?
- [ ] CI/CD secret'lari GitHub Secrets'ta mi (hardcoded degil)?

---

## 20. Teslim Kontrol Listesi

### 19.1 Backend
- [ ] Kod derlenip calistiyor (`cargo build --release` basarili)
- [ ] Tum unit testler geciyor (`cargo nextest run` basarili)
- [ ] Integration testler geciyor
- [ ] E2E testler geciyor (tum senaryolar)
- [ ] Clippy sifir uyari (`cargo clippy -- -D warnings`)
- [ ] Format tutarli (`cargo fmt --check`)
- [ ] Coverage >= %80 (`cargo tarpaulin`)
- [ ] Guvenlik taramasi temiz (`cargo audit`)
- [ ] `scan` komutu ornek verilerle calistiyor
- [ ] `monitor` komutu baslatilip durduruluyor
- [ ] `report` komutu JSON cikti uretiyor
- [ ] `config-check` komutu AnyDesk yapilandirmasini kontrol ediyor

### 19.2 Frontend (Tauri)
- [ ] Frontend derlenip calistiyor (`pnpm build` basarili)
- [ ] Tauri uygulama aciliyor (`cargo tauri dev`)
- [ ] Dashboard dogru verileri gosteriyor
- [ ] Tum butonlar ve interaksiyonlar calisiyor
- [ ] Dark/Light tema calisiyor
- [ ] Production build olusturuluyor (`cargo tauri build`)
- [ ] Windows installer (.msi) calisiyor

### 19.3 Docker
- [ ] Docker image olusturuluyor (`docker build` basarili)
- [ ] Docker container calistiyor (`docker-compose up` basarili)
- [ ] Container icindeki smoke test basarili

### 19.4 CI/CD
- [ ] CI pipeline tum job'lar basarili
- [ ] PR merge sureci calisiyor
- [ ] Release pipeline calisiyor (tag push ile tetikleme)
- [ ] Docker image registry'ye push ediliyor

### 19.5 Dokumantasyon & Repo
- [ ] Git repo temiz, tum dosyalar commit edildi
- [x] README.md guncel ve eksiksiz
- [ ] `.env.example` mevcut ve guncel
- [ ] ARCHITECTURE.md mevcut
- [ ] CONTRIBUTING.md mevcut
- [ ] CHANGELOG.md mevcut

---

**Toplam Gorev Sayisi**: 408  
**Tahmini Karmasiklik**: Yuksek  
**Oncelik Sirasi**: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> 11 -> 12 -> 13 -> 14 -> 15 -> 16 -> 17 -> 18 -> 19 -> 20  
**Kritik Yol**: Ortam(1) -> Proje(2) -> IDE(3) -> Deps(4) -> Config(5) -> Parsers(6) -> Monitors(7) -> Analyzers(8) -> Reporters(9) -> MainApp(10) -> Tauri(11) -> Docker(12) -> Tests+E2E(13) -> Hata Azaltma(14) -> CI/CD(15) -> Surecler(16) -> Build(17) -> Docs(18) -> Guvenlik(19) -> Teslim(20)
