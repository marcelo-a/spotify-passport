$('#trigger').onclick = () => {
    $.ajax({
        type: 'get',
        url: '../main.py',
        cache: false,
        async: 'asynchronous',
        dataType: 'json',
        success: function(data) {
          console.log(JSON.stringify(data))
        },
        error: function(request, status, error) {
          console.log("Error: " + error)
        }
    });
}