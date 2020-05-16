document.addEventListener('DOMContentLoaded', () => {
    const input = document.querySelector('input');
    document.querySelector('form').addEventListener('submit', function(event) {
        event.preventDefault();

        $.ajax({
            type: 'get',
            url: '/test',
            cache: false,
            async: 'asynchronous',
            data : encodeURIComponent(input.value), // properly encode spaces
            dataType: 'json',
            success: function(response) {
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

function parseData(data) {
    alert(JSON.stringify(data));
}
  
