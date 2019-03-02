function refresh() {
    var textContent = document.getElementById('code-area').value;
    document.getElementById('eval-area').srcdoc = textContent;
}