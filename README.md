# Postulo - Intelligent Job Application Assistant 🚀

![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Groq](https://img.shields.io/badge/Groq-f3f4f6?style=for-the-badge&logo=lightning&logoColor=black)

**Postulo** is a native desktop application designed to automate and optimize your job application process. By combining **Rust** performance with AI through **Groq (Llama 3.3 70B)**, it instantly generates tailored cover letters, company summaries, and adapts your CV to any job offer.

---

## 📸 Overview

- **URL Analysis**: Paste a link (LinkedIn, Indeed, etc.) and let the AI extract key information.
- **Privacy First**: Your data and API keys stay on your local machine. No tracking, no cloud storage.
- **Fast & Reliable**: Native execution through Tauri/Rust for a smooth desktop experience.

---

## 📄 Automated Document Generation

Postulo generates three high-quality PDF documents for every application based on **customizable HTML/CSS templates**:

1.  **📊 Company & Job Summary**:
    - A concise overview of the company culture and the job requirements.
2.  **✉️ Tailored Cover Letter**:
    - A professional letter generated from a **flexible HTML template** (`public/lettre.html`).
    - Styled with **CSS** (`public/lettre-style.css`) to match your personal branding.
    - AI-generated content is automatically injected into the template while respecting a professional single-page layout.
3.  **📄 Adapted CV**:
    - Your original **HTML CV** (`index.html`) automatically updated with a job-optimized title.
    - Uses the root **CSS stylesheet** (`style.css`) for consistent rendering.

---

## 🛠️ Tech Stack

| Component | Technology |
| --- | --- |
| **Native Framework** | [Tauri v2](https://tauri.app/) |
| **Logic Backend** | [Rust](https://www.rust-lang.org/) |
| **Frontend UI** | [React](https://reactjs.org/) + [TypeScript](https://www.typescriptlang.org/) |
| **Intelligence** | [Groq Cloud API](https://groq.com/) (Llama 3.3 70B) |
| **PDF Engine** | Headless Chrome (via `headless_chrome` Rust crate) |

---

## 🚀 Installation & Setup

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Node.js & npm](https://nodejs.org/)
- Google Chrome or Microsoft Edge (installed on your system for PDF rendering)

### Setup Steps

1.  **Clone the Repository**
    ```bash
    git clone https://github.com/YourUsername/postulo.git
    cd postulo
    ```

2.  **Install Dependencies**
    ```bash
    npm install
    ```

3.  **Environment Configuration**
    Navigate to the Tauri backend folder and create a `.env` file:
    ```bash
    cd src-tauri
    touch .env
    ```
    Add your Groq API key:
    ```env
    GROQ_API_KEY=your_groq_api_key_here
    ```

4.  **Template Configuration (HTML/CSS)**
    Postulo uses its parent's root HTML and CSS as the master templates to generate your documents:
    - `index.html` & `style.css`: Your main CV template and styles (at the project root).
    - `public/lettre.html` & `public/lettre-style.css`: Your cover letter template and styles.
    - `public/Profil_ai1.jpg`: Your profile picture (automatically embedded in both).

---

## 💻 Usage

To launch the app in development mode:

```bash
npm run tauri dev
```

Once opened:
1. Paste the job offer URL.
2. Click **Generate**.
3. Download your optimized documents: **"C'est dans la poche flemard!"** 🚀

---

## 📁 Project Structure

```text
.
├── index.html          # CV Master Template (Root)
├── style.css           # CV Master Styling (Root)
├── postulo/            # Native Application
│   ├── src-tauri/      # Backend (Rust Logic)
│   ├── public/         # Letter Templates & Assets
│   │   ├── lettre.html
│   │   └── lettre-style.css
│   ├── src/            # Frontend (React)
│   └── package.json
└── README.md
```

---

## 📜 Acknowledgments

Developed as part of an intensive software engineering program at **Holberton School Toulouse**.

---

## ✍️ Author

- **Loïc Cerqueira** - [@Loic2888](https://github.com/Loic2888)
