# 📡 Personal Intelligence Radar (Zero-Cost AI OSINT)

Dünyadaki teknoloji gürültüsünden sıkıldınız mı? Saatlerce video izlemek veya clickbait makaleler okumak yerine, yapay zekanın sizin yerinize en önemli haberleri seçip, özetleyip, sizinle tartıştığı bir asistana ne dersiniz?

Personal Intelligence Radar; **%100 Ücretsiz**, tamamen **GitHub Actions** üzerinde çalışan, **Rust** ile güçlendirilmiş ve **Gemini 2.5 Flash API** kullanan açık kaynaklı bir istihbarat hattıdır.

## ✨ Ne Yapar?
1. **Gecelik Hasat:** Her gece belirlediğiniz kaynakları (HackerNews, RSS vs.) tarar.
2. **AI Filtresi:** Gemini 2.5 API kullanarak magazinel/çöp haberleri atar, sadece sistem mimarisi, deep-tech ve spesifik konuları seçip özetler.
3. **Günlük Gazete:** Sonuçları size bir GitHub Issue olarak açar.
4. **Chatbot (En İyisi!):** Açılan bu Issue'ya yorum yazdığınızda, yapay zeka okuduğunuz haber içeriğine ve sizin yorumunuza göre size bir "Yazılım Mimarı" gibi teknik tavsiyelerde bulunur, kod yazar.

## 🚀 Kendi Radarınizi Nasıl Kurarsınız? (Sadece 2 Dakika)

1. Sağ üstteki yeşil **"Use this template" -> "Create a new repository"** butonuna basın ve kendi reponuzu oluşturun.
2. Kendi reponuzda **Settings > Secrets and variables > Actions** sekmesine gidin.
3. **New repository secret** butonuna basarak adını `GEMINI_API_KEY` yapın ve [Google AI Studio](https://aistudio.google.com/)'dan aldığınız ücretsiz API anahtarınızı yapıştırın.
4. Reponuzun içindeki `system_prompt.txt` dosyasını kendi ilgi alanlarınıza göre (Örn: "Sadece uzay teknolojileri ve Python haberlerini getir") düzenleyin.
5. GitHub'ın üst menüsünden **Actions** sekmesine girip, "Daily Intelligence Radar" iş akışını manuel tetikleyerek (Run workflow) ilk gazetenizi okumaya başlayın!

## 🛠️ Mimari ve Teknolojiler
- **Toplayıcı ve Bot:** Rust (Çok hafif olduğu için GitHub Actions süresini sıfıra yakın kullanır).
- **Yapay Zeka:** Gemini 2.5 Flash API.
- **Veritabanı ve Arayüz:** Sadece GitHub Issues! Sunucu yok, veritabanı maliyeti yok.
