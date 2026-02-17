use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use headless_chrome::{Browser, LaunchOptions};
use base64::{Engine as _, engine::general_purpose};
use chrono::Local;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnalysisResult {
    pub company_name: String,
    pub company_summary: String,
    pub cover_letter: String,
    pub new_cv_title: String,
}

#[tauri::command]
async fn process_offer(url: String) -> Result<AnalysisResult, String> {
    // 0. Load API Key from Env
    dotenv::dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|_| "GROQ_API_KEY non trouvée dans le fichier .env (src-tauri/.env)".to_string())?;

    // 1. Fetch Job Content
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7")
        .send().await
        .map_err(|e| format!("Erreur réseau lors de la récupération de l'offre: {}", e))?;

    let body = resp.text().await.map_err(|e| e.to_string())?;

    // Debug: Log the first 500 chars of body
    println!("DEBUG: Contenu de la page (500 chars): {}", body.chars().take(500).collect::<String>());

    if body.contains("Security Verification") || body.contains("CAPTCHA") {
        return Err("L'offre est protégée par un CAPTCHA ou LinkedIn bloque l'accès direct. Essayez de copier le texte de l'offre si possible (intégration future).".to_string());
    }

    // Basic stripping (real scraping would be better but let's keep it robust)
    let job_text = body.chars().take(5000).collect::<String>();

    // 2. Read User CV (Template)
    let cv_path = PathBuf::from("..").join("public").join("cv.html");
    let cv_content = fs::read_to_string(cv_path).map_err(|_| "CV (cv-template.html) non trouvé.".to_string())?;

    // 3. Groq AI Call
    let today = Local::now().format("%d/%m/%Y").to_string();
    let letter_template = format!("
        [Prénom] [Nom]
        [Téléphone] | [Email]

        À l'attention du responsable du recrutement
        [Nom de l'entreprise]
        [Lieu], le {}

        Objet : Candidature au poste de [Titre du poste]

        Monsieur, Madame,

        [Introduction : Pourquoi cette entreprise et ce poste ? Accroche forte liée à l'offre.]

        [VOUS : Montrez que vous avez compris leurs enjeux/besoins actuels.]

        [MOI : Vos compétences clés et réalisations en lien avec l'offre. Parlez de votre formation à Holberton et vos projets.]

        [NOUS : Ce que vous allez apporter à l'entreprise. Votre motivation pour l'alternance.]

        Dans l'attente d'un échange, je vous prie d'agréer, Monsieur, Madame, l'expression de mes salutations distinguées.

        [Prénom] [Nom]
    ",
        today
    );

    let prompt = format!(
        "Tu es un assistant de recrutement expert. Je vais te donner une offre d'emploi et mon CV. 
        Tache : 
        1. Identifie le nom de l'entreprise.
        2. Fais un résumé court (2-3 phrases) de l'entreprise et de l'offre.
        3. Rédige une LETTRE DE MOTIVATION PARFAITE en français en suivant EXACTEMENT ce modèle de structure :
        ---
        {}
        ---
        4. Propose un titre court pour mon CV adapté à ce poste.

        Offre d'emploi :
        ---
        {}
        ---

        Mon CV :
        ---
        {}
        ---

        Réponds UNIQUEMENT au format JSON strict suivant :
        {{\"company_name\": \"...\", \"company_summary\": \"...\", \"cover_letter\": \"...\", \"new_cv_title\": \"...\"}}",
        letter_template, job_text, cv_content
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let payload = serde_json::json!({
        "messages": [{"role": "user", "content": prompt}],
        "model": "llama-3.3-70b-versatile",
        "response_format": { "type": "json_object" }
    });

    let resp = client.post("https://api.groq.com/openai/v1/chat/completions")
        .headers(headers)
        .json(&payload)
        .send().await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = resp.json().await.map_err(|e| format!("Erreur JSON Groq: {}", e))?;
    
    let ai_text = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| format!("Réponse Groq vide ou malformée: {:?}", json))?;

    println!("DEBUG: Réponse IA: {}", ai_text);

    let result: AnalysisResult = serde_json::from_str(ai_text).map_err(|e| format!("Erreur de parsing AI Result: {} | Texte: {}", e, ai_text))?;
    
    Ok(result)
}

fn html_to_pdf(html: &str) -> Result<String, String> {
    let browser = Browser::new(LaunchOptions::default()).map_err(|e| e.to_string())?;
    let tab = browser.new_tab().map_err(|e| e.to_string())?;
    
    // We use a data URL to load the HTML content
    let data_url = format!("data:text/html;charset=utf-8,{}", urlencoding::encode(html));
    tab.navigate_to(&data_url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;
    
    let pdf_options = None; // Default A4
    let pdf_bytes = tab.print_to_pdf(pdf_options).map_err(|e| e.to_string())?;
    
    Ok(general_purpose::STANDARD.encode(pdf_bytes))
}

#[tauri::command]
async fn generate_pdfs(result: AnalysisResult, url: String) -> Result<serde_json::Value, String> {
    let cv_html_path = PathBuf::from("..").join("public").join("cv.html");
    let cv_css_path = PathBuf::from("..").join("public").join("cv-style.css");
    let original_cv = fs::read_to_string(cv_html_path).map_err(|e| e.to_string())?;
    let css = fs::read_to_string(cv_css_path).map_err(|e| e.to_string())?;

    // 1. Summary PDF
    let summary_html = format!(
        "<html><body style='font-family: sans-serif; padding: 40px;'><h1>Résumé : {}</h1><h2>Entreprise</h2><p>{}</p><h2>Source</h2><p>{}</p></body></html>",
        result.company_name, result.company_summary, url
    );
    let summary_pdf = html_to_pdf(&summary_html)?;

    // 2. Letter PDF
    let letter_html = format!(
        "<html><body style='font-family: serif; padding: 50px; white-space: pre-wrap;'>{}</body></html>",
        result.cover_letter
    );
    let letter_pdf = html_to_pdf(&letter_html)?;

    // 3. Adapted CV PDF
    let mut adapted_cv = original_cv.replace(
        "<h2 id=\"user-title\" contenteditable=\"true\">DÉVELOPPEUR WEB</h2>",
        &format!("<h2 id=\"user-title\" contenteditable=\"true\">{}</h2>", result.new_cv_title.to_uppercase())
    );
    
    // Inject CSS
    adapted_cv = adapted_cv.replace("<link rel=\"stylesheet\" href=\"cv-style.css\">", &format!("<style>{}</style>", css));
    
    // Fix Image (Embed as Base64 for maximum reliability)
    let photo_path = PathBuf::from("..").join("public").join("Profil_ai1.jpg");
    if let Ok(photo_bytes) = fs::read(photo_path) {
        let b64 = general_purpose::STANDARD.encode(photo_bytes);
        adapted_cv = adapted_cv.replace("src=\"Profil_ai1.jpg\"", &format!("src=\"data:image/jpeg;base64,{}\"", b64));
    }

    let cv_pdf = html_to_pdf(&adapted_cv)?;

    Ok(serde_json::json!({
        "summary": summary_pdf,
        "letter": letter_pdf,
        "cv": cv_pdf
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_offer, generate_pdfs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
