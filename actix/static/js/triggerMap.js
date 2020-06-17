document.addEventListener('DOMContentLoaded', () => {
    const input = document.querySelector('input');
    document.querySelector('form').addEventListener('submit', function(event) {
        // on submit, show spinner
        showSpinner();

        // send request
        event.preventDefault();

        $.ajax({
            type: 'get',
            url: '/run',
            cache: true,
            async: 'asynchronous',
            data : {
                playlist: encodeURIComponent(input.value) // properly encode spaces
            },
            dataType: 'json',
            success: function(response) {
                // update map data
                console.log(response)
                parseData(response);
            },
            error: function(request, status, error) {
                hideSpinner();
                console.log("Error: " + error)
                alert("Error " + request.status + ": " + error);
            }
        });

        
    });
});

function parseData(data) {
    // on success, hide spinner
    hideSpinner();

    // update map
    for (var key in data) {
        countries_array.forEach( function(pair) {
            if (key == pair[0]) {
                pair[1] = data[key];
            }
        });
    }

    // display side note
    if ('unknown' in data) {
        document.getElementById('sidenote').innerHTML = '\* Unable to find nationality information for ' + data['unknown'] + ' artists.';
    }
    else {
        document.getElementById('sidenote').innerHTML = '';
    }

    drawRegionsMap(data);
}
  
function showSpinner() {
    var spinner = document.getElementById('spinner');
    var form = document.getElementById('trigger');
    
    spinner.style.display = 'inline-block';
    form.style.display = 'none';
}

function hideSpinner() {
    var spinner = document.getElementById('spinner');
    var form = document.getElementById('trigger');
    spinner.style.display = 'none';
    form.style.display = 'initial';
}
