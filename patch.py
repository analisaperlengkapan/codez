import sys

with open('tests/stars_watches.spec.ts', 'r') as f:
    code = f.read()

# Replace button:has-text(...) with page.getByRole matching
code = code.replace("actionsDiv.getByRole('button', { name: /^Star \\(/ }).first()", "actionsDiv.locator('button', { hasText: /^Star \\(/ }).first()")
code = code.replace("actionsDiv.getByRole('button', { name: /^Unstar \\(/ }).first()", "actionsDiv.locator('button', { hasText: /^Unstar \\(/ }).first()")
code = code.replace("actionsDiv.getByRole('button', { name: /^Watch \\(/ }).first()", "actionsDiv.locator('button', { hasText: /^Watch \\(/ }).first()")
code = code.replace("actionsDiv.getByRole('button', { name: /^Unwatch \\(/ }).first()", "actionsDiv.locator('button', { hasText: /^Unwatch \\(/ }).first()")

with open('tests/stars_watches.spec.ts', 'w') as f:
    f.write(code)
