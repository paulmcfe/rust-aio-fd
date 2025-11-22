import init, { analyze_password } from "./pkg/password_analyzer.js";

const passwordInput = document.getElementById("pw");
const strengthLabel = document.getElementById("strength-label");
const reportPre = document.getElementById("report");

function chooseColor(score) {
    if (score < 30) return "red";
    if (score < 50) return "orangered";
    if (score < 70) return "orange";
    if (score < 90) return "green";
    return "darkgreen";
}

init().then(() => {
    passwordInput.addEventListener("input", () => {
        const pw = passwordInput.value;

        if (pw.length === 0) {
            strengthLabel.textContent = "Type a password to see its score.";
            strengthLabel.style.color = "inherit";
            reportPre.textContent = "";
            return;
        }

        const report = analyze_password(pw);

        // Extract the "Score: NN/100 (...)" line for a quick label.
        const scoreLine = report.split("\n").find(line => line.startsWith("Score:")) || "";
        const scoreMatch = scoreLine.match(/Score:\s+(\d+)/);
        const score = scoreMatch ? parseInt(scoreMatch[1], 10) : 0;

        strengthLabel.textContent = scoreLine || "Score unavailable";
        strengthLabel.style.color = chooseColor(score);
        reportPre.textContent = report;
    });
}).catch(err => {
    console.error("Error initializing WebAssembly module:", err);
    strengthLabel.textContent = "Error loading password checker.";
});
