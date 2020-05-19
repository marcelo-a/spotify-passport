$('#login').addEventListener('click', () => {
    const input = document.querySelector('input');
    document.querySelector('form').addEventListener('submit', function(event) {
        event.preventDefault();

        $.ajax({
            type: 'get',
            url: '/run',
            cache: false,
            async: 'asynchronous',
            data : {
                username: encodeURIComponent(input.value), // properly encode spaces
                playlist: 'BONEYARD'
            },
            dataType: 'json',
            success: function(response) {
                console.log(response);
                parseData(response);
            },
            // success: parseData(data) {
            //     console.log(JSON.stringify(data))
            // },
            error: function(request, status, error) {
                console.log("Error: " + error)
            }
        });
    });
});