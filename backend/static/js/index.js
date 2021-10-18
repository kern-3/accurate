document.querySelector(".submitButton").addEventListener("click", () => {
    fetch("/predict?text=" + document.querySelector("#post-inp").value).then(res => res.json()).then(body => {
        document.querySelector(".percentage").textContent = parseInt(body*1000)/10 + "%";
    })
})