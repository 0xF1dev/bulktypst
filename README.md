# Bulktypst

Bulktypst is a simple CLI that renders a Typst template in bulk from data in a CSV file written in Rust.

## Instructions
To use Bulktypst, prepare the Typst template you wish to use and add the dynamic content with:
```typst
#import sys: inputs

Example content: #inputs.name
```
where "name" is the column's header in the CSV.

Then, run:
```shell
bulktypst --template template.typ --csv data.csv --output ./out/
```

## Why?
This was mostly made for personal use, to start using Rust and as a project for the [Stardance](https://stardance.hackclub.com/) challenge
