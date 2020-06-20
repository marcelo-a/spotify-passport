document.addEventListener('DOMContentLoaded', () => {
    const input = document.querySelector('input');
    
    document.querySelector('form').addEventListener('submit', function user_login(e){
        e.preventDefault();
        
        $.ajax({
            type: 'GET',
            url: '/login',
            cache: false,
            async: 'asynchronous',
            success: function(response) {
                console.log(response);
                // redirect to main page
                location = 'main';
            },
            error: function(request, status, error) {
                console.log("Error: " + error);
                alert("Error: " + error);
            }
        });

    });
});