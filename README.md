# Tauri Calc

This is a spreadsheet demo application built using Tauri and SvelteKit. Right now, the following features are supported:

- the application starts with a spreadsheet containing three rows and four columns
- cells can contain numbers, text (in double quotes) and formulas (`=<expression>`)
  - supported expressions are numbers, text, cell references and function calls, with any number of parameter expressions
  - the only available function is `sum(...)`, which requires that all parameters evaluate to numbers
- changing a cell's value recalculates other cells as necessary, using [topological ordering](https://en.wikipedia.org/wiki/Topological_sorting)
  - circular dependencies are not properly handles and crach the application
