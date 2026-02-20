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
    pub job_title: String,
    #[serde(default)]
    pub offer_text: String, // Added field
}

#[tauri::command]
async fn process_offer(url: String) -> Result<AnalysisResult, String> {
    // 0. Load API Key from Env
    dotenv::dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|_| "GROQ_API_KEY non trouvée dans le fichier .env (src-tauri/.env)".to_string())?;

    // 1. Fetch Job Content using Headless Chrome (better for JS & text extraction)
    println!("DEBUG: Fetching URL with Headless Chrome: {}", url);
    let job_text = std::thread::spawn(move || -> Result<String, String> {
        let browser = Browser::new(LaunchOptions::default()).map_err(|e| e.to_string())?;
        let tab = browser.new_tab().map_err(|e| e.to_string())?;
        
        tab.navigate_to(&url).map_err(|e| e.to_string())?;
        tab.wait_until_navigated().map_err(|e| e.to_string())?;
        
        // Wait a bit for dynamic content?
        std::thread::sleep(std::time::Duration::from_secs(2));

        let body = tab.wait_for_element("body").map_err(|e| e.to_string())?;
        let text = body.get_inner_text().map_err(|e| e.to_string())?;
        
        Ok(text)
    }).join().map_err(|_| "Thread panic".to_string())??;

    // Debug: Log the first 500 chars of text
    println!("DEBUG: Contenu de la page (500 chars): {}", job_text.chars().take(500).collect::<String>());

    // 2. Read User CV (Template)
    let cv_path = PathBuf::from("..").join("public").join("cv.html");
    let cv_content = fs::read_to_string(cv_path).map_err(|_| "CV (cv-template.html) non trouvé.".to_string())?;

    // 3. Groq AI Call
    
    // Updated Template: Removed Header placeholders because formatting handles it. 
    // AI only provides Body.
    let letter_structure = "
        Monsieur, Madame,

        [Introduction : Accroche personnalisée. Pourquoi cette entreprise spécifiquement ? Qu'est-ce qui vous attire dans leur vision ou leurs produits ?]

        [VOUS : Analyse des besoins de l'entreprise basés sur l'offre. Montrez que vous comprenez leurs défis techniques ou business.]

        [MOI : Focus sur votre parcours à Holberton School (approche peer-learning, projets intensifs). Mentionnez vos compétences techniques (JS, Python, Rust, etc.) et vos projets personnels comme votre plateforme de cybersécurité (whooami.net). Faites le lien direct avec l'offre.]

        [NOUS : Projection de votre collaboration. Ce que votre curiosité et votre capacité d'apprentissage rapide apporteront à l'équipe. Votre motivation pour ce contrat d'apprentissage/alternance.]

        Dans l'attente d'un échange, je vous prie d'agréer, Monsieur, Madame, l'expression de mes salutations distinguées.
    ";

    let prompt = format!(
        "Tu es un assistant de recrutement expert. Je vais te donner une offre d'emploi et mon CV. 
        Tache : 
        1. Identifie le nom de l'entreprise.
        2. Identifie le TITRE EXACT DU POSTE (job_title).
        3. Fais un résumé court (2-3 phrases) de l'entreprise et de l'offre.
        4. Rédige une LETTRE DE MOTIVATION en français.
           IMPORTANT : 
           - NE METS PAS d'en-tête (Nom, Prénom, Adresse, Date...) car ils sont ajoutés automatiquement.
           - NE METS PAS la ligne 'Objet : ...' car elle est déjà présente dans le template.
           - Commence DIRECTEMENT par la salutation 'Monsieur, Madame,'.
           - Utilise cette structure :
           {}
           - Rédige environ 350 à 400 mots pour que la lettre soit bien remplie mais TIENNE SUR UNE SEULE PAGE A4.
           - Sois dense, professionnel et enthousiaste.
           - Ne fais pas de remplissage inutile, chaque phrase doit apporter de la valeur.
           - Sépare BIEN les paragraphes par une double ligne vide (\n\n) pour qu'ils soient lisibles.
        ---
        5. Propose un titre pour mon CV (new_cv_title).
           IMPORTANT : Ce titre doit être court et générique (ex: 'Assistant Informatique', 'Développeur Web'). 
           DÉDUIS-le de l'offre mais simplifie-le. Si l'offre est 'Assistant Informatique - Alternance', mets JUSTE 'Assistant Informatique'.

        Offre d'emploi (Texte extrait) :
        ---
        {}
        ---

        Mon CV :
        ---
        {}
        ---

        Réponds UNIQUEMENT au format JSON strict suivant :
        {{\"company_name\": \"...\", \"job_title\": \"...\", \"company_summary\": \"...\", \"cover_letter\": \"...\", \"new_cv_title\": \"...\"}}",
        letter_structure, job_text, cv_content
    );

    let client = reqwest::Client::new();
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

    let mut result: AnalysisResult = serde_json::from_str(ai_text).map_err(|e| format!("Erreur de parsing AI Result: {} | Texte: {}", e, ai_text))?;
    
    // Add the original job text to the result
    result.offer_text = job_text;

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
        "<html>
            <body style='font-family: sans-serif; padding: 40px;'>
                <h1>Résumé : {}</h1>
                <h2>Entreprise</h2>
                <p>{}</p>
                
                <h2>Offre d'emploi (Extrait)</h2>
                <div style='background: #f4f4f4; padding: 15px; border-radius: 5px; white-space: pre-wrap; font-size: 0.9em; max-height: 800px; overflow: hidden;'>
                    {}
                </div>

                <h2>Source</h2>
                <p><a href='{}'>{}</a></p>
            </body>
        </html>",
        result.company_name, result.company_summary, result.offer_text, url, url
    );
    let summary_pdf = html_to_pdf(&summary_html)?;

    // 2. Letter PDF
    println!("DEBUG: Generating PDF with NEW template logic");
    let letter_html_path = PathBuf::from("..").join("public").join("lettre.html");
    let letter_css_path = PathBuf::from("..").join("public").join("lettre-style.css");
    
    let mut letter_html = fs::read_to_string(letter_html_path).unwrap_or_else(|_| "Error loading template".to_string());
    let letter_css = fs::read_to_string(letter_css_path).unwrap_or_else(|_| "".to_string());

    // Prepare Date
    let today = Local::now().format("%d/%m/%Y").to_string();

    // Prepare Photo Base64
    let photo_path = PathBuf::from("..").join("public").join("Profil_ai1.jpg");
    let photo_data = if let Ok(photo_bytes) = fs::read(photo_path) {
        format!("data:image/jpeg;base64,{}", general_purpose::STANDARD.encode(photo_bytes))
    } else {
        "".to_string()
    };

    // Format cover letter content with <p> tags
    let formatted_content = result.cover_letter
        .split("\n\n")
        .map(|p| format!("<p>{}</p>", p.trim()))
        .collect::<Vec<String>>()
        .join("\n");

    // Replacements
    letter_html = letter_html
        .replace("[MY_NAME]", "LOÏC CERQUEIRA")
        .replace("[MY_TITLE]", &result.new_cv_title.to_uppercase())
        .replace("[MY_PHONE]", "06 58 11 63 69")
        .replace("[MY_EMAIL]", "cerqueira.loic88@gmail.com")
        .replace("[MY_LOCATION]", "Toulouse")
        .replace("[MY_PHOTO_DATA]", &photo_data)
        .replace("[DATE]", &today)
        .replace("[COMPANY_NAME]", &result.company_name)
        .replace("[COMPANY_ADDRESS]", "") // AI doesn't provide address yet
        .replace("[JOB_TITLE]", &result.job_title)
        .replace("[LETTER_CONTENT]", &formatted_content);

    // Inject CSS
    letter_html = letter_html.replace("/* Placeholder for injected CSS */", &letter_css);

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
