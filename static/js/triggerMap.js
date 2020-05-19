document.addEventListener('DOMContentLoaded', () => {
    const input = document.querySelector('input');
    document.querySelector('form').addEventListener('submit', function(event) {
        // on submit, show spinner
        var spinner = document.getElementById('spinner');
        spinner.style.display = 'block';
        // hide form
        var form = document.getElementById('form');
        form.style.display = 'none';

        // send request
        event.preventDefault();

        $.ajax({
            type: 'get',
            url: '/run',
            cache: true,
            async: 'asynchronous',
            data : {
                playlist: encodeURIComponent(input.value) // properly encode spaces
                // playlist: 'BONEYARD'
            },
            dataType: 'json',
            success: function(response) {
                // update map data
                parseData(response);
            },
            error: function(request, status, error) {
                // console.log("Error: " + error)
                alert("Error " + request.status + ": " + error);
            }
        });

        
    });
});

function parseData(data) {
    // on success, hide spinner
    var spinner = document.getElementById('spinner');
    var form = document.getElementById('form');
    spinner.style.display = 'none';
    form.style.display = 'block';

    for (var key in data) {
        countries_array.forEach( function(pair) {
            if (key == pair[0]) {
                pair[1] = data[key];
            }
        });
    }
    drawRegionsMap(data);
}
  
