const backend_ip = "166.70.232.119:99"

function check_textarea() {
    let input = document.getElementById("post-inp").value;

    document.getElementById("accuracy-rating").innerHTML = "...";

    console.log("Querying server about:\n" + input);

    let response = fetch(backend_ip + "/query?q=" + encodeURI(input));

    let json = JSON.parse(response);

    document.getElementById("accuracy-rating").innerHTML = json['p'] + "%";
}

