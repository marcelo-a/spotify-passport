document.addEventListener('DOMContentLoaded', () => {
    const input = document.querySelector('input');
    document.querySelector('form').addEventListener('submit', function user_login(){
        const input = document.querySelector('input');

        $.ajax({
            type: 'GET',
            url: '/login',
            cache: false,
            async: 'asynchronous',
            data : {
                username: encodeURIComponent(input.value) // properly encode spaces
            },
            dataType: 'json',
            success: function(response) {
                console.log(response);
                // window.location.replace("callback");
                // handle_success();
            },
            error: function(request, status, error) {
                // console.log("Error: " + error);
                alert("Error: " + error);
            }
        });
    });
});

function handle_success() {
    window.location = 'main';
}