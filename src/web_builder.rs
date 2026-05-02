use pulldown_cmark::{html, Parser};
use std::fs;

fn main() {
    println!("🌐 Radar Web Sitesi (Blog) Üretiliyor...");

    // Eğer public klasörü yoksa oluştur
    let _ = fs::create_dir_all("public");

    // daily_radar.md dosyasını oku (Yoksa boş bir metin oluştur)
    let markdown_content = fs::read_to_string("daily_radar.md")
        .unwrap_or_else(|_| "Henüz veri yok.".to_string());

    // Markdown'ı HTML'ye çevir
    let parser = Parser::new(&markdown_content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // CSS ve HTML İskeletini oluştur (Matrix/Fütüristik Tema)
    let final_html = format!(
        r#"<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Omni-Radar Intelligence</title>
    <style>
        body {{
            background-color: #0d1117;
            color: #c9d1d9;
            font-family: 'Courier New', Courier, monospace;
            line-height: 1.6;
            max-width: 900px;
            margin: 0 auto;
            padding: 40px 20px;
        }}
        h1, h2, h3 {{ color: #58a6ff; }}
        a {{ color: #8a2be2; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        blockquote {{
            border-left: 4px solid #30363d;
            padding-left: 20px;
            color: #8b949e;
            font-style: italic;
        }}
        .header {{
            text-align: center;
            border-bottom: 1px solid #30363d;
            padding-bottom: 20px;
            margin-bottom: 40px;
        }}
        .logo {{ font-size: 2em; font-weight: bold; color: #58a6ff; }}
        .slogan {{ color: #8b949e; font-size: 0.9em; }}
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">📡 Omni-Radar Intelligence</div>
        <div class="slogan">Günlük Fütüristik Sentez ve Makro-Trendler</div>
    </div>
    <div class="content">
        {}
    </div>
</body>
</html>"#,
        html_output
    );

    // public/index.html olarak kaydet
    fs::write("public/index.html", final_html).expect("HTML dosyası yazılamadı!");
    println!("✅ Web sitesi başarıyla üretildi: public/index.html");
}