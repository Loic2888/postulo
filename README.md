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

Postulo generates three high-quality PDF documents for every application:

1.  **📊 Company & Job Summary**:
    - A concise 2-3 sentence overview of the company culture and the job requirements.
    - Helps you prepare for interviews and understand the employer's priorities.
2.  **✉️ Tailored Cover Letter**:
    - A professional letter structured according to industry standards (You/Me/Us).
    - Fully personalized to match the specific job description while highlighting your relevant experience.
    - Includes proper headers with current date and contact info.
3.  **📄 Adapted CV**:
    - Your original CV (`index.html`) automatically updated with a **new title** specifically optimized for the job offer you're applying for.
    - Ensures your profile catches the eye of Recruiters/ATS (Applicant Tracking Systems) immediately.

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

4.  **CV Template Configuration**
    Postulo uses your original CV files at the root of the project to generate the new ones. Ensure you have:
    - `cv-template.html`: Your HTML CV template.
    - `cv-template.css`: The associated stylesheet.
    - `cv-photo.jpg`: Your profile picture.

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
├── src/                # Frontend source (React + TS)
├── src-tauri/        # Backend source (Rust)
│   ├── src/main.rs     # Rust entry point
│   ├── src/lib.rs      # Command logic (Scraping, IA, PDF)
│   └── .env            # Private variables (IGNORED)
├── index.html          # Source CV template
├── style.css           # Source CV style
└── package.json        # Node.js dependencies
```

---

## 📜 Acknowledgments

Developed as part of an intensive software engineering program at **Holberton School Toulouse**.

---

## ✍️ Author

- **Loïc Cerqueira** - [@Loic2888](https://github.com/Loic2888)
