window.onload = () => {
    var some_map;
    $.ajax({
        type: 'get',
        url: '/user_playlists',
        cache: true,
        async: 'asynchronous',
        dataType: 'json',
        success: function(response) {
            // update map data
            buildForm(response);
        },
        error: function(request, status, error) {
            console.log("Error: " + error);
            alert("Error! Please log in.");
            location = "/logout";
        }
    });
};

function buildForm(playlists) {
    var form = document.getElementById('select_form');

    // var sorted = Object.values(playlists).sort();

    var sorted = Object.entries(playlists).sort(function(a,b) {
        return a[1] < b[1]
    })

    var sorted = Object.entries(playlists).sort(function(a, b) {
        var a = a[1].toUpperCase(); // ignore upper and lowercase
        var b = b[1].toUpperCase(); // ignore upper and lowercase
        if (a < b) {
          return -1;
        }
        else if (a > b) {
          return 1;
        }
        // names must be equal
        return 0;
      })

    for (const pair of sorted) {
        var opt = document.createElement('option');
        opt.value = pair[0];
        opt.innerHTML = pair[1];
        form.appendChild(opt);
        // console.log(response[r])
    }
}