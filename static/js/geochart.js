google.charts.load('current', {
  'packages':['geochart'],
  // Note: you will need to get a mapsApiKey for your project.
  // See: https://developers.google.com/chart/interactive/docs/basic_load_libs#load-settings
  'mapsApiKey': 'AIzaSyB7nmLiSk9Kt0naPZUjuopjYZaH3QiWNHw' /* REMEMBER TO RENEW */
});
google.charts.setOnLoadCallback(drawRegionsMap);

function drawRegionsMap() {
  var data = google.visualization.arrayToDataTable(countries_array);

  var options = {
    // backgroundColor: '#f5f5f5',
    // enableRegionInteractivity: false,
    // tooltip: { textStyle: { fontName: 'helvetica', fontSize: 12 } },
    backgroundColor:'transparent',
    colorAxis: {colors: ['white', '#1DB954']}
  };

  var chart = new google.visualization.GeoChart(document.getElementById('regions_div'));

  chart.draw(data, options);
}

var countries_array = [['Country', 'Number of Artists'],
['AF', 0],  // Afghanistan
['AX', 0],  // Åland Islands
['AL', 0],  // Albania
['DZ', 0],  // Algeria
['AS', 0],  // American Samoa
['AD', 0],  // Andorra
['AO', 0],  // Angola
['AI', 0],  // Anguilla
['AQ', 0],  // Antarctica
['AG', 0],  // Antigua and Barbuda
['AR', 0],  // Argentina
['AM', 0],  // Armenia
['AW', 0],  // Aruba
['AU', 0],  // Australia
['AT', 0],  // Austria
['AZ', 0],  // Azerbaijan
['BS', 0],  // Bahamas
['BH', 0],  // Bahrain
['BD', 0],  // Bangladesh
['BB', 0],  // Barbados
['BY', 0],  // Belarus
['BE', 0],  // Belgium
['BZ', 0],  // Belize
['BJ', 0],  // Benin
['BM', 0],  // Bermuda
['BT', 0],  // Bhutan
['BO', 0],  // Bolivia, Plurinational State of
['BQ', 0],  // Bonaire, Sint Eustatius and Saba
['BA', 0],  // Bosnia and Herzegovina
['BW', 0],  // Botswana
['BV', 0],  // Bouvet Island
['BR', 0],  // Brazil
['IO', 0],  // British Indian Ocean Territory
['BN', 0],  // Brunei Darussalam
['BG', 0],  // Bulgaria
['BF', 0],  // Burkina Faso
['BI', 0],  // Burundi
['KH', 0],  // Cambodia
['CM', 0],  // Cameroon
['CA', 0],  // Canada
['CV', 0],  // Cape Verde
['KY', 0],  // Cayman Islands
['CF', 0],  // Central African Republic
['TD', 0],  // Chad
['CL', 0],  // Chile
['CN', 0],  // China
['CX', 0],  // Christmas Island
['CC', 0],  // Cocos (Keeling) Islands
['CO', 0],  // Colombia
['KM', 0],  // Comoros
['CG', 0],  // Congo
['CD', 0],  // Congo, the Democratic Republic of the
['CK', 0],  // Cook Islands
['CR', 0],  // Costa Rica
['CI', 0],  // Côte d'Ivoire
['HR', 0],  // Croatia
['CU', 0],  // Cuba
['CW', 0],  // Curaçao
['CY', 0],  // Cyprus
['CZ', 0],  // Czech Republic
['DK', 0],  // Denmark
['DJ', 0],  // Djibouti
['DM', 0],  // Dominica
['DO', 0],  // Dominican Republic
['EC', 0],  // Ecuador
['EG', 0],  // Egypt
['SV', 0],  // El Salvador
['GQ', 0],  // Equatorial Guinea
['ER', 0],  // Eritrea
['EE', 0],  // Estonia
['ET', 0],  // Ethiopia
['FK', 0],  // Falkland Islands (Malvinas)
['FO', 0],  // Faroe Islands
['FJ', 0],  // Fiji
['FI', 0],  // Finland
['FR', 0],  // France
['GF', 0],  // French Guiana
['PF', 0],  // French Polynesia
['TF', 0],  // French Southern Territories
['GA', 0],  // Gabon
['GM', 0],  // Gambia
['GE', 0],  // Georgia
['DE', 0],  // Germany
['GH', 0],  // Ghana
['GI', 0],  // Gibraltar
['GR', 0],  // Greece
['GL', 0],  // Greenland
['GD', 0],  // Grenada
['GP', 0],  // Guadeloupe
['GU', 0],  // Guam
['GT', 0],  // Guatemala
['GG', 0],  // Guernsey
['GN', 0],  // Guinea
['GW', 0],  // Guinea-Bissau
['GY', 0],  // Guyana
['HT', 0],  // Haiti
['HM', 0],  // Heard Island and McDonald Islands
['VA', 0],  // Holy See (Vatican City State)
['HN', 0],  // Honduras
['HK', 0],  // Hong Kong
['HU', 0],  // Hungary
['IS', 0],  // Iceland
['IN', 0],  // India
['ID', 0],  // Indonesia
['IR', 0],  // Iran, Islamic Republic of
['IQ', 0],  // Iraq
['IE', 0],  // Ireland
['IM', 0],  // Isle of Man
['IL', 0],  // Israel
['IT', 0],  // Italy
['JM', 0],  // Jamaica
['JP', 0],  // Japan
['JE', 0],  // Jersey
['JO', 0],  // Jordan
['KZ', 0],  // Kazakhstan
['KE', 0],  // Kenya
['KI', 0],  // Kiribati
['KP', 0],  // Korea, Democratic People's Republic of
['KR', 0],  // Korea, Republic of
['KW', 0],  // Kuwait
['KG', 0],  // Kyrgyzstan
['LA', 0],  // Lao People's Democratic Republic
['LV', 0],  // Latvia
['LB', 0],  // Lebanon
['LS', 0],  // Lesotho
['LR', 0],  // Liberia
['LY', 0],  // Libya
['LI', 0],  // Liechtenstein
['LT', 0],  // Lithuania
['LU', 0],  // Luxembourg
['MO', 0],  // Macao
['MK', 0],  // Macedonia, the Former Yugoslav Republic of
['MG', 0],  // Madagascar
['MW', 0],  // Malawi
['MY', 0],  // Malaysia
['MV', 0],  // Maldives
['ML', 0],  // Mali
['MT', 0],  // Malta
['MH', 0],  // Marshall Islands
['MQ', 0],  // Martinique
['MR', 0],  // Mauritania
['MU', 0],  // Mauritius
['YT', 0],  // Mayotte
['MX', 0],  // Mexico
['FM', 0],  // Micronesia, Federated States of
['MD', 0],  // Moldova, Republic of
['MC', 0],  // Monaco
['MN', 0],  // Mongolia
['ME', 0],  // Montenegro
['MS', 0],  // Montserrat
['MA', 0],  // Morocco
['MZ', 0],  // Mozambique
['MM', 0],  // Myanmar
['NA', 0],  // Namibia
['NR', 0],  // Nauru
['NP', 0],  // Nepal
['NL', 0],  // Netherlands
['NC', 0],  // New Caledonia
['NZ', 0],  // New Zealand
['NI', 0],  // Nicaragua
['NE', 0],  // Niger
['NG', 0],  // Nigeria
['NU', 0],  // Niue
['NF', 0],  // Norfolk Island
['MP', 0],  // Northern Mariana Islands
['NO', 0],  // Norway
['OM', 0],  // Oman
['PK', 0],  // Pakistan
['PW', 0],  // Palau
['PS', 0],  // Palestine, State of
['PA', 0],  // Panama
['PG', 0],  // Papua New Guinea
['PY', 0],  // Paraguay
['PE', 0],  // Peru
['PH', 0],  // Philippines
['PN', 0],  // Pitcairn
['PL', 0],  // Poland
['PT', 0],  // Portugal
['PR', 0],  // Puerto Rico
['QA', 0],  // Qatar
['RE', 0],  // Réunion
['RO', 0],  // Romania
['RU', 0],  // Russian Federation
['RW', 0],  // Rwanda
['BL', 0],  // Saint Barthélemy
['SH', 0],  // Saint Helena, Ascension and Tristan da Cunha
['KN', 0],  // Saint Kitts and Nevis
['LC', 0],  // Saint Lucia
['MF', 0],  // Saint Martin (French part)
['PM', 0],  // Saint Pierre and Miquelon
['VC', 0],  // Saint Vincent and the Grenadines
['WS', 0],  // Samoa
['SM', 0],  // San Marino
['ST', 0],  // Sao Tome and Principe
['SA', 0],  // Saudi Arabia
['SN', 0],  // Senegal
['RS', 0],  // Serbia
['SC', 0],  // Seychelles
['SL', 0],  // Sierra Leone
['SG', 0],  // Singapore
['SX', 0],  // Sint Maarten (Dutch part)
['SK', 0],  // Slovakia
['SI', 0],  // Slovenia
['SB', 0],  // Solomon Islands
['SO', 0],  // Somalia
['ZA', 0],  // South Africa
['GS', 0],  // South Georgia and the South Sandwich Islands
['SS', 0],  // South Sudan
['ES', 0],  // Spain
['LK', 0],  // Sri Lanka
['SD', 0],  // Sudan
['SR', 0],  // Suriname
['SJ', 0],  // Svalbard and Jan Mayen
['SZ', 0],  // Swaziland
['SE', 0],  // Sweden
['CH', 0],  // Switzerland
['SY', 0],  // Syrian Arab Republic
['TW', 0],  // Taiwan, Province of China
['TJ', 0],  // Tajikistan
['TZ', 0],  // Tanzania, United Republic of
['TH', 0],  // Thailand
['TL', 0],  // Timor-Leste
['TG', 0],  // Togo
['TK', 0],  // Tokelau
['TO', 0],  // Tonga
['TT', 0],  // Trinidad and Tobago
['TN', 0],  // Tunisia
['TR', 0],  // Turkey
['TM', 0],  // Turkmenistan
['TC', 0],  // Turks and Caicos Islands
['TV', 0],  // Tuvalu
['UG', 0],  // Uganda
['UA', 0],  // Ukraine
['AE', 0],  // United Arab Emirates
['GB', 0],  // United Kingdom
['US', 0],  // United States
['UM', 0],  // United States Minor Outlying Islands
['UY', 0],  // Uruguay
['UZ', 0],  // Uzbekistan
['VU', 0],  // Vanuatu
['VE', 0],  // Venezuela, Bolivarian Republic of
['VN', 0],  // Viet Nam
['VG', 0],  // Virgin Islands, British
['VI', 0],  // Virgin Islands, U.S.
['WF', 0],  // Wallis and Futuna
['EH', 0],  // Western Sahara
['YE', 0],  // Yemen
['ZM', 0],  // Zambia
['ZW', 0]  // Zimbabwe
];