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
body {
            background-color: #0d1117;
            color: #c9d1d9;
            font-family: 'Fira Code', 'Courier New', Courier, monospace;
            line-height: 1.6;
            max-width: 900px;
            margin: 0 auto;
            padding: 40px 20px;
        }
        h1, h2, h3 { 
            color: #58a6ff; 
            border-bottom: 1px solid #30363d;
            padding-bottom: 0.3em;
        }
        a { color: #bc8cff; text-decoration: none; transition: 0.3s; }
        a:hover { color: #58a6ff; text-shadow: 0 0 8px rgba(88,166,255,0.5); }
        blockquote {
            border-left: 4px solid #bc8cff;
            background: rgba(188, 140, 255, 0.05);
            padding: 10px 20px;
            border-radius: 0 8px 8px 0;
            color: #8b949e;
            font-style: italic;
        }
        .header {
            text-align: center;
            border-bottom: 2px solid #58a6ff;
            padding-bottom: 20px;
            margin-bottom: 40px;
            position: relative;
        }
        .logo { 
            font-size: 2.5em; 
            font-weight: 900; 
            color: transparent; 
            background: linear-gradient(90deg, #58a6ff, #bc8cff);
            -webkit-background-clip: text;
            letter-spacing: -1px;
        }
        .slogan { color: #8b949e; font-size: 1.1em; letter-spacing: 2px; text-transform: uppercase; margin-top: 10px;}
        .glitch-bar {
            height: 2px;
            background: #bc8cff;
            width: 50px;
            margin: 20px auto 0;
            animation: scan 3s infinite alternate;
        }
        @keyframes scan {
            0% { width: 10px; opacity: 0.2; }
            100% { width: 200px; opacity: 1; box-shadow: 0 0 10px #bc8cff; }
        }
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">📡 Omni-Radar Intelligence</div>
        <div class="slogan">Günlük Fütüristik Sentez ve Makro-Trendler</div>
        <div class="glitch-bar"></div>
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