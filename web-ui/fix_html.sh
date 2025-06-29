#!/bin/bash

echo "Fixing HTML file for Chrome extension compatibility..."

# Create init.js file
cat > dist/init.js << 'EOF'
import init, * as bindings from './ccrxt-web-ui.js';

async function initApp() {
    try {
        const wasm = await init('./ccrxt-web-ui_bg.wasm');
        window.wasmBindings = bindings;
        dispatchEvent(new CustomEvent("TrunkApplicationStarted", {detail: {wasm}}));
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
    }
}

initApp();
EOF

# Create a clean HTML file by removing inline scripts and integrity attributes
awk '
BEGIN { in_script = 0 }
/<script type="module">/ {
    # Replace inline script with external script reference
    print "    <script type=\"module\" src=\"./init.js\"></script>"
    in_script = 1
    next
}
in_script && /<\/script>/ {
    # End of script block, stop skipping
    in_script = 0
    next
}
!in_script {
    # Not in script block, process the line
    line = $0
    # Remove integrity attributes
    gsub(/crossorigin="anonymous" integrity="[^"]*"/, "", line)
    gsub(/integrity="[^"]*"/, "", line)
    gsub(/crossorigin="anonymous" /, "", line)
    print line
}
' dist/index.html > dist/index.html.tmp && mv dist/index.html.tmp dist/index.html

echo "HTML file fixed successfully"