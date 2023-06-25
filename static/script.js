
function init() {
    const input = document.getElementById("answer");
    input.addEventListener("input", updateValue);

    fetch('/', {
        method: 'GET',})
        .then(response => console.log(response));
}

function updateValue(e) {
    const data = {
        "contents": e.target.value,
    };

// Send a post request
    fetch("/message", {
        method: "POST",
        body: JSON.stringify(data),
        }).then((response) => {

    });



}

window.onload = init;

