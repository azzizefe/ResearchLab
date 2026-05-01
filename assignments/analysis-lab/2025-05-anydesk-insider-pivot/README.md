# AnyDesk İç Tehdit ve Pivot Tespit Sistemi 🛡️

![Rust](https://img.shields.io/badge/rust-1.78+-orange.svg)
![Tauri](https://img.shields.io/badge/tauri-v2-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![CI](https://img.shields.io/badge/CI-GitHub%20Actions-brightgreen.svg)

**AnyDesk Insider Pivot** saldırılarını tanımlamak ve etkisiz hale getirmek için tasarlanmış gelişmiş bir tespit ve izleme sistemi. Bu araç, AnyDesk'in tescilli protokolünü kullanarak gerçekleştirilen yetkisiz uzaktan erişim, yanal hareket (lateral movement) ve veri sızdırma girişimlerine karşı gerçek zamanlı görünürlük sağlar.

---

## 🚀 Genel Bakış
sssss
Modern kurumsal ortamlarda, AnyDesk gibi meşru uzaktan erişim araçları, çevre güvenliğini atlatmak için kötü niyetli içerideki kişiler (insiders) veya ele geçirilmiş hesaplar tarafından sıklıkla suistimal edilir. Bu sistem; AnyDesk davranışlarını izleyen, adli bilişim kalıntılarını analiz eden ve şüpheli etkinliklere risk puanları atayan Rust tabanlı güçlü bir tespit motoru sunar.

### Temel Yetenekler
*   **🔍 Gelişmiş Log Analizi**: Bağlantı ID'lerini, oturum sürelerini ve güvenlik yapılandırmalarını çıkarmak için `ad.trace`, `system.conf` ve `service.conf` dosyalarının derinlemesine analizi.
*   **⏱️ Gerçek Zamanlı İzleme**: AnyDesk süreçlerinin (processes), dosya sistemi değişikliklerinin ve ağ etkinliklerinin canlı takibi.
*   **🧠 Tehdit Analiz Motoru**: Mesai saatleri dışı bağlantılar, yetkisiz ACL değişiklikleri ve şüpheli süreç başlatma (örn: `anydesk.exe` -> `cmd.exe`) durumlarını tespit eden kural tabanlı motor ve anomali puanlayıcı.
*   **🖥️ Masaüstü Arayüzü (Tauri)**: Görsel izleme ve yapılandırma yönetimi için React ve Tailwind CSS ile oluşturulmuş premium dashboard.
*   **📊 Çoklu Raporlama**: JSON, Syslog (CEF/LEEF) ve formatlanmış konsol tabloları şeklinde dışa aktarılabilir raporlar.
*   **🐳 Docker Desteği**: Ölçeklenebilir izleme ortamları için konteynerize dağıtım desteği.

---

## 🛠️ Teknoloji Yığını

| Bileşen | Teknoloji |
| :--- | :--- |
| **Çekirdek Motor** | Rust (Tokio, Serde, Notify) |
| **GUI Çerçevesi** | Tauri v2 |
| **Frontend** | Vite, React 19, TypeScript, Tailwind CSS 4 |
| **Durum Yönetimi** | Zustand & TanStack Query |
| **Test Araçları** | Nextest, Tarpaulin, Criterion |
| **Dağıtım** | Docker & Docker Compose |

---

## 📂 Proje Yapısı

```text
anydesk-pivot-detector/
├── src/                    # Rust Backend
│   ├── api/                # Tauri IPC Komutları
│   ├── parsers/            # Log ve Yapılandırma Ayrıştırıcılar
│   ├── monitors/           # Süreç, Dosya ve Ağ İzleyiciler
│   ├── analyzers/          # Kural Motoru ve Anomali Puanlama
│   └── reporters/          # JSON, Konsol ve Syslog Çıktıları
├── src-tauri/              # Tauri Yapılandırması
├── frontend/               # React Dashboard (Vite + TS)
├── config/                 # Varsayılan yapılandırmalar
├── tests/                  # Birim, Entegrasyon ve E2E Testleri
└── docker-compose.yml      # Konteyner Orkestrasyonu
```

---

## 🚦 Başlangıç

### Gereksinimler
*   **Rust**: 1.78 veya üzeri (`rustup update stable`)
*   **Node.js**: 18 LTS veya üzeri
*   **Tauri CLI**: `cargo install tauri-cli`
*   **AnyDesk**: Log üretimi için yüklü veya taşınabilir sürüm

### Kurulum
1.  Depoyu klonlayın:
    ```bash
    git clone https://github.com/keyvanarasteh/ResearchLab.git
    cd ResearchLab/assignments/analysis-lab/2025-05-anydesk-insider-pivot
    ```
2.  Frontend bağımlılıklarını yükleyin:
    ```bash
    cd frontend && pnpm install && cd ..
    ```
3.  Ortamı yapılandırın:
    ```bash
    cp .env.example .env
    # .env dosyasını kendi yollarınıza göre düzenleyin
    ```

### Uygulamayı Çalıştırma
*   **Geliştirme Modu (GUI)**:
    ```bash
    cargo tauri dev
    ```
*   **CLI İzleme Modu**:
    ```bash
    cargo run -- monitor
    ```
*   **Tek Seferlik Tarama**:
    ```bash
    cargo run -- scan --path "C:/Log/Yolu"
    ```

---

## 🛡️ Tespit Senaryoları
Sistem aşağıdaki yüksek riskli senaryoları tespit edecek şekilde önceden yapılandırılmıştır:
1.  **İç Tehdit Pivotu**: Beyaz listede olmayan bir ID'den gelen yetkisiz bağlantı.
2.  **Shadow IT**: Yetkisiz dizinlerde taşınabilir (portable) AnyDesk sürümlerinin çalıştırılması.
3.  **Kalıcılık (Persistence)**: Zayıf veya statik parolalarla etkinleştirilmiş "Unattended Access".
4.  **Yanal Hareket**: AnyDesk üzerinden başlatılan şüpheli alt süreçler (örn: `net.exe`, `nmap.exe`).
5.  **Veri Sızdırma**: Uzak oturumlar sırasında tespit edilen yüksek hacimli veri transferleri.

---

## 🧪 Test ve Kalite
Kod kalitesi ve güvenilirlik için yüksek standartlar uygulanmaktadır:
*   **Birim Testleri**: `cargo test`
*   **Lint Kontrolü**: `cargo clippy -- -D warnings`
*   **Kod Kapsamı**: `cargo tarpaulin --out Html`
*   **Güvenlik Denetimi**: `cargo audit`

---

## 📝 Lisans ve Yazar
*   **Öğrenci No**: `2420191044`
*   **Proje UUID**: `1253dbcd-b308-4443-a29e-036bbe0b27c7`
*   **Lisans**: MIT

---
*ResearchLab Güvenlik Analizi Ödevleri kapsamında oluşturulmuştur.*
