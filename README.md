# COVID-CLI - A COVID-19 CLI application made using Rust

![COVID-CLI](https://firebasestorage.googleapis.com/v0/b/roshen-nair.appspot.com/o/project-images%2Fcovid-cli.png?alt=media&token=4cdcbf83-81b4-4940-aac4-adc65b921d8c)

This is a small project I made the day after attending a Rust workshop hosted by APU's Student Developer Society. Essentially, it is a tool that allows users to get the latest data on COVID-19 cases, deaths & recoveries on a global and local (country-based) scale.

## Requirements:
- rustc
- cargo

## Installation

The fastest way to install covid-cli globally: 

```cargo install covid-cli```

## Commands:

- ```covid-cli global``` - Prints the global summary data to the console.
- ```covid-cli <country-slug>``` - Prints the local summary data for a specific country* to the console.

\* In order to get a full list of country slug codes for each country, visit https://api.covid19api.com/countries.
