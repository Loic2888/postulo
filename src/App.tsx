import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface AnalysisResult {
  company_name: String;
  company_summary: String;
  cover_letter: String;
  new_cv_title: String;
}

interface PDFBytes {
  summary: string;
  letter: string;
  cv: string;
}

function App() {
  const [url, setUrl] = useState("");
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<AnalysisResult | null>(null);
  const [pdfs, setPdfs] = useState<PDFBytes | null>(null);

  const handleProcess = async () => {
    if (!url) {
      setStatus("Veuillez entrer le lien de l'offre.");
      return;
    }

    setLoading(true);
    setStatus("Analyse de l'offre par Groq...");
    setResults(null);
    setPdfs(null);

    try {
      // API Key is now handled in the Rust backend via .env
      const analysis: AnalysisResult = await invoke("process_offer", { url });
      setResults(analysis);
      setStatus("Génération des PDFs (cela peut prendre quelques secondes)...");

      const pdfBytes: PDFBytes = await invoke("generate_pdfs", { result: analysis, url });
      setPdfs(pdfBytes);
      setStatus("Documents prêts !");
    } catch (err) {
      console.error(err);
      setStatus(`Erreur: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const downloadPdf = (base64: string, filename: string) => {
    try {
      const byteCharacters = atob(base64);
      const byteNumbers = new Array(byteCharacters.length);
      for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
      }
      const byteArray = new Uint8Array(byteNumbers);
      const blob = new Blob([byteArray], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);

      const link = document.createElement("a");
      link.href = url;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      // Cleanup
      setTimeout(() => URL.revokeObjectURL(url), 100);

      setStatus("C'est dans la poche flemard ! 🚀");
    } catch (e) {
      console.error("Download failed", e);
      setStatus("Erreur lors du téléchargement du PDF.");
    }
  };

  return (
    <div className="container">
      <header>
        <h1>POSTULO</h1>
        <p className="subtitle">Assistant de candidature IA</p>
      </header>

      <div className="card glass main-input">
        <input
          type="url"
          placeholder="Collez le lien de l'offre ici (LinkedIn, Indeed...)"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          disabled={loading}
          autoFocus
        />
        <button onClick={handleProcess} disabled={loading || !url}>
          {loading ? "Chargement..." : "Générer"}
        </button>
      </div>

      {status && <div className={`status ${loading ? "pulse" : ""}`}>{status}</div>}

      {results && pdfs && (
        <div className="results-grid fade-in">
          <div className="doc-card glass" onClick={() => downloadPdf(pdfs.summary, `Resume_${results.company_name}.pdf`)}>
            <div className="icon">📋</div>
            <h3>Résumé + Offre</h3>
            <button className="dl-btn">Télécharger</button>
          </div>

          <div className="doc-card glass" onClick={() => downloadPdf(pdfs.letter, `Lettre_${results.company_name}.pdf`)}>
            <div className="icon">✉️</div>
            <h3>Lettre</h3>
            <button className="dl-btn">Télécharger</button>
          </div>

          <div className="doc-card glass" onClick={() => downloadPdf(pdfs.cv, `CV_${results.company_name}.pdf`)}>
            <div className="icon">📄</div>
            <h3>CV Adapté</h3>
            <button className="dl-btn">Télécharger</button>
          </div>
        </div>
      )}

      {!results && !loading && (
        <footer className="footer fade-in">
          <p>Préparez vos documents en un clic.</p>
        </footer>
      )}
    </div>
  );
}

export default App;
