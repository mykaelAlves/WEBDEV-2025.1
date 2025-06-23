async function gerarTabela() {
    const url = "http://127.0.0.1:30000/"

    try {
        const response = await fetch(url+"api/alunos");

        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        document.getElementById("tab").innerHTML = await response.text();
    } catch (error) {
        console.error(error.message);
    }
}