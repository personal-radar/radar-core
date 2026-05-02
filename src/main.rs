use reqwest::Client;
use serde_json::json;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Radar uyandı. Haberler toplanıyor...");

    // 1. Repo Sahibinin Adını Dinamik Al
    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_else(|_| "Geliştirici".to_string());
    println!("👤 Hedef Kullanıcı/Organizasyon: {}", repo_owner);

    let client = Client::new();

    // 2. Hacker News'ten En Popüler Haberleri Çek
    let hn_url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let story_ids: Vec<u64> = client.get(hn_url).send().await?.json().await?;

    let mut news_titles = String::new();
    for &id in story_ids.iter().take(15) {
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

    // 3. Dışarıdaki Prompt dosyasını oku
    let prompt_template = fs::read_to_string("system_prompt.txt")
        .await
        .unwrap_or_else(|_| "Haberleri özetle.".to_string());

    // 4. Prompt'u repo sahibiyle birleştir
    let final_prompt = format!(
        "{}\n\nLütfen bu analizi organizasyon/kullanıcı olan '{}' için özelleştir.\n\nİşte Haberler:\n{}",
        prompt_template, repo_owner, news_titles
    );

    let payload = json!({
        "contents": [{
            "parts": [{"text": final_prompt}]
        }]
    });

    let response = client.post(&gemini_url).json(&payload).send().await?;
    let status = response.status();
    let response_text = response.text().await?; // Hata olursa diye metni okuyoruz

    // 5. Hata Kontrolü!
    if !status.is_success() {
        eprintln!("❌ Gemini API Hatası: {}", response_text);
        std::process::exit(1); // Action'ı burada durdurur
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        fs::write("daily_radar.md", text).await?;
        println!("✅ Analiz tamamlandı. daily_radar.md oluşturuldu.");
    } else {
        eprintln!("❌ Gemini'den anlamsız yanıt geldi. Detay: {}", response_text);
        std::process::exit(1);
    }

    Ok(())
}