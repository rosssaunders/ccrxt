#!/bin/bash

echo "🔧 Applying final Chrome extension fixes..."

# Ensure all extension files are copied
echo "📁 Copying extension files..."
cp manifest.json background.js dist/
cp -r icons dist/

# Create init.js for WASM loading
echo "⚡ Creating init.js..."
cat > dist/init.js << 'EOF'
import init, * as bindings from './ccrxt-web-ui.js';

async function initApp() {
    try {
        // Use the new single-parameter format to avoid deprecation warning
        const wasm = await init({
            module_or_path: './ccrxt-web-ui_bg.wasm'
        });
        window.wasmBindings = bindings;
        dispatchEvent(new CustomEvent("TrunkApplicationStarted", {detail: {wasm}}));
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
    }
}

initApp();
EOF

# Fix HTML file
echo "🧹 Cleaning HTML file..."
awk '
BEGIN { in_script = 0 }
/<script type="module">/ {
    print "    <script type=\"module\" src=\"./init.js\"></script>"
    in_script = 1
    next
}
in_script && /<\/script>/ {
    in_script = 0
    next
}
!in_script {
    line = $0
    # Remove integrity attributes and crossorigin
    gsub(/crossorigin="anonymous" integrity="[^"]*"/, "", line)
    gsub(/integrity="[^"]*"/, "", line)
    gsub(/crossorigin="anonymous" /, "", line)
    
    # Remove problematic preload links that cause warnings
    if (line ~ /<link rel="(modulepreload|preload)"/) {
        next
    }
    
    print line
}
' dist/index.html > dist/index.html.tmp && mv dist/index.html.tmp dist/index.html

# Verify all files exist
echo "✅ Verifying extension files..."
errors=0

if [[ ! -f "dist/manifest.json" ]]; then
    echo "❌ manifest.json missing"
    errors=$((errors + 1))
fi

if [[ ! -f "dist/background.js" ]]; then
    echo "❌ background.js missing"  
    errors=$((errors + 1))
fi

if [[ ! -d "dist/icons" ]]; then
    echo "❌ icons directory missing"
    errors=$((errors + 1))
fi

if [[ ! -f "dist/icons/icon16.png" ]]; then
    echo "❌ icon16.png missing"
    errors=$((errors + 1))
fi

if [[ ! -f "dist/init.js" ]]; then
    echo "❌ init.js missing"
    errors=$((errors + 1))
fi

if [[ ! -f "dist/index.html" ]]; then
    echo "❌ index.html missing"
    errors=$((errors + 1))
fi

# Check for inline scripts
if grep -q 'import init' dist/index.html; then
    echo "⚠️  WARNING: Inline script still present in HTML"
    errors=$((errors + 1))
fi

if [[ $errors -eq 0 ]]; then
    echo "🎉 Chrome extension ready!"
    echo "📂 Load from: $(pwd)/dist"
else
    echo "❌ $errors error(s) found. Extension may not load properly."
    exit 1
fi