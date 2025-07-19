---
mode: agent
tools: ["codebase", "editFiles", "fetch"]
---

For the given endpoint file, update the code so it matches all relevant project instructions exactly (including coding, documentation, error handling, and example requirements).

If the URL to the official API documentation for the endpoint is missing or unknown, prompt the user to provide it before making any changes. Once provided, automatically load the URL and verify that it is correct and accessible.

Success criteria:

- All code and documentation strictly conform to the instructions files for the endpoint.
- The official API documentation URL is present, correct, and verified.
- If the URL is missing, the user is prompted and the process pauses until it is supplied.
- The file has unit tests that cover all relevant functionality.
- The code is well-structured, follows best practices, and is free of linting errors.
