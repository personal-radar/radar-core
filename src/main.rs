use reqwest::Client;
use serde_json::json;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Radar uyandı. Haberler toplanıyor...");

    let client = Client::new();

    // 1. Hacker News'ten En Popüler Haber ID'lerini çek
    let hn_url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let story_ids: Vec<u64> = client.get(hn_url).send().await?.json().await?;

    // İlk 10 haberi alıp başlıklarını toplayalım
    let mut news_titles = String::new();
    for &id in story_ids.iter().take(10) {
        let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        let story: serde_json::Value = client.get(&story_url).send().await?.json().await?;
        if let Some(title) = story["title"].as_str() {
            news_titles.push_str(&format!("- {}\n", title));
        }
    }

    println!("🧠 Gemini API'ye bağlanılıyor...");
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY bulunamadı!");
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    // 2. Gemini için Harika Bir Sistem Prompt'u
    let prompt = format!(
        "Sen üst düzey bir Yazılım Mimarı ve teknoloji analistisin. \
        Aşağıda bugünün en popüler 10 teknoloji haberi var. \
        Lütfen magazin veya önemsiz konuları filtrele. \
        Sadece Rust, Sistem Mimarisi, Dağıtık Sistemler (Space OS vb.), Yapay Zeka veya önemli teknolojik gelişmelerle ilgili olan en kritik 3 haberi seç. \
        Bunları Türkçe olarak, teknik ama akıcı bir dille, Markdown formatında bir bülten haline getir. \
        Sonuna mutlaka 'Azmi için Mimari Yorum' başlıklı kısa bir tavsiye paragrafı ekle.\n\n\
        İşte Haberler:\n{}",
        news_titles
    );

    let payload = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let response = client.post(&gemini_url).json(&payload).send().await?;
    let response_json: serde_json::Value = response.json().await?;

    // 3. Gemini'nin cevabını ayrıştır
    if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        // Sonucu bir Markdown dosyasına kaydet
        fs::write("daily_radar.md", text).await?;
        println!("✅ Analiz tamamlandı. daily_radar.md dosyası oluşturuldu.");
    } else {
        println!("❌ Gemini'den beklenen yanıt alınamadı.");
    }

    Ok(())
}