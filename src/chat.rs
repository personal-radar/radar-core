use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Chatbot uyandı. Yorum analiz ediliyor...");

    // GitHub Action ortamından verileri al
    let issue_body = env::var("ISSUE_BODY").unwrap_or_default();
    let user_comment = env::var("USER_COMMENT").unwrap_or_default();
    let commenter = env::var("COMMENTER").unwrap_or_default();
    
    // Eğer bot kendi kendine yorum yaptıysa, sonsuz döngüye girmemek için dur
    if commenter == "github-actions[bot]" || commenter.contains("bot") {
        println!("Bota bot cevap vermez. İşlem iptal.");
        return Ok(());
    }

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY bulunamadı!");
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    // Chat için Prompt
    let chat_prompt = format!(
        "Sen üst düzey bir Yazılım Mimarı asistanısın.\n\n\
        Aşağıda kullanıcının (Azmi/Kullanıcı) okuduğu güncel teknoloji raporu var:\n\
        ---\n{}\n---\n\n\
        Kullanıcının ({}) bu rapora yaptığı yorum/soru şu:\n\
        👉 \"{}\"\n\n\
        Kullanıcıya teknik, yol gösterici ve çözüm odaklı bir yanıt ver. \
        Gerekirse kod örneği (özellikle Rust veya Platform mühendisliği ise) ekle.",
        issue_body, commenter, user_comment
    );

    let client = Client::new();
    let payload = json!({
        "contents": [{
            "parts": [{"text": chat_prompt}]
        }]
    });

    let response = client.post(&gemini_url).json(&payload).send().await?;
    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        // Çıktıyı GitHub Action'ın bir sonraki adıma aktarması için bir dosyaya yaz
        tokio::fs::write("chat_reply.md", text).await?;
        println!("✅ Chat cevabı hazırlandı.");
    } else {
        eprintln!("❌ Gemini cevap üretemedi.");
        std::process::exit(1);
    }

    Ok(())
}