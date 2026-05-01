# AnyDesk Insider Pivot Detection & Monitoring System 🛡️

![AnyDesk Pivot Detector Logo](https://raw.githubusercontent.com/your-repo/anydesk-pivot-detector/main/app-icon.png)

![Rust](https://img.shields.io/badge/rust-1.94.1-orange.svg?style=for-the-badge&logo=rust)
![Tauri](https://img.shields.io/badge/tauri-v2-blue.svg?style=for-the-badge&logo=tauri)
![Architecture](https://img.shields.io/badge/Architecture-Modular-blueviolet.svg?style=for-the-badge)
![Security](https://img.shields.io/badge/Security-Hardened-red.svg?style=for-the-badge)

**AnyDesk Insider Pivot Detection System**, kurumsal ağlarda "Remote Access" (Uzaktan Erişim) araçlarının suistimal edilmesiyle gerçekleştirilen sofistike saldırıları tespit etmek için Rust diliyle geliştirilmiş, yüksek performanslı bir güvenlik çözümüdür.

---

## 📖 İçindekiler
1. [Sistem Nedir?](#-sistem-nedir)
2. [Pivot Saldırısı Nedir ve Neden Tehlikelidir?](#-pivot-saldırısı-nedir)
3. [Temel Özellikler](#-temel-özellikler)
4. [Teknik Mimari ve Çalışma Mantığı](#-teknik-mimari)
5. [Tespit Kuralları ve Risk Puanlama](#-tespit-kuralları)
6. [Kurulum ve Başlatma](#-kurulum)
7. [Kullanım Kılavuzu (GUI & CLI)](#-kullanım-kılavuzu)
8. [Geliştirici Notları ve Güvenlik](#-geliştirici-notları)
9. [Sıkça Sorulan Sorular (S.S.S)](#-sss)

---

## 🕵️ Sistem Nedir?

Bu sistem, AnyDesk uygulaması üzerinden gerçekleştirilen **"İç Tehdit" (Insider Threat)** ve **"Yanal Hareket" (Lateral Movement)** aktivitelerini gerçek zamanlı olarak izler. Geleneksel antivirüslerin (AV) ve EDR sistemlerinin genellikle "meşru bir yazılım" olduğu için gözden kaçırdığı AnyDesk tabanlı saldırı vektörlerini, davranışsal analiz ve log analizi yöntemleriyle saniyeler içinde ortaya çıkarır.

---

## 🌪️ Pivot Saldırısı Nedir?

**Pivot**, bir saldırganın ağdaki zayıf bir cihazı (başlangıç noktası) ele geçirdikten sonra, bu cihazı bir sıçrama tahtası olarak kullanarak ağın daha derinlerindeki (normalde erişilemeyen) sunuculara veya kritik verilere ulaşması işlemidir.

**AnyDesk bu süreçte nasıl kullanılır?**
*   **Bypass**: Güvenlik duvarlarını ve dışarıdan içeriye olan kısıtlamaları AnyDesk'in tersine bağlantı (reverse connection) özelliğiyle aşmak.
*   **Persistent Access**: Kurbanın bilgisayarına "Unattended Access" (Gözetimsiz Erişim) şifresi koyarak kalıcılık sağlamak.
*   **Data Exfiltration**: Hassas dosyaları AnyDesk'in dosya transferi özelliğiyle dışarı sızdırmak.

---

## ✨ Temel Özellikler

### 🛡️ 1. Gerçek Zamanlı İzleme (Real-time Monitoring)
*   `ad.trace` log dosyalarını `FileWatcher` mekanizması ile saniye saniye takip eder.
*   Yeni bir bağlantı kurulduğu an saldırganın AnyDesk ID'sini, IP adresini ve bağlantı yönünü analiz eder.

### 🧠 2. Akıllı Risk Puanlama (Anomaly Scorer)
*   Gerçekleştirilen her şüpheli eylem (Dosya transferi, mesai dışı erişim, bilinmeyen ID) sisteme bir puan ekler.
*   Risk puanı kritik eşiği (%70+) aştığında görsel ve sesli uyarılar oluşturur.

### 🔍 3. Adli Bilişim (Forensics) Desteği
*   Geçmişe dönük log taraması yaparak, saldırganın sistemde bıraktığı "ayak izlerini" (artifacts) bir araya getirir.
*   Saldırı anını kronolojik bir rapor (JSON/HTML) olarak sunar.

### 🌐 4. Muazzam Kullanıcı Arayüzü (Premium GUI)
*   **Glassmorphism** tasarımı ile modern ve göz yormayan karanlık tema.
*   **TR/EN Dil Desteği** ile global standartlarda kullanım.
*   **Dinamik Dashboard**: Canlı risk grafikleri ve anlık olay akışı.

---

## 🏗️ Teknik Mimari

Sistem üç ana katmandan oluşur:

### 1. Rust Core Engine (Backend)
*   **Parser Modülleri**: AnyDesk'in karmaşık log yapısını çözer.
*   **Rule Engine**: Konfigürasyon dosyasındaki kuralları (`ACL`, `Working Hours`, `Suspicious Processes`) her olaya uygular.
*   **Safety Layer**: `#![forbid(unsafe_code)]` direktifi ile bellek güvenliği en üst düzeyde tutulur.

### 2. Tauri App Layer (Bridge)
*   Rust'ın gücüyle HTML/JS'in esnekliğini birleştirir.
*   Backend'den gelen güvenlik uyarılarını anlık olarak (`IPC` üzerinden) arayüze iletir.

### 3. Frontend (React + Tailwind v4)
*   Vite ile optimize edilmiş, ultra hızlı kullanıcı arayüzü.
*   `Zustand` ile merkezi durum yönetimi (State management).
*   `Lucide-React` ile vektörel güvenlik ikonları.

---

## 🎯 Tespit Kuralları

Sistem şu senaryoları otomatik olarak yakalar:

1.  **Unauthorized ID**: Beyaz listede (ACL) olmayan bir AnyDesk numarasından gelen bağlantılar.
2.  **After Hours Access**: Şirket çalışma saatleri (örn: 09:00-18:00) dışında gerçekleşen tüm aktiviteler.
3.  **Sensitive Path Access**: `SAM`, `ntds.dit`, `.env` veya cüzdan dosyalarına erişim girişimleri.
4.  **Process Injection via AnyDesk**: AnyDesk'in `cmd.exe` veya `powershell.exe` gibi kritik süreçleri başlatması.
5.  **Unattended Access Usage**: Şifre ile otomatik giriş yapıldığında (Kalıcılık belirtisidir).

---

## ⚙️ Kurulum

### Gereksinimler
*   **Rust**: 1.94.1+
*   **Node.js**: 20+
*   **AnyDesk**: İzlenecek makinede yüklü olmalıdır (veya log dosyaları mevcut olmalıdır).

### Adımlar
1.  **Depoyu Klonlayın**:
    ```bash
    git clone https://github.com/your-username/anydesk-pivot-detector.git
    cd anydesk-pivot-detector
    ```

2.  **Konfigürasyonu Ayarlayın**:
    `.env.example` dosyasını `.env` olarak kopyalayın ve log yollarınızı belirtin:
    ```env
    ANYDESK_TRACE_PATH="C:\ProgramData\AnyDesk\ad.trace"
    ```

3.  **Uygulamayı Başlatın**:
    ```bash
    # Arayüz ile başlatmak için
    cargo tauri dev

    # CLI (Komut Satırı) ile izleme başlatmak için
    cargo run -- monitor
    ```

---

## 🖥️ Kullanım Kılavuzu

### Dashboard (Panel)
Sistemin genel sağlığını gösterir. Eğer Risk Puanı kırmızıya dönerse, derhal "Alerts" sekmesine geçilmelidir.

### Alerts (Uyarılar)
Tüm güvenlik olaylarını ciddiyet derecesine (Düşük, Orta, Yüksek, Kritik) göre listeler. Her uyarıya tıklayarak "Evidence" (Kanıt) verilerini görebilirsiniz.

### Logs (Canlı Loglar)
AnyDesk'in arka planda ürettiği ham verileri temizlenmiş ve renklendirilmiş bir terminal formatında sunar.

---

## 🛡️ Geliştirici Notları ve Güvenlik

*   **Veri Maskeleme**: Sistem, AnyDesk ID'lerinin son hanelerini maskeleyerek (örn: 123 456 ***) PII (Kişisel Veri) güvenliğini sağlar.
*   **Düşük Kaynak Tüketimi**: Rust sayesinde sistem boştayken %0.1'den daha az CPU tüketir.
*   **Bağımsızlık**: Sistem tamamen lokalde çalışır; hiçbir veriniz buluta veya dış sunuculara gönderilmez.

---

## ❓ Sıkça Sorulan Sorular (S.S.S)

**S: Bu bir antivirüs mü?**
C: Hayır, bu bir "Davranışsal Tespit Sistemi"dir (Detection System). AnyDesk'in kötüye kullanımına odaklanır.

**S: Kendi AnyDesk ID'mi beyaz listeye nasıl eklerim?**
C: `config/default.toml` dosyasındaki `allowed_anydesk_ids` listesine ID'nizi eklemeniz yeterlidir.

**S: Linux veya macOS üzerinde çalışır mı?**
C: Evet, sistem multi-platform (Windows, Linux, macOS) desteğine sahiptir ancak AnyDesk log yolları işletim sistemine göre manuel ayarlanmalıdır.

---

## 📜 Lisans

Bu proje **MIT Lisansı** ile lisanslanmıştır. Daha fazla bilgi için `LICENSE` dosyasına bakınız.

---
**Geliştiren:** `2420191044`  
**Proje Durumu:** v0.1.0 - Production Ready 🚀
