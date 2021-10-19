const backend_ip = "http://166.70.232.119:443"
// const backend_ip = "https://ssl-proxy.my-addr.org/myaddrproxy.php/http/166.70.232.119:443" // Certbot didn't work, and I have limited time

var getJSON = function(url, callback) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', "https://www.apple.com/", true);

    xhr.withCredentials = true;
    xhr.setRequestHeader("Access-Control-Allow-Credentials", "true");
    xhr.setRequestHeader("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
    xhr.setRequestHeader('Access-Control-Allow-Methods', 'POST, GET, HEAD');
    xhr.setRequestHeader("Access-Control-Allow-Origin", "*");
    xhr.responseType = 'text/plain';

    xhr.onload = function() {
      var status = xhr.status;
      if (status === 200) {
        callback(null, xhr.response);
      } else {
        callback(status, xhr.response);
      }
    };
    xhr.send();
};


function check_textarea() {
    let input = document.getElementById("post-inp").value;

    document.getElementById("accuracy-rating").innerHTML = "...";

    console.log("Querying server about:\n" + input);

    let api_call = backend_ip + "/query?q=" + encodeURI(input);
    const fetch_headers = new Headers({
        'Origin': backend_ip,
        'Access-Control-Allow-Origin': backend_ip,
    });

    // getJSON(api_call, function(err, data) {
    /* $.getJSON("https://www.apple.com/", function(data) {
        console.log("Got:\n" + data);

        document.getElementById("accuracy-rating").innerHTML = data['p'] + "%";
    }); */

    $.ajax({
    url: api_call,
    type: 'GET',
    // This is the important part
    xhrFields: {
        withCredentials: true
    },
    // This is the important part
    // data: data,
    success: function (response) {
        // handle the response
        let data = JSON.parse(response);
        console.log("Got:\n" + data);
        document.getElementById("accuracy-rating").innterHTML = data['p'] + '%';
    },
    error: function (xhr, status) {
        // handle errors
    }
    });
}

