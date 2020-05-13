google.charts.load('current', {
  'packages':['geochart'],
  // Note: you will need to get a mapsApiKey for your project.
  // See: https://developers.google.com/chart/interactive/docs/basic_load_libs#load-settings
  'mapsApiKey': 'AIzaSyB7nmLiSk9Kt0naPZUjuopjYZaH3QiWNHw'
});
google.charts.setOnLoadCallback(drawRegionsMap);

function drawRegionsMap(data_table) {
  var data = google.visualization.arrayToDataTable([
    ['Country', 'Popularity'],
    ['Germany', 200],
    ['United States', 300],
    ['Brazil', 400],
    ['Canada', 500],
    ['France', 600],
    ['RU', 700],
    ['France', 1000]
  ]);

  var options = {
    // region: 'IT',
    // displayMode: 'markers',
    colorAxis: {colors: ['white', '#1DB954']}
  };

  var chart = new google.visualization.GeoChart(document.getElementById('regions_div'));

  chart.draw(data, options);
}