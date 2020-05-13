window.onload = function () {
    const input = document.querySelector('input');
    // add routine for submit button
    document.querySelector('form').addEventListener('submit', function(event) {
      event.preventDefault();
      const searchQuery = '/search?query=' + (query => !query ? "cut to the feeling" : query)(input.value);
      
      fetch(searchQuery).then(e => e.json()).then(data => {
        document.getElementById('results').innerHTML = data.tracks.items
          .map(track => `<li class="text-salmon" onClick="getAnalysis(&apos;${ track.id }&apos;)">${track.name} - ${track.artists[0].name}</li>`)
          .join('\n');
      }).catch(error => {
        document.getElementById('results').innerHTML = error;
      });
    });
};