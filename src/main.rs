use reqwest::Client;
use serde_json::json;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Omni-Radar uyandı. Farklı disiplinlerden veriler hasat ediliyor...");

    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_else(|_| "Kullanıcı".to_string());
    let client = Client::new();

    // 1. Kaynakları Oku
    let sources_content = fs::read_to_string("sources.json").await.unwrap_or_else(|_| "[]".to_string());
    let sources: Vec<String> = serde_json::from_str(&sources_content)?;

    let mut all_news = String::new();

    // 2. Tüm RSS Kaynaklarını Tara
    for url in sources {
        println!("🔍 Taranıyor: {}", url);
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(bytes) = response.bytes().await {
                    if let Ok(feed) = feed_rs::parser::parse(bytes.as_ref()) {
                        all_news.push_str(&format!("\n--- Kaynak: {} ---\n", feed.title.map_or("Bilinmeyen Kaynak".to_string(), |t| t.content)));
                        for entry in feed.entries.into_iter().take(3) { // Her kaynaktan en yeni 3 haber
                            let title = entry.title.map_or("Başlıksız".to_string(), |t| t.content);
                            let summary = entry.summary.map_or("".to_string(), |s| s.content);
                            // Metni çok uzatmamak için özetin sadece ilk 200 karakterini alıyoruz
                            let short_summary = if summary.len() > 200 { format!("{}...", &summary[..200]) } else { summary };
                            all_news.push_str(&format!("- Başlık: {}\n  Detay: {}\n", title, short_summary));
                        }
                    }
                }
            }
            Err(e) => eprintln!("⚠️ {} okunamadı: {}", url, e),
        }
    }

    println!("🧠 Gemini 2.5 Flash Sentez Motoru Başlatılıyor...");
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY bulunamadı!");
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    let prompt_template = fs::read_to_string("system_prompt.txt")
        .await
        .unwrap_or_else(|_| "Haberleri özetle.".to_string());

    let final_prompt = format!(
        "{}\n\nLütfen bu çok disiplinli analizi ve sentezi organizasyon/kullanıcı olan '{}' için özelleştir.\n\nİşte Günün Küresel Verileri:\n{}",
        prompt_template, repo_owner, all_news
    );

    let payload = json!({
        "contents": [{
            "parts": [{"text": final_prompt}]
        }]
    });

    let response = client.post(&gemini_url).json(&payload).send().await?;
    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        eprintln!("❌ Gemini API Hatası: {}", response_text);
        std::process::exit(1);
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        fs::write("daily_radar.md", text).await?;
        println!("✅ Çok disiplinli analiz tamamlandı. daily_radar.md oluşturuldu.");
    } else {
        eprintln!("❌ Gemini sentez yapamadı. Detay: {}", response_text);
        std::process::exit(1);
    }

    Ok(())
}