document.addEventListener('DOMContentLoaded', () => {
    const input = document.getElementById('select_form');
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
                playlist_id: encodeURIComponent(input.value) // properly encode spaces
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
                alert("Oops! Something went wrong, please try again.");
            }
        });

        
    });
});

function parseData(data) {
    // on success, hide spinner
    hideSpinner();

    // reset array to zeroes
    for (const iso_pair of countries_array) {
        if (iso_pair[0] == 'Country') continue;
        else iso_pair[1] = 0;
    }

    // update map
    for (const key in data) {
        countries_array.forEach( function(pair) {
            if (key == pair[0]) {
                pair[1] = data[key];
                return;
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

    drawRegionsMap();
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
